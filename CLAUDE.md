# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

A small dashboard that uses the Captain of Industry web API. It is a Rust app built with **Dioxus 0.7** (with the router feature) and targets the web by default.

## Dioxus reference: read AGENTS.md first

**Before writing or changing any Dioxus code, read [AGENTS.md](AGENTS.md).** It is the reference for Dioxus 0.7 APIs: RSX syntax, components and props, signals, memos, context, `use_resource`, routing, assets, and fullstack server functions with hydration.

Dioxus 0.7 changed every API compared with earlier versions. `cx`, `Scope` and `use_state` no longer exist. Don't rely on Dioxus knowledge from before 0.7; if your memory and AGENTS.md disagree, AGENTS.md is right.

## Commands

Install the Dioxus CLI with `curl -sSL http://dioxus.dev/install.sh | sh`. You can also use `cargo binstall dioxus-cli`.

- Dev server with hot reload (web, the default): `dx serve`
- Other platforms: `dx serve --platform desktop` (also `mobile`). These match the Cargo features `web`, `desktop` and `mobile`.
- Type-check: `cargo check`
- Lint: `cargo clippy`
- Format: `cargo fmt`. Use `dx fmt` to format the contents of `rsx!` blocks.
- Tests: `cargo test`, or `cargo test <test_name>` to run one test.

## Architecture

- `src/main.rs` is the entry point and currently holds every component. It calls `dioxus::launch(App)`, and `App` injects the favicon and stylesheet links, then renders `Router::<Route>`.
- Routing uses one `Route` enum that derives `Routable`. Pages are its variants, and they share a `Navbar` layout through `#[layout(Navbar)]`, which renders `Outlet::<Route>`. The `Home` and `Blog` routes come from the starter template and are placeholders to replace with dashboard pages.
- Assets live in `assets/` and are referenced with `asset!("/assets/...")`, using paths from the project root.
- `Dioxus.toml` holds the `dx` CLI config, such as the web page title.
- `clippy.toml` sets `await-holding-invalid-types`, so Clippy flags any signal read or write guard (`GenerationalRef`, `GenerationalRefMut` or `WriteLock`) held across an `.await`. Inside async code (`use_resource`, spawned tasks), clone the value out or drop the guard before awaiting.
