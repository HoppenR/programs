package main

import (
	"errors"
	"fmt"
	"github.com/gdamore/tcell/v2"
	"github.com/rivo/tview"
	"net/url"
	"os"
	"os/exec"
	"strings"
	"time"
)

type UI struct {
	app   *tview.Application
	pg1   MainPage
	pg2   PopupPage
	pages *tview.Pages
}

type PopupPage struct {
	win   *tview.Flex
	input *tview.InputField
}

type MainPage struct {
	win         *tview.Flex
	streamList  *tview.List
	displayList *tview.List
	streamInfo  *tview.TextView
}

// TODO: return errors where the program should exit and show errors on the
// StreamInfo panel when it should not

// TODO: move this into PrintMenu so that I can return errors?
func (ui *UI) inputHandler(event *tcell.EventKey) *tcell.EventKey {
	sel := ui.pg1.displayList.GetCurrentItem()
	cnt := ui.pg1.displayList.GetItemCount()
	switch event.Key() {
	case tcell.KeyRune:
		switch event.Rune() {
		case 'g':
			ui.pg1.displayList.SetCurrentItem(0)
			return nil
		case 'G':
			ui.pg1.displayList.SetCurrentItem(cnt - 1)
			return nil
		case 'M':
			ui.pg1.displayList.SetCurrentItem((cnt - 1) / 2)
			return nil
		case 'j':
			if sel == cnt-1 {
				ui.pg1.displayList.SetCurrentItem(0)
			} else {
				ui.pg1.displayList.SetCurrentItem(ui.pg1.displayList.GetCurrentItem() + 1)
			}
			return nil
		case 'k':
			if sel == 0 {
				ui.pg1.displayList.SetCurrentItem(cnt - 1)
			} else {
				ui.pg1.displayList.SetCurrentItem(ui.pg1.displayList.GetCurrentItem() - 1)
			}
			return nil
		case 'f':
			ui.pages.ShowPage("Popup")
			// store selection?
		case 'l':
			primaryText, _ := ui.pg1.displayList.GetItemText(sel)
			ui.openLink(primaryText)
			return nil
		case 'q':
			ui.app.Stop()
			return nil
		}
	case tcell.KeyEnter:
		primaryText, _ := ui.pg1.displayList.GetItemText(sel)
		ui.openLink(primaryText)
		return nil
	}
	// Let the default list primitive key event handler handle the rest
	return event
}

func (ui *UI) openLink(userName string) {
	// Is there a good way to handle errors here?
	// move function?
	if userName == "" {
		ui.pg1.streamInfo.SetText("[red]⚠Cannot open empty result[-]")
		return
	}
	browser := os.Getenv("BROWSER")
	if browser == "" {
		ui.pg1.streamInfo.SetText("[red]⚠set $BROWSER before opening links[-]")
		return
	}
	ui.app.Stop()
	q := url.Values{
		"channel": {strings.ToLower(userName)},
		"parent":  {"strims.gg"},
	}
	u := &url.URL{
		Scheme:   "https",
		Host:     "player.twitch.tv",
		RawQuery: q.Encode(),
	}
	cmd := exec.Command(browser, u.String())
	err := cmd.Start()
	if err != nil {
		panic("Error forking $BROWSER")
	}
}

func printMenu(games *GameInfo, channels *Channels) (err error) {
	if len(channels.Data) == 0 {
		return errors.New("No live channels found")
	}
	ui := &UI{
		app:   tview.NewApplication(),
		pages: tview.NewPages(),
		pg1: MainPage{
			win:         tview.NewFlex(),
			streamList:  tview.NewList(),
			displayList: tview.NewList(),
			streamInfo:  tview.NewTextView(),
		},
		pg2: PopupPage{
			input: tview.NewInputField(),
		},
	}
	ui.app.SetBeforeDrawFunc(func(screen tcell.Screen) bool {
		screen.Clear()
		return false
	})
	err = ui.setupMainPage(games, channels)
	if err != nil {
		return err
	}
	ui.setupPopupPage()
	ui.pages.AddPage("Main Window", ui.pg1.win, true, true)
	ui.pages.AddPage("Popup", ui.pg2.win, true, false)
	err = ui.app.SetRoot(ui.pages, true).Run()
	if err != nil {
		return err
	}
	return nil
}

func (ui *UI) setupMainPage(games *GameInfo, channels *Channels) error {
	ui.pg1.win.AddItem(ui.pg1.displayList, 0, 1, true)
	ui.pg1.win.AddItem(ui.pg1.streamInfo, 0, 4, false)
	for i := 0; i < len(channels.Data); i++ {
		v := &channels.Data[i]
		gameName, err := games.getName(v.GameID)
		if err != nil {
			return err
		}
		secondaryText := fmt.Sprintf(
			" %-6d[green:-:u]%s[-:-:-]",
			v.ViewerCount, tview.Escape(gameName),
		)
		ui.pg1.streamList.AddItem(v.UserName, secondaryText, 0, nil)
		ui.pg1.displayList.AddItem(v.UserName, secondaryText, 0, nil)
	}
	ui.pg1.displayList.SetBackgroundColor(0)
	ui.pg1.displayList.SetSecondaryTextColor(0)
	ui.pg1.displayList.SetTitle("Live Streams")
	ui.pg1.displayList.SetInputCapture(ui.inputHandler)
	// ui.pg1.displayList.SetSelectedFunc(ui.openLink)
	ui.pg1.streamInfo.SetDynamicColors(true)
	ui.pg1.streamInfo.SetWrap(false) // Default?
	updateStreamInfo := func(ix int, pri, sec string, _ rune) {
		var dataix = ui.pg1.streamList.FindItems(pri, sec, true, false)
		if dataix == nil {
			ui.pg1.streamInfo.SetText("No results")
			return
		}
		var startLocal time.Time = channels.Data[dataix[0]].StartedAt.Local()
		ui.pg1.streamInfo.SetText(
			fmt.Sprintf("[red]Title[-]: %s\n[red]Started At[-]: %2.2d:%2.2d\n",
				tview.Escape(channels.Data[dataix[0]].Title),
				startLocal.Hour(),
				startLocal.Minute(),
			),
		)
	}
	ui.pg1.displayList.SetChangedFunc(updateStreamInfo)
	ui.pg1.streamInfo.SetBackgroundColor(0)
	ui.pg1.streamInfo.SetTitle("Stream Info")
	ui.pg1.streamInfo.SetBorder(true)
	ui.pages.SetBackgroundColor(0)
	// initialize StreamInfo with the 0th item
	pri, sec := ui.pg1.displayList.GetItemText(0)
	updateStreamInfo(0, pri, sec, 0)
	return nil
}

func (ui *UI) setupPopupPage() {
	// Ftwpala jbpratt I could just check if the filter state is the same
	// before and after the dialog, and if it is restore the selection
	ui.pg2.input.SetBorder(true).SetBackgroundColor(0).SetBorder(true)
	ui.pg2.input.SetLabel("Filter:").SetFieldWidth(20).SetAcceptanceFunc(tview.InputFieldInteger)
	ui.pg2.input.SetDoneFunc(func(key tcell.Key) { ui.pages.HidePage("Popup") /* restore selection? */ })
	ui.pg2.input.SetAcceptanceFunc(func(string, rune) bool {
		return true
	})
	ui.pg2.input.SetChangedFunc(func(filter string) {
		ui.pg1.displayList.Clear()
		if filter == "" {
			for i := 0; i < ui.pg1.streamList.GetItemCount(); i++ {
				mainstr, secstr := ui.pg1.streamList.GetItemText(i)
				ui.pg1.displayList.AddItem(mainstr, secstr, 0, nil)
			}
			return
		}
		var ixs []int = ui.pg1.streamList.FindItems(filter, filter, false, true)
		if ixs == nil {
			ui.pg1.displayList.AddItem("", "", 0, nil)
			return
		}
		for _, v := range ixs {
			mainstr, secstr := ui.pg1.streamList.GetItemText(v)
			ui.pg1.displayList.AddItem(mainstr, secstr, 0, nil)
		}
	})
	ui.pg2.input.SetAcceptanceFunc(tview.InputFieldMaxLength(19))
	ui.pg2.win = tview.NewFlex().
		AddItem(nil, 0, 1, false).
		AddItem(tview.NewFlex().SetDirection(tview.FlexRow).
			AddItem(nil, 0, 1, false).
			AddItem(ui.pg2.input, 3, 1, true).
			AddItem(nil, 0, 1, false), 30, 1, true).
		AddItem(nil, 0, 1, false)
}
