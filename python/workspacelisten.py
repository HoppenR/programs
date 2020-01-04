#!/usr/bin/env python3

import i3ipc

i3 = i3ipc.Connection()

def on_workspace_focus(self,event):
    if(event.current):
        if(event.current.num == 3):
            i3.command('mode "minimal"')
        elif(event.old.num == 3):
            i3.command('mode "default"')

i3.on('workspace::focus', on_workspace_focus)
i3.main()
