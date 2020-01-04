#!/bin/env python3
import i3ipc

i3 = i3ipc.Connection()

cons = i3.get_tree().find_focused().workspace().nodes

for con in cons:
    children = con.descendants()
    if not children:
        print(con.name, con.percent)
    else:
        print(con.layout, con.percent)
    indents = 1
    for child in children:
        if child.name == None:
            print("\t" * indents, child.layout, child.percent)
            indents += 1
        else:
            print("\t" * indents, child.name, child.percent)
