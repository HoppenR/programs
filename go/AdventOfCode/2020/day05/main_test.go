package main

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestPart1(t *testing.T) {
	assert.Equal(t, 357, DecodePass("FBFBBFFRLR"))
	examplePasses := []string{"BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"}
	assert.Equal(t, 820, GetHighestPassID(examplePasses))
}

func TestPart2(t *testing.T) {
	smolPlanePasses := []string{"FFFFFFFRRR", "FFFFFFBLLL", "FFFFFFBLLR",
		"FFFFFFBLRL", "FFFFFFBLRR", "FFFFFFBRLL", "FFFFFFBRLR", "FFFFFFBRRL",
		"FFFFFFBRRR", "FFFFFBFLLL", "FFFFFBFLLR", "FFFFFBFLRR", "FFFFFBFRLL",
		"FFFFFBFRLR", "FFFFFBFRRL", "FFFFFBFRRR", "FFFFFBBLLL", "FFFFFBBLLR"}
	ans, err := FindMissingID(smolPlanePasses)
	assert.Nil(t, err)
	assert.Equal(t, 18, ans)
}
