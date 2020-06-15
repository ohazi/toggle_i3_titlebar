# toggle_i3_titlebar

## Deprecation Notice
Use [oi3h](https://github.com/ohazi/oi3h/) instead. Specifically, you can use:
```
$ oi3h border -t 'normal 2' 'pixel 2'
```
or something similar to achieve equivalent functionality.

## What does this do?

This program toggles titlebar visibility for the currently focused window in i3. 

More specifically, it toggles between the pair:
  - "border normal 2"
  - "border pixel 2"

instead of the triplet:
  - "border normal 2"
  - "border pixel 0"
  - "border pixel 2"

which is what the "border toggle" i3 command does. 

## Why would I want this?

You might want to be able to quickly show and then hide the titlebar, for example to read the title text, or to see if a window has a mark set. If you want to use a single key binding with the "border toggle" command, you'll have to press the key once to turn the titlebar on, and then twice to turn it off again (or vice-versa, if your theme/style doesn't use window borders). This is annoying. Another option is to define two key bindings -- one to show the titlebar, and another to hide it. This is also annoying. 

## How do I use this?

### Build
```
$ cargo build --release
$ cp target/release/toggle_i3_titlebar /somewhere/in/your/$PATH/
```
### Use
`~/.config/i3/config`
```
# manually enable / disable titlebar
#bindsym $mod+t border toggle                           # (︶︹︺)
#bindsym $mod+t border normal 2                         # (︶︹︺)
#bindsym $mod+Shift+t border pixel 2
bindsym $mod+t exec --no-startup-id toggle_i3_titlebar  # <(￣︶￣)>
```

## etc.

If you don't use a window border, or use a value other than 2 pixels, you'll need to modify the source, but that should be pretty easy. TODO: command line flags? I originally wrote this in Python, but the time it takes for the Python interpreter to start is noticable.
