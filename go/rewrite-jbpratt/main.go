package main

import (
	"encoding/json"
	"io"
	"io/ioutil"
	"net/http"
	"os"
)

type AssetNames struct {
	Names []string `json:"default"`
}

type GuildEmotes []struct {
	Name string `json:"name"`
	ID   int    `json:"id"`
}

var (
	TOKEN    string
	GUILD_ID string
)

func getGuildEmotes(tempFile *os.File) {
	req, err := http.NewRequest("GET", "https://discordapp.com/api/guilds/"+GUILD_ID+"/emojis", nil)
	if err != nil {
		panic(err)
	}
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		panic(err)
	}
	defer resp.Body.Close()
	if (resp.StatusCode / 10) == 40 {
		panic("failed to request discord emote manifest")
	}
	written, err := io.Copy(tempFile, resp.Body)
	if err != nil {
		panic(err)
	}
	if written == 0 {
		panic("zero bytes written to file")
	}
}

func compareEmotes(tempFile, emotesFile *os.File) {
	dec := json.NewDecoder(emotesFile)
	assetNames := new(AssetNames)
	err := dec.Decode(assetNames)
	if err != nil {
		panic(err)
	}
	dec = json.NewDecoder(tempFile)
	guildEmotes := new(GuildEmotes)
	err = dec.Decode(guildEmotes)
	if err != nil {
		panic(err)
	}
	for _, v := range assetNames.Names {
		for _, w := range *guildEmotes {
			if v == w.Name {
				req, err := http.NewRequest(
					"DELETE",
					"https://discordapp.com/api/guilds/"+GUILD_ID+"/emojis"+w.Name,
					nil,
				)
				if err != nil {
					panic(err)
				}
				http.DefaultClient.Do(req)
				break
			}
		}
		// check if assets/emotes/emoticons/4x/EMOTE exists
		// POST emote if exists "https://discordapp.com/api/guilds/"+GUILD_ID+"/emojis"
	}
}

func main() {
	tempFile, err := ioutil.TempFile(os.TempDir(), "emoji-")
	if err != nil {
		panic(err)
	}
	defer tempFile.Close()
	defer os.Remove(tempFile.Name())
	// getGuildEmotes()
	emotesFile, err := os.Open("emotes.json")
	if err != nil {
		panic(err)
	}
	defer emotesFile.Close()
	compareEmotes(tempFile, emotesFile)
}
