# Horizon Essentials (native mod)

A native Horizon mod written in Rust. Original code; not affiliated with any
other mod. The Ferrite version is dead (that project's repository is gone),
so this uses Horizon's official native hook points.

## What it does

- **HUD** in a screen corner: title, XYZ, friend count, first five friends.
- **/friend** `list | add <name> | remove <name> | address <ip:port> | invite <name>`
  Friends are saved as a mod setting. `invite` prints a line for you to copy,
  because native mods can't send chat as the player.
- **/essentials** `hud` toggles the panel, `corner` cycles its corner.
- **Options > Mods > Horizon Essentials**: buttons for HUD on/off and corner.

## What the hook points don't allow

Custom screens or menus, sending chat or commands as the player, reading
health or food, screenshots, cosmetics. Essential's menu UI can't be copied.

## Status

- `runtime.rs` (all the logic) has 8 passing unit tests: `cargo test`.
- The hook files (`hud.rs`, `command.rs`, ...) were written from Horizon's
  docs without being able to compile them. The docs left some argument types
  unstated (colour types, what `corners.place` returns, `NativeMod.keys`).
  The GitHub check (`.github/workflows/check.yml`) compiles them against
  Horizon's real source and will show any mismatch.
- The revisions in `compatibility.json` come from Horizon's docs example and
  may be out of date. If the check complains, copy the pair from
  `native/fps-overlay/compatibility.json` in horizon-rust/horizon-mods.

## Build and run

Horizon's launcher builds native mods: put this folder in `horizon-native-mods/`
in your game folder, open the launcher's Mods page, press Build on
"Native build" and confirm "I trust these mods: build". The first build takes
20-30 minutes and downloads Rust and Horizon's source.
