# `re-posel`

**Reverse engineering Posel Smrti/The Black Mirror (2003)**

## About

This repository contains tools for reverse engineering and analysing the game [**The Black Mirror**](https://en.wikipedia.org/wiki/The_Black_Mirror_(video_game)), first released as **Posel Smrti** in Czechia in 2003. The bulk of the work was done by analysing the `agds.exe` binary in [Ghidra](https://github.com/NationalSecurityAgency/ghidra/) and by analysing the datafiles packaged with the game. The latter are in a bespoke format (with light XOR-based "encryption") and also contain bytecode.

## Features

The [`analyser`](./analyser) tool, implemented in Rust, can:

- extract assets from `*.grp` files: these are simply big archive formats with no compression;
- extract *objects* (strings, dialogue scripts, references to assets, screen regions, bytecode scripts) from `*.adb` files;
- analyse and visualise objects: references to objects and assets are resolved, bytecode is decompiled into readable script;
- patch `*.adb` files to fix or modify game behaviour.

The tool requires you to provide paths to the original data files.

## Patches

By default, the `patch` command will apply all of the following patches:

- `chapter_select`: allows the player to choose a starting chapter (I. - VI.) when starting a new game;
- `check_again`: removes dialogue paths where the same person has to be asked multiple times before the game progresses (currently only done for dialogue with Harry);
- `skip_intros`: skips intro logos and menu animation on launch, quits faster.

## Goals

The main reason I am doing this is because I enjoy reverse engineering, and this game is close to my heart.

Other than that, some goals I hope to achieve are:

- patch game to avoid wasting player's time
  - [x] skip game intros
  - [x] re-enable chapter selection
  - ask a person three times before plot is advanced:
    - [x] Harry, waiting for Mark
    - waiting for Frederick... (extra annoying because of horizontal scroll?)
    - ...?
  - [ ] avoid Sierra-style deaths and unwinnable game states
  - [ ] double click to instantly move, even across horizontal scroll screens
  - [ ] unskippable animations before dialogue is started
- find optimal routes in the game for speedrunning (look at that [thriving community](https://www.speedrun.com/black_mirror/resources))
  - create Autosplits https://github.com/LiveSplit/LiveSplit.AutoSplitters
- recreate game engine in Rust
  - run natively on more platforms
  - higher resolutions (scaling filter...?)
  - allow mixed language audio and subtitles
- make video content about the game
  - find cut content (e.g. VA instructions in dialogue scripts)
  - explain how game works
  - find differences between English and Czech versions
