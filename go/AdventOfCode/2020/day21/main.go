package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"sort"
	"strings"

	"github.com/scylladb/go-set/strset"
)

// ATI = Allergen -> ingredients.
// Ingredients is a slice of each line (as a set) where the allergen appears
// multiple allergens can link to an identical set of ingredients
// ati[dairy] -> [Set("kfcds", "nhms"), Set("trh", "fvjkl")]
// ati[fish]  -> [Set("kfcds", "nhms"), Set("sbzzf")]
type ATI map[string][]*strset.Set

type Book []Ingredient

type Ingredient struct {
	ingName  string
	allergen string
}

var pattern = regexp.MustCompile(`(.+) \(contains (.+)\)`)

func main() {
	// Ati links to same set of ingredients several times,
	// but ingredients only contain one single instance of each set
	ati, ingredients, err := ReadFoods("input")
	if err != nil {
		log.Fatalln(err)
	}
	fmt.Println("1:", NumSafe(ati, ingredients))
	fmt.Println("2:", DangerousIngredients(ati))
}

func NumSafe(ati ATI, ingredients []*strset.Set) (cnt int) {
	safe := SafeIngredients(ati)
	for _, ing := range ingredients {
		cnt += strset.Intersection(ing, safe).Size()
	}
	return cnt
}

func SafeIngredients(ati ATI) *strset.Set {
	inter := strset.New()
	union := strset.New()
	for _, v := range ati {
		inter.Merge(strset.Intersection(v...))
		union.Merge(strset.Union(v...))
	}
	return strset.Difference(union, inter)
}

func DangerousIngredients(ati ATI) string {
	// allergen -> Set(candidate1, candidate2...)
	dangerous := make(map[string]*strset.Set, 0)
	for k, v := range ati {
		dangerous[k] = strset.Intersection(v...)
	}
	assigned := make(Book, 0)
	// Resolve all multiple-candidates
	for len(dangerous) > 0 {
		for k, v := range dangerous {
			if v.Size() == 1 {
				assigned = append(assigned, Ingredient{v.List()[0], k})
				delete(dangerous, k)
				for i := range dangerous {
					dangerous[i].Separate(v)
				}
			}
		}
	}
	// Sort by allergen
	sort.Sort(assigned)
	var items []string
	for _, v := range assigned {
		items = append(items, v.ingName)
	}
	return strings.Join(items, ",")
}

func (b Book) Len() int {
	return len(b)
}

func (b Book) Less(i, j int) bool {
	return b[i].allergen < b[j].allergen
}

func (b Book) Swap(i, j int) {
	b[i], b[j] = b[j], b[i]
}

func ReadFoods(filename string) (ATI, []*strset.Set, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, nil, err
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	ati := make(ATI, 0)
	var ingredients []*strset.Set
	for scanner.Scan() {
		groups := pattern.FindAllStringSubmatch(scanner.Text(), -1)
		lineSet := strset.New(strings.Fields(groups[0][1])...)
		for _, aller := range strings.Split(groups[0][2], ", ") {
			ati[aller] = append(ati[aller], lineSet)
		}
		ingredients = append(ingredients, lineSet)
	}
	if err := scanner.Err(); err != nil {
		return nil, nil, err
	}
	return ati, ingredients, nil
}
