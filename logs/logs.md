# Development logs
> The chronological order of updates for the nocutra component: Lepus---
# 📚 Documentation - V-0-0-0

**Time**: 05/09/2026 - 12:58 PM
**Status**: 🌕 Complete

## Summary
This is the official repository and update logs for Lepus, Nocturas desktop shell. This log has started after the shell has had basic functions implemented

## Notice
Because currently this shell is underdeveloped, expect a lot of architectural changes

## Things that were added/changed
```
src/       - 🟢 Added
src-tauri/ - 🟢 Added
logs/      - 🟢 Added
.gitignore - 🟡 Changed
```

---
# ⭐ New Feature - V-0-1-1

**Time**: 06/09/2026 - 07:37 PM
**Status**: 🌗 Partially Complete

## Summary
This minor version is going to revolve around making the features of Lepus activatable through nyx-server. This patch was to make the notification system
activatable through nyx-server

## Notice
As of now, to send a notifcation you have to send the message:
```
{
    "payload": {
        "request": "notify",
        "title": "Enter name of your app",
        "summary": "Summary of msg",
        "Options": [// empty or options]   NOTE: options not supported yet
    }
}
```
NOTE: you can also just use the classic linux D-bus **later** when noctura will support it


## Things that were added/changed
```
notify.vue - 🟡 Changed
notify.css - 🟡 Changed
```
---
# ⭐ New Feature - V-0-1-2

**Time**: 08/09/2026 - 12:44 AM
**Status**: 🌗 Partially Complete

## Summary
Added better launcher animation. 
The context menu can now work properly.
The time on the panel updates

## Notice
Launcher now has `display` set to none when not visible, and set to "flex" when visible.
There is a minor delay when setting launcher to visible

## Things that were added/changed
```
topPanel.vue - 🟡 Changed
shell.vue    - 🟡 Changed
launcher.vue - 🟡 Changed
```
---
# ⭐ New Feature - V-0-1-3

**Time**: 10/09/2026 - 09:57 AM
**Status**: 🌕 Complete

## Summary
Added workspace indication
Fixed context menu and added more features
Added conext menu to launcher
## Notice
There is now a new context block at launcher.vue. That context is for the launcher
The variable `workspaces` Now stores the workspacees available

## Things that were added/changed
```
launcher.vue - 🟡 Changed
topPanel.vue - 🟡 Changed
shell.vue    - 🟡 Changed
```
---
# 📁 Architecture change / ⭐ New Feature - V-0-1-4

**Time**: 12/09/2026 - 03:38 PM
**Status**: 🌕 Complete

## Summary
The background is now transparent and fullscreen
There is also a new OSD system (OSD: The popup that popups up when brightness/volume is changed)

## Notice
The key press system was moved from launcher.vue to App.vue


## Things that were added/changed
```
osd.vue      - 🟢 Added
launcher.vue - 🟡 Changed
App.vue      - 🟡 Changed
```
---
# 📁 Architecture change - V-0-1-5

**Time**: 19/09/2026 - 01:46 PM
**Status**: 🌕 Complete

## Summary
Refractored code so code can be ccompiled

## Notice
Essentially, all the:
`<style src="PATH"></style>`
has been replaced with:
`import "PATH"`

## Things that were added/changed
```
src/components/  -  🟡 Changed

```
---
# 📁 Architecture change - V-0-1-6

**Time**: 09/10/2026 - 01:22 AM
**Status**: 🌘 Detected

## Summary
Restructured frontend into the following 3 parts:
- OSD (things that will be on top rgardless of state)
- MiddleGround (things that will be on screen, but fullscreen can change that)
- BackGround (lowest on the Z-Index)

## Notice
This is a HUGE architectural change. This is how to activate each layer though:
```sh
npx run tauri dev -- -- --(osd/middle/bg)
```

## Things that were added/changed
```
Everything  -  🟡 Changed

```
---
# ⭐ New Feature - V-0-1-7

**Time**: 11/10/2026 - 06:02 PM
**Status**: 🌕 Complete

## Summary
Added background feature 

## Notice
Stores the background our apps data directory
If no background has been preset, we use the strong primary contrast color for
the background

## Things that were added/changed
```
shell.vue (Frontends/Background)  -  🟡 Changed
app.vue (Frontends/Background)    -  🟡 Changed


```
---
# ⭐ New Feature - V-0-1-8

**Time**: 13/10/2026 - 05:26 PM
**Status**: 🌕 Complete

## Summary
Added search feature for the launcher menu

## Notice
To implement the search feature, I used fuse.js
The default context menu is also removed.
The system context menu will be moved to another ground (LayerSurface::Bottom)

## Things that were added/changed
```
App.vue       -  🟡 Changed
Launcher.vue  -  🟡 Changed

```
---
# ⭐ New Feature / 📁 Architecture change - V-0-1-9

**Time**: 15/10/2026 - 05:37 PM
**Status**: 🌗 Partially Complete

## Summary
Added another layer for widgets, and added interconnection between Lepus parts
by making use of nyx-server. 

## Notice
A new frontend layer has been made, widgets, its the LayerSurface::Bottom
Random ports will be selected instead of a predefined one.
Run before command changed to run scripts/dev.mjs
To test system context:

    - Open 3 terminals:
        - on terminal 1, run:
            ```sh
                # change directory to nyx-server
                npx tsx src/server.ts
            ```
        - on terminal 2, run:
            ```sh
                LEPUS_FRONTEND=Background npm run dev
            ```
        - on terminal 3, run:
            ```sh
                LEPUS_FRONTEND=Widgets npm run dev
            ```

## Things that were added/changed
```
frontends/Widgets/            -  🟢 Added
scripts/dev.mjs               -  🟢 Added
frontends/Background/App.vue  -  🟡 Changed

```
---
# ⭐ New Feature - V-0-1-10

**Time**: 19/10/2026 - 01:17 PM
**Status**: 🌗 Partially Complete

## Summary
Added pin and unpin features for apps

## Notice
panel.json location as be shifted from public directory to
app base directory (~/.config/...)
there has been a slight change to the data being stored in panel.json
they dont have the "id" key anymore.

## Things that were added/changed
```
launcher.vue  -  🟡 Changed
panel.vue     -  🟡 Changed
app.vue       -  🟡 Changed

```
---
# ⭐ New Feature / 📁 Architecture change - V-0-1-11

**Time**: 20/10/2026 - 10:20 PM
**Status**: 🌕 Complete

## Summary
Apps can now display their name when sending notifications

## Notice
**⚠️⚠️⚠️ PLEASE NOTICE THESE SIGNIFICANT CHANGES:**
**NYX_NAME OF WIDGET LAYER HAS BEEN CHANGED TO**: Lepus_widget
**NYX_NAME OF MIDDLEGROUND LAYER HAS BEEN CHANGED TO**: Lepus_main
**WHEN SENDING NOTIFCATION, THE NEW STRUCUTRE FOR THE PAYLOAD IS**: 
```
{
    "appName": "NAMEOFAPP",
    "request": "notify",
    "title": "TITLE",
    "subject": "SUBJECT",
    "options": ["Yes", "No"]  // YOU CAN ALSO NOT INCLUDE THIS
}
```

## Things that were added/changed
```
App.vue     -  🟡 Changed
notify.vue  -  🟡 Changed
shell.vue   -  🟡 Changed

```
