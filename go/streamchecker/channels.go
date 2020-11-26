package main

import (
	"encoding/json"
	"io/ioutil"
	"net/http"
	"net/url"
	"time"
)

type Channels struct {
	Data []StreamData `json:"data"`
}

type StreamData struct {
	GameName    string    `json:"game_name"`
	Language    string    `json:"language"`
	Title       string    `json:"title"`
	Type        string    `json:"type"`
	UserName    string    `json:"user_name"`
	ViewerCount int       `json:"viewer_count"`
	StartedAt   time.Time `json:"started_at"`
}

func (lhs *Channels) update(rhs *Channels) {
	lhs.Data = append(lhs.Data, rhs.Data...)
}

func (si *Channels) Less(i, j int) bool {
	return si.Data[i].ViewerCount < si.Data[j].ViewerCount
}

func (si *Channels) Len() int {
	return len(si.Data)
}

func (si *Channels) Swap(i, j int) {
	si.Data[i], si.Data[j] = si.Data[j], si.Data[i]
}

func getLiveChannelsPart(token, clientID string, follows *Follows, first int) (
	channelsPart string, err error,
) {
	req, err := http.NewRequest("GET", "https://api.twitch.tv/helix/streams", nil)
	if err != nil {
		return "", err
	}
	req.Header.Add("Authorization", "Bearer "+token)
	req.Header.Add("Client-Id", clientID)
	query := make(url.Values)
	for i := first; i != follows.Total && i < (first+100); i++ {
		query.Add("user_id", follows.Data[i].ToID)
	}
	query.Add("first", "100")
	req.URL.RawQuery = query.Encode()
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()
	jsonBody, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return "", err
	}
	channelsPart = string(jsonBody)
	return channelsPart, nil
}

func initializeChannels(token, clientID string, follows *Follows) (channels *Channels, err error) {
	jsonBody, err := getLiveChannelsPart(token, clientID, follows, 0)
	if err != nil {
		return nil, err
	}
	err = json.Unmarshal([]byte(jsonBody), &channels)
	if err != nil {
		return nil, err
	}
	for i := 100; i < follows.Total; i += 100 {
		jsonBody, err = getLiveChannelsPart(token, clientID, follows, i)
		if err != nil {
			return nil, err
		}
		tmpChannels := new(Channels)
		err = json.Unmarshal([]byte(jsonBody), &tmpChannels)
		if err != nil {
			return nil, err
		}
		channels.update(tmpChannels)
	}
	return channels, nil
}
