# Development logs
> The chronological order of updates for the nocutra component: Lepus---
# 📚 Documentation - V-1-0-0

**Time**: 05/09/2026 - 12:58 PM
**Status**: 🌕 Complete

## Summary
This is the official repository and update logs for Lepus, Nocturas desktop shell. This log has started after the shell has had basic functions implemented

## Notice
Because currently this shell is underdeveloped, expect a lot of architectural changes

## Things that were added/changed
```
src/  -  🟢 Added
src-tauri/  -  🟢 Added
logs/  -  🟢 Added
.gitignore  -  🟡 Changed
```

---
# ⭐ New Feature - V-1-0-1

**Time**: 06/09/2026 - 07:37 PM
**Status**: 🌗 Partially Complete

## Summary
This minor version is going to revolve around making the features of Lepus activatable through nyx-server. This patch was to make the notification system
activatable through nyx-server

## Notice
As of now, to send a notifcation you have to send the message:
```{
    "payload": {
        "request": "notify",
        "title": "Enter name of your app",
        "summary": "Summary of msg",
        "Options": [// empty or options]   NOTE: options not supported yet
    }
}```
NOTE: you can also just use the classic linux D-bus **later** when noctura will support it


## Things that were added/changed
```
notify.vue  -  🟡 Changed
notify.css  -  🟡 Changed

```
---
# ⭐ New Feature - V-1-0-2

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
topPanel.vue  -  🟡 Changed
shell.vue  -  🟡 Changed
launcher.vue  -  🟡 Changed

```
---
# ⭐ New Feature - V-1-0-3

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
launcher.vue  -  🟡 Changed
topPanel.vue  -  🟡 Changed
shell.vue  -  🟡 Changed

```
