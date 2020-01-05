package main

import (
	"errors"
	"net/url"
	"os"
	"os/exec"
	"strings"

	sc "github.com/HoppenR/streamchecker"
)

type OpenMethod int

const (
	lnkOpenEmbed OpenMethod = iota
	lnkOpenHomePage
	lnkOpenStrims
)

func (ui *UI) openSelectedStream(method OpenMethod) {
	printerr := func(errstr string) {
		ui.pg1.streamInfo.SetText("[red]⚠ " + errstr + "[-]")
	}
	listIdx := ui.pg1.focusedList.GetCurrentItem()
	var data sc.StreamData
	primaryText, _ := ui.pg1.focusedList.GetItemText(listIdx)
	switch ui.pg1.focusedList.GetTitle() {
	case "Twitch":
		for _, v := range ui.pg1.streams.Twitch.Data {
			if v.UserName == primaryText {
				data = &v
				break
			}
		}
	case "Strims":
		for _, v := range ui.pg1.streams.Strims.Data {
			if v.Channel == primaryText {
				data = &v
				break
			}
		}
	}
	if data == nil {
		printerr("Cannot open empty result")
		return
	}
	browser := os.Getenv("BROWSER")
	if browser == "" {
		printerr("set $BROWSER before opening links")
		return
	}
	rawURL, err := streamToUrlString(data, method)
	if err != nil {
		printerr(err.Error())
		return
	}
	if exec.Command(browser, rawURL).Run() != nil {
		printerr("Error forking $BROWSER")
	}
}

func streamToUrlString(data sc.StreamData, method OpenMethod) (string, error) {
	var (
		q url.Values
		u *url.URL
	)
	switch method {
	case lnkOpenEmbed:
		switch data.GetService() {
		case "angelthump":
			q = url.Values{
				"channel": {strings.ToLower(data.GetName())},
			}
			u = &url.URL{
				Host: "player.angelthump.com",
			}
		case "m3u8":
			u = &url.URL{
				Host: "strims.gg",
				Path: "m3u8/" + data.GetName(),
			}
		case "twitch", "twitch-followed":
			q = url.Values{
				"channel": {strings.ToLower(data.GetName())},
				"parent":  {"strims.gg"},
			}
			u = &url.URL{
				Host: "player.twitch.tv",
			}
		case "twitch-vod":
			q = url.Values{
				"video":  {"v" + data.GetName()},
				"parent": {"strims.gg"},
			}
			u = &url.URL{
				Host: "player.twitch.tv",
			}
		case "youtube":
			q = url.Values{
				"autoplay": {"true"},
			}
			u = &url.URL{
				Host: "www.youtube.com",
				Path: "embed/" + data.GetName(),
			}
		default:
			return "", errors.New("Platform " + data.GetService() + " not implemented!")
		}
	case lnkOpenHomePage:
		switch data.GetService() {
		case "angelthump":
			u = &url.URL{
				Host: "angelthump.com",
				Path: data.GetName(),
			}
		case "m3u8":
			u = &url.URL{
				Host: "strims.gg",
				Path: "m3u8/" + data.GetName(),
			}
		case "twitch", "twitch-followed":
			u = &url.URL{
				Host: "www.twitch.tv",
				Path: data.GetName(),
			}
		case "twitch-vod":
			u = &url.URL{
				Host: "www.twitch.tv",
				Path: "videos/" + data.GetName(),
			}
		case "youtube":
			u = &url.URL{
				Host: "www.youtube.com",
				Path: "watch",
			}
			q = url.Values{
				"v": {data.GetName()},
			}
		default:
			return "", errors.New("Platform " + data.GetService() + " not implemented!")
		}
	case lnkOpenStrims:
		u = &url.URL{
			Host: "strims.gg",
			Path: strings.Replace(
				data.GetService(),
				"-followed",
				"",
				1,
			) + "/" + strings.ToLower(data.GetName()),
		}
	}
	u.Scheme = "https"
	u.RawQuery = q.Encode()
	return u.String(), nil
}

func embedString(rawURL string) (string, error) {
	// TODO: Better ways of splitting the url path fields?
	URL, err := url.Parse(rawURL)
	if err != nil {
		return "", err
	}
	var data sc.StreamData
	switch URL.Host {
	case "angelthump.com":
		data = &sc.StrimsStreamData{
			Channel: URL.Path[1:],
			Service: "angelthump",
		}
	case "www.twitch.tv":
		if URL.Path[1:7] == "videos" {
			data = &sc.StrimsStreamData{
				Channel: URL.Path[8:],
				Service: "twitch-vod",
			}
		} else {
			data = &sc.TwitchStreamData{
				UserName: URL.Path[1:],
			}
		}
	case "strims.gg":
		if URL.Path[1:5] == "m3u8" {
			return URL.String(), nil
		}
		// TODO: handle more cases than m3u8
		panic("not handled")
	case "www.youtube.com":
		data = &sc.StrimsStreamData{
			Channel: URL.Query().Get("v"),
			Service: "youtube",
		}
	}
	if data == nil {
		return "", errors.New("url not handled")
	}
	return streamToUrlString(data, lnkOpenEmbed)
}
