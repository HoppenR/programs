# Grades TUI

Grades TUI is a command-line university tool written in rust that allows
tracking academic progress by building up a digital record of courses, grades,
and accomplishments.

The tool utilizes a tree-like structure, which can be navigated and manipulated
using a cursor.

## Features

- Courses, semesters, moments, and tasks can be interactively added with their relevant information.
- Entries can directly be edited or removed.
- The accumulated and total ECTS credits for completed courses are displayed for each period and semester.
- The data can be saved to and loaded from a file.
- Low amount of dependencies. The only direct dependencies are `libc` and `serde_json`
- Colorcoded entries depending on status.

## Usage

The tool can be run by cloning the repository and running the following command:

```sh
cargo run filename
```

Navigation of the tree-like user interface is possible with the following commands:

- `j` or `↓`: Move the cursor down one level.
- `k` or `↑`: Move the cursor up one level.
- `l` or `→`: Move the cursor to the right node.
- `h` or `←`: Move the cursor to the left node.

The following actions can be performed using the appropriate commands:

- `a`: Add a new node.
- `e`: Edit an existing node.
- `d`: Delete a node.
- `q`: Quit.
- `<C-e>`: Scroll the window down one line.
- `<C-y>`: Scroll the window up one line.
