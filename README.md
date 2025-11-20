# Playing with UI crates in Rust

## Why?
- Cross-platform "web" solutions are terrible.
- I desire some simplicity but don't want a simple app being 500 MB and use GBs of RAM.
- Would be nice to run natively on Mac, Linux, and (maybe) Windows.
- Bonus if I can run on web (WASM?)


## Bevy
Last time I looked at Bevy was 0.13.1. Let's see how much it has changed / evolved.

Conclusions:
- Love the concept of this crate.
- Probably coming along nicely for game experimentation
- Not really useful for building cross-platform UI apps.


## GPUI
New to me. Using 0.2.2.

Conclusions:
- Library is way too early for use.
- Clunky and "web-like". Not a fan of the "div" idea.
- Having to build your own basic components is clunky.
- Tried 'gpui-component' crate, but didn't like it.
- Not a fan of the styling mechanic.
- Not really useful at the moment.


## Iced
New to me. Using forked 'master' (0.14.0-dev) branch to experiment.

Conclusions:
- Pattern is easy to understand.
- Still a lot of work for even basic functionality.
- I like the possible patterns for components (simple free functions, viewable pattern, full widget).
- The data model is quite clunky. Still experimenting with how to integrate a "data store" idea.

