# [Boytacean](https://boytacean.joao.me)

A Game Boy emulator that is written in Rust 🦀.

<a href="https://boytacean.joao.me" target="_blank"><img src="https://github.com/joamag/boytacean/raw/master/res/videos/002-mario.low.gif" width="360" /></a>

**This emulator has been written for educational purposes and shouldn't be taken too seriously.** But yeahh it plays most Game Boy games, which is cool... 🕹️

## Features

* Game Boy (DMG) and Game Boy Color (CGB) emulation
* Simple navigable source-code
* Web and SDL front-ends
* Audio, with a pretty accurate APU
* Serial Data Transfer (Link Cable) support
* Game Boy Printer emulation
* Support for multiple MBCs: MBC1, MBC2, MBC3, and MBC5
* Variable CPU clock speed
* Accurate PPU - passes [dmg-acid2](https://github.com/mattcurrie/dmg-acid2) tests

For the Web front-end...

* Mobile first experience
* Transparent RAM saving using [Web Storage API](https://developer.mozilla.org/docs/Web/API/Window/localStorage)
* GamePad support using [Web Gamepad API](https://developer.mozilla.org/docs/Web/API/Gamepad_API)
* Unobstructive and effective on-screen GamePad
* Cool bespoke display palettes built by [TheWolfBunny64](https://www.deviantart.com/thewolfbunny).
* Fullscreen browser mode
* Debug mode - VRAM and registers

What's missing...

* Game Boy Color (GBC) emulation

## Deployments

| Provider  | Stable  | URL                                                              |
| --------- | ------- | ---------------------------------------------------------------- |
| Cloudfare | `True`  | [boytacean.joao.me](https://boytacean.joao.me)                   |
| Cloudfare | `True`  | [boytacean.pages.dev](https://boytacean.pages.dev)               |
| Cloudfare | `True`  | [prod.boytacean.pages.dev](https://prod.boytacean.pages.dev)     |
| Cloudfare | `True`  | [stable.boytacean.pages.dev](https://stable.boytacean.pages.dev) |
| Cloudfare | `False` | [master.boytacean.pages.dev](https://master.boytacean.pages.dev) |

## Build

### Library

```bash
cargo build
```

### WASM for Node.js

```bash
cargo install wasm-pack
wasm-pack build --release --target=nodejs -- --features wasm
```

### WASM for Web

```bash
cargo install wasm-pack
wasm-pack build --release --target=web --out-dir=frontends/web/lib -- --features wasm
cd frontends/web
npm install && npm run build
cd dist && python3 -m http.server
```

## Web front-end

The Web front-end makes use of [EmuKit](https://github.com/joamag/emukit) which is a UI toolkit that provides the required infrastructure for the creation
of interfaces for emulation in a Web context.

### Configuration

You can use some GET parameters to control the initial behavior of the emulator.

| Parameter    | Type    | Description                                                                                    |
| ------------ | ------- | ---------------------------------------------------------------------------------------------- |
| `rom_url`    | String  | The URL from which the initial ROM is going to be loaded, should support CORS.                 |
| `url`        | String  | The same as `rom_url`.                                                                         |
| `fullscreen` | Boolean | If the emulator should start in fullscreen mode.                                               |
| `fs`         | Boolean | The same as `fullscreen`.                                                                      |
| `debug`      | Boolean | If the "debugger" should start visible.                                                        |
| `keyboard`   | Boolean | If the on-screen keyboard should start visible.                                                |
| `palette`    | String  | The name of the palette to be set at startup( eg: `christmas`, `hogwards`, `mariobros`, etc.). |

## Palettes

The palettes offered in the web version were provided by [TheWolfBunny64](https://www.deviantart.com/thewolfbunny).

## Inspiration

To get some information about the resources that inspired me through the emulation creation journey check [Inspiration](doc/inspiration.md).

## License

Boytacean is currently licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/).

## Build Automation

[![Build Status](https://github.com/joamag/boytacean/workflows/Main%20Workflow/badge.svg)](https://github.com/joamag/boytacean/actions)
[![crates Status](https://img.shields.io/crates/v/boytacean)](https://crates.io/crates/boytacean)
[![npm Status](https://img.shields.io/npm/v/boytacean.svg)](https://www.npmjs.com/package/boytacean)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/)
