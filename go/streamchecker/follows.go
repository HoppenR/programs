package main

import (
	"encoding/json"
	"io/ioutil"
	"net/http"
	"net/url"
	"time"
)

type Follows struct {
	Total      int              `json:"total"`
	Data       []FollowData     `json:"data"`
	Pagination FollowPagination `json:"pagination"`
}

type FollowData struct {
	FromID     string    `json:"from_id"`
	FromName   string    `json:"from_name"`
	ToID       string    `json:"to_id"`
	ToName     string    `json:"to_name"`
	FollowedAt time.Time `json:"followed_at"`
}

type FollowPagination struct {
	Cursor string `json:"cursor"`
}

func (lhs *Follows) update(rhs *Follows) {
	lhs.Pagination = rhs.Pagination
	lhs.Data = append(lhs.Data, rhs.Data...)
}

func getFollowsPart(token, clientID, userID, pagCursor string) (followsPart string, err error) {
	req, err := http.NewRequest("GET", "https://api.twitch.tv/helix/users/follows", nil)
	if err != nil {
		return "", err
	}
	req.Header.Add("Authorization", "Bearer "+token)
	req.Header.Add("Client-Id", clientID)
	query := make(url.Values)
	query.Add("from_id", userID)
	query.Add("first", "100")
	query.Add("after", pagCursor)
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
	followsPart = string(jsonBody)
	return followsPart, nil
}

func getAllFollows(token, clientID, userID string) (follows *Follows, err error) {
	jsonBody, err := getFollowsPart(token, clientID, userID, "")
	if err != nil {
		return nil, err
	}
	err = json.Unmarshal([]byte(jsonBody), &follows)
	if err != nil {
		return nil, err
	}
	for len(follows.Data) != follows.Total {
		jsonBody, err = getFollowsPart(token, clientID, userID, follows.Pagination.Cursor)
		if err != nil {
			return nil, err
		}
		tmpFollows := new(Follows)
		err = json.Unmarshal([]byte(jsonBody), &tmpFollows)
		if err != nil {
			return nil, err
		}
		follows.update(tmpFollows)
	}
	return follows, nil
}
