package main

import (
	"errors"
	"fmt"
	"net/url"
	"os"
	"os/exec"
	"regexp"
	"strings"

	"github.com/gdamore/tcell/v2"
	"github.com/rivo/tview"
)

type UI struct {
	app   *tview.Application
	pages *tview.Pages
	pg1   MainPage
	pg2   PopupPage
}

type PopupPage struct {
	con   *tview.Grid
	input *tview.InputField
}

type MainPage struct {
	con         *tview.Flex
	displayList *tview.List
	streamInfo  *tview.TextView
	streamList  Channels
}

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
		case 'F':
			ui.pg2.input.SetText("")
		case 'l':
			primaryText, _ := ui.pg1.displayList.GetItemText(sel)
			ui.openLink(primaryText)
			return nil
		case 'q':
			ui.app.Stop()
			return nil
		case '/':
			ui.pages.ShowPage("Popup")
		}
	case tcell.KeyEnter, tcell.KeyCtrlJ:
		primaryText, _ := ui.pg1.displayList.GetItemText(sel)
		ui.openLink(primaryText)
		return nil
	}
	// Let the default list primitive key event handler handle the rest
	return event
}

func (ui *UI) openLink(userName string) {
	// We don't need to handle errors in here, so just print them?
	printerr := func(errstr string) {
		ui.pg1.streamInfo.SetText("[red]⚠" + errstr + "[-]")
	}
	if userName == "" {
		printerr("Cannot open empty result")
		return
	}
	browser := os.Getenv("BROWSER")
	if browser == "" {
		printerr("set $BROWSER before opening links")
		return
	}
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
		printerr("Error forking $BROWSER")
		return
	}
	ui.app.Stop()
}

func initUI(channels *Channels) (err error) {
	if len(channels.Data) == 0 {
		return errors.New("No live channels found")
	}
	ui := &UI{
		app:   tview.NewApplication(),
		pages: tview.NewPages(),
		pg1: MainPage{
			con:         tview.NewFlex(),
			displayList: tview.NewList(),
			streamInfo:  tview.NewTextView(),
			streamList:  *channels,
		},
		pg2: PopupPage{
			con:   tview.NewGrid(),
			input: tview.NewInputField(),
		},
	}
	ui.app.SetBeforeDrawFunc(func(screen tcell.Screen) bool {
		screen.Clear()
		return false
	})
	ui.pages.SetBackgroundColor(tcell.ColorDefault)
	err = ui.setupMainPage(channels)
	if err != nil {
		return err
	}
	ui.setupPopupPage()
	ui.pages.AddPage("Main Window", ui.pg1.con, true, true)
	ui.pages.AddPage("Popup", ui.pg2.con, true, false)
	err = ui.app.SetRoot(ui.pages, true).Run()
	if err != nil {
		return err
	}
	return nil
}

func (ui *UI) setupMainPage(channels *Channels) error {
	// DisplayList
	ui.pg1.con.AddItem(ui.pg1.displayList, 0, 1, true)
	updateStreamInfo := func(ix int, pri, sec string, _ rune) {
		var index int = -1
		for i, v := range channels.Data {
			if pri == v.UserName {
				index = i
				break
			}
		}
		add := func(c string) {
			ui.pg1.streamInfo.Write([]byte(c))
		}
		ui.pg1.streamInfo.Clear()
		if index == -1 {
			add("No results")
		} else {
			// TODO: figure out how to add tags to streaminfo
			selChannel := &channels.Data[index]
			startLocal := selChannel.StartedAt.Local()
			selChannel.Title = tview.Escape(selChannel.Title)
			if selChannel.GameName == "" {
				selChannel.GameName = "[::d]None[::-]"
			}
			selChannel.Title = strings.ReplaceAll(selChannel.Title, "\n", " ")
			add(fmt.Sprintf("[red]Title[-]: %s\n", selChannel.Title))
			add(fmt.Sprintf("[red]Viewers[-]: %d\n", selChannel.ViewerCount))
			add(fmt.Sprintf("[red]Game[-]: %s\n", selChannel.GameName))
			add(fmt.Sprintf("[red]Started At[-]: %2.2d:%2.2d\n",
				startLocal.Hour(),
				startLocal.Minute()))
			add(fmt.Sprintf("[red]Language[-]: %s\n", selChannel.Language))
			add(fmt.Sprintf("[red]Type[-]: %s\n", selChannel.Type))
		}
	}
	ui.pg1.displayList.SetChangedFunc(updateStreamInfo)
	for i := 0; i < len(channels.Data); i++ {
		v := &channels.Data[i]
		secondaryText := fmt.Sprintf(
			" %-6d[green:-:u]%s[-:-:-]",
			v.ViewerCount, tview.Escape(v.GameName),
		)
		ui.pg1.displayList.AddItem(v.UserName, secondaryText, 0, nil)
	}
	ui.pg1.displayList.SetBackgroundColor(tcell.ColorDefault)
	ui.pg1.displayList.SetBorder(true)
	ui.pg1.displayList.SetBorderPadding(0, 0, 1, 1)
	ui.pg1.displayList.SetInputCapture(ui.inputHandler)
	ui.pg1.displayList.SetSecondaryTextColor(tcell.ColorDefault)
	ui.pg1.displayList.SetTitle("Live Streams")
	// StreamInfo
	ui.pg1.con.AddItem(ui.pg1.streamInfo, 0, 4, false)
	ui.pg1.streamInfo.SetBackgroundColor(tcell.ColorDefault)
	ui.pg1.streamInfo.SetBorder(true)
	ui.pg1.streamInfo.SetDynamicColors(true)
	ui.pg1.streamInfo.SetTitle("Stream Info")
	ui.pg1.streamInfo.SetWrap(false)
	return nil
}

func (ui *UI) setupPopupPage() {
	ui.pg2.input.SetBackgroundColor(tcell.ColorDefault)
	ui.pg2.input.SetBorder(true)
	ui.pg2.input.SetTitle("Filter")
	ui.pg2.input.SetFinishedFunc(func(key tcell.Key) { ui.pages.HidePage("Popup") })
	ui.pg2.input.SetChangedFunc(ui.filterPage)
	const (
		PopupWidth  = 26
		PopupHeight = 3
	)
	ui.pg2.input.SetAcceptanceFunc(tview.InputFieldMaxLength(PopupWidth - 3))
	ui.pg2.con.SetColumns(0, PopupWidth, 0)
	ui.pg2.con.SetRows(0, PopupHeight, 0)
	ui.pg2.con.AddItem(ui.pg2.input, 1, 1, 1, 1, 0, 0, true)
}

func (ui *UI) matchStreamlistIndex(filter string, inverted bool) []int {
	var ixs []int
	re := regexp.MustCompile("(?i)" + regexp.QuoteMeta(filter))
	for i, v := range ui.pg1.streamList.Data {
		matches := []bool{
			re.MatchString(v.GameName),
			re.MatchString(v.Title),
			re.MatchString(v.UserName),
		}
		valid := inverted
		for _, v := range matches {
			if v && !inverted {
				valid = true
				break
			}
			if v && inverted {
				valid = false
				break
			}
		}
		if valid {
			ixs = append(ixs, i)
		}
	}
	return ixs
}

func (ui *UI) filterPage(filter string) {
	ui.pg1.displayList.Clear()
	inverted := false
	if len(filter) > 0 && filter[0] == '!' {
		inverted = true
		filter = filter[1:]
	}
	ixs := ui.matchStreamlistIndex(filter, inverted)
	if ixs == nil {
		ui.pg1.displayList.AddItem("", "", 0, nil)
		return
	}
	for _, v := range ixs {
		mainstr := ui.pg1.streamList.Data[v].UserName
		secstr := fmt.Sprintf(
			" %-6d[green:-:u]%s[-:-:-]",
			ui.pg1.streamList.Data[v].ViewerCount,
			tview.Escape(ui.pg1.streamList.Data[v].GameName),
		)
		ui.pg1.displayList.AddItem(mainstr, secstr, 0, nil)
	}
}
