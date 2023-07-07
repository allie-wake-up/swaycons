# swaycons

Window Icons in Sway with Nerd Fonts!

## Why?

- I heavily use sway tabs and adding icons makes it a lot easier to visually pick out the window that I'm looking for.
- Sway issue discussing window icons: https://github.com/swaywm/sway/issues/4882
- Workarounds using just the sway config have significant limitations. Sway will only trigger changes for `for_window [title="firefox"] title_format {icon} %title` type configs once.  That makes it less than ideal for icons in many cases.
- Because swaycons has a default config included, it's a lot simpler to get running than configuring font icons manually in a sway config

Go from 
![before](/screenshots/before.png)

to

![after](/screenshots/after.png)

## Installation

### Prerequisites

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Sway
3. A font with icons and [pango](https://docs.gtk.org/Pango/) enabled in your sway config. The default config assumes a nerd font
   is used.  The config should look something like this:

```
font pango:FuraCode Nerd Font 11
```

### Install Swaycons

1. `cargo install swaycons`


## Usage

- run `swaycons`
- recommend adding something like `exec swaycons` to sway config
- swaycons must be restarted after config changes


## Config

- by default the config file should be placed at `~/.config/swaycons/config.toml`, but it will look for `swaycons/config.toml` in whatever your configured XDG config folder is.
- the default config is included in the binary and can be viewed [here](src/config.toml)
- Your custom config will add to the default config
- The best place to find icons for nerd fonts is the [nerd fonts cheat sheet](https://www.nerdfonts.com/cheat-sheet)
- learn about pango Attributes [here](https://docs.gtk.org/Pango/pango_markup.html#the-span-attributes)
- to find the `app_id` or `class` I recommend running `swaymsg -t get_tree | less` and using `/` to search for the app you're looking for
- There are a lot of great resources to learn how to write regular expressions out there.  The examples in the title section will cover most simple cases though.
- Here is an example with comments:

```toml

# global section. all windows will default to these settings
[global]
color = "#FFFFFF" # this must be a valid color
focused_color = "#FFFFFF" # to disable a focused_color set this to ""
icon = "󰖯" # to disable a default icon just set this to ""
size = "14pt" # must be a valid pango size value
separator = " " # anything between the icon and window title

# app_id section.  This does an exact string comparison to the app_id or
# window_properties.class value reported in swaymsg -t get_tree for the window
# It will be app_id for wayland apps and window_properties.class for X11 apps
[app_id]
chromium = { icon = "", color = "#a1c2fa", size = "13pt" }
firefox = { icon = "", color = "#ff8817" }
foot = { icon = "" }
neovide = { icon = "", color = "#8fff6d" }

# This does a regex match on the window title.  Matches from this section
# will take precedence over matches from the app_id section. A very basic
# algorithm is used to select the more exact regex if there are multiple
# matches. If 1 regex contains another it will choose the longer one. For
# instance mail\\.google\\.com and google\\.com/maps will be chosen over 
# google\\.com
[title]
# escape . for an exact match.  Normally . matches any character
"crates\\.io" = { icon = "󰏗", color = "#ffc933" }
"github\\.com" = { icon = "" }
"google\\.com" = { icon = "", color = "#4285f4" }
"google\\.com/maps" = { icon = "󰗵", color = "#4caf50" }
"mail\\.google\\.com" = { icon = "󰊫", color = "#ad1f1c" }

# use | for or
"sr\\.ht|sourcehut\\.org" = { icon = "" }

# can do an or around just a substring with (a|b)
"travis-ci\\.(com|org)" = { icon = "", color = "#cd324a" } 

# The app_id setting means that this will only match if both the title matches
# the regex and the app_id or window_properties.class equals one of the values
# provided in the app_id array
# For example this allows a vim logo in the terminal but keeps a github logo
# when viewing a github page with vim in the repository name
vim = { app_id = ["foot", "Alacritty"], icon = "", color = "#8fff6d" }
```

## Replace Firefox Tabs with Sway Tabs

This plugin is extremely useful when using sway tabs instead of browser tabs.  To get this working properly with Firefox a few steps and plugins are necessary:

1. Open Firefox Settings and Disable `Open links in tabs instead of new windows`
2. Install [Tab-less](https://github.com/iainbeeston/tab-less) from [Firefox Add-ons](https://addons.mozilla.org/en-US/firefox/search/?q=tab-less)
3. Install [URL in Title](https://github.com/M-Gregoire/Firefox-AddUrlToWindowsTitle) from [Firefox Add-ons](https://addons.mozilla.org/en-US/firefox/addon/url-in-title-keepass/?utm_source=addons.mozilla.org&utm_medium=referral&utm_content=search)
4. Hide the tab bar - directions are in the [sidebery github wiki](https://github.com/mbnuqw/sidebery/wiki/Firefox-Styles-Snippets-(via-userChrome.css)#firefox-styles-snippets-via-userchromecss)
