package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"net/url"
)

type GameInfo struct {
	GameIdentities []GameIdentity `json:"data"`
	IDToName       map[string]string
}

type GameIdentity struct {
	ID   string `json:"id"`
	Name string `json:"name"`
}

func getGameInfo(token, clientID string, channels *Channels) (games *GameInfo, err error) {
	req, err := http.NewRequest("GET", "https://api.twitch.tv/helix/games", nil)
	if err != nil {
		return nil, err
	}
	req.Header.Add("Authorization", "Bearer "+token)
	req.Header.Add("Client-Id", clientID)
	query := make(url.Values)
	for _, v := range channels.Data {
		query.Add("id", v.GameID)
	}
	req.URL.RawQuery = query.Encode()
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()
	jsonBody, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	err = json.Unmarshal([]byte(jsonBody), &games)
	return games, nil
}

func (g *GameInfo) getName(id string) (gameName string, err error) {
	if id == "" {
		return "None", nil
	}
	if g.IDToName == nil {
		// Populate map
		g.IDToName = make(map[string]string)
		for _, v := range g.GameIdentities {
			g.IDToName[v.ID] = v.Name
		}
	}
	gameName, ok := g.IDToName[id]
	if !ok {
		return "", fmt.Errorf("ID does not exist")
	}
	return gameName, nil
}
