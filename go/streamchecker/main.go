package main

import (
	"log"
	"os"
	"sort"
)

// go build -ldflags "-X main.userID=XXX -X main.clientID=XXX -X main.clientSecret=XXX"
var (
	userID       string
	clientID     string
	clientSecret string
	tmptoken     string
)

func main() {
	// TODO: validate the token and refresh as needed
	var err error
	log.SetFlags(log.Lshortfile)
	follows, err := get_all_follows(tmptoken, clientID, userID)
	if err != nil {
		log.Fatalln(err.Error())
	}
	channels, err := initialize_channels(tmptoken, clientID, follows)
	if err != nil {
		log.Fatalln(err.Error())
	}
	sort.Sort(sort.Reverse(channels))
	games, err := get_game_info(tmptoken, clientID, channels)
	if err != nil {
		log.Fatalln(err.Error())
	}
	err = print_menu(games, channels)
	if err != nil {
		log.Println(err.Error())
		os.Exit(2)
	}
}
