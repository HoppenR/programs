package main

import (
	"flag"
	"log"
	"os"
	"sort"
)

// go build -ldflags "-X main.userID=XXX -X main.clientID=XXX -X main.clientSecret=XXX"
var (
	userID   string
	clientID string
	// clientSecret string
	tmptoken string
)

// ✅ searching/filtering,
// ❌ ordering
// ❌ command line arguments,
// ❌ figure out live alert library meme,
// ❌ and background checking

// Figure out how to create more tview windows, display extra info on the
// hovered window
func main() {
	//TODO: validate the token and refresh as needed
	background := flag.Bool("b", false,
		"Specify whether to check for streams in the background instead of interactively")
	flag.Parse()

	if *background {
		log.Fatalln("Unimplemented background mode")
		for true {
			// get streams and save them every n interval
			// compare to previous streams, notify-send changes
		}
	}

	if !*background {
		var err error
		log.SetFlags(log.Lshortfile)
		follows, err := getAllFollows(tmptoken, clientID, userID)
		if err != nil {
			log.Fatalln(err.Error())
		}
		channels, err := initializeChannels(tmptoken, clientID, follows)
		if err != nil {
			log.Fatalln(err.Error())
		}
		sort.Sort(sort.Reverse(channels))
		err = initUI(channels)
		if err != nil {
			log.Println(err.Error())
			os.Exit(2)
		}
	}
}
