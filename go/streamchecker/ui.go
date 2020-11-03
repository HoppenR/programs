package main

import (
	"fmt"
	"github.com/gdamore/tcell/v2"
	"github.com/rivo/tview"
	"os"
)

type UI struct {
	app        *tview.Application
	list       *tview.List
	errorState error
}

func (ui *UI) inputHandler(event *tcell.EventKey) *tcell.EventKey {
	sel := ui.list.GetCurrentItem()
	cnt := ui.list.GetItemCount()
	switch event.Key() {
	case tcell.KeyRune:
		switch event.Rune() {
		case 'g':
			ui.list.SetCurrentItem(0)
			return nil
		case 'G':
			ui.list.SetCurrentItem(cnt - 1)
			return nil
		case 'M':
			ui.list.SetCurrentItem((cnt - 1) / 2)
			return nil
		case 'j':
			if sel == cnt-1 {
				ui.list.SetCurrentItem(0)
			} else {
				ui.list.SetCurrentItem(ui.list.GetCurrentItem() + 1)
			}
			return nil
		case 'k':
			if sel == 0 {
				ui.list.SetCurrentItem(cnt - 1)
			} else {
				ui.list.SetCurrentItem(ui.list.GetCurrentItem() - 1)
			}
			return nil
		case 'l':
			// Is calling SelectedFunc immediately a good idea?
			primaryText, _ := ui.list.GetItemText(sel)
			ui.openLink(0, primaryText, "", 0)
			return nil
		case 'q':
			ui.app.Stop()
			return nil
		}
	}
	// Let the default list primitive key event handler handle the rest
	return event
}

func (ui *UI) openLink(_ int, userName string, _ string, _ rune) {
	ui.app.Stop()
	browser := os.Getenv("BROWSER")
	if browser == "" {
		// Is there a good way to handle errors here?
		ui.errorState = fmt.Errorf("set $BROWSER before opening links")
	} else {
		fmt.Printf(
			"%s %s%s%s\n", browser,
			"https://player.twitch.tv?channel=", userName, "&parent=strims.gg",
		)
	}
}

func printMenu(games *GameInfo, channels *Channels) (err error) {
	ui := &UI{
		app:  tview.NewApplication(),
		list: tview.NewList(),
	}
	ui.app.SetBeforeDrawFunc(func(screen tcell.Screen) bool {
		screen.Clear()
		return false
	})
	for i := 0; i < len(channels.Data); i++ {
		v := &channels.Data[i]
		gameName, err := games.getName(v.GameID)
		if err != nil {
			return err
		}
		secondaryText := fmt.Sprintf(
			" %-6d[green:-:u]%s[-:-:-]: %s",
			v.ViewerCount, tview.Escape(gameName),
			tview.Escape(v.Title),
		)
		ui.list.AddItem(v.UserName, secondaryText, 0, nil)
	}
	ui.list.SetBackgroundColor(0)
	ui.list.SetSecondaryTextColor(0)
	ui.list.SetTitle("Streamchecker")
	ui.list.SetBorder(true)
	ui.list.SetInputCapture(ui.inputHandler)
	ui.list.SetSelectedFunc(ui.openLink)
	err = ui.app.SetRoot(ui.list, true).SetFocus(ui.list).Run()
	if err != nil {
		return err
	}
	if ui.errorState != nil {
		return ui.errorState
	}
	return nil
}
