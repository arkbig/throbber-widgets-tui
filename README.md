# Throbber widget of [ratatui]

[ratatui]: https://github.com/ratatui-org/ratatui
[tui-rs]: https://github.com/fdehau/tui-rs

> **_NOTE:_** This crate was originally created as a widget for [tui-rs], so you can see `tui::` on the sample code, but (by default) [ratatui] is used, so please replace it accordingly.

`throbber-widgets-tui` is a [ratatui] (or [tui-rs]) widget that displays throbber.

A throbber may also be called:

- activity indicator
- indeterminate progress bar
- loading icon
- spinner
- ぐるぐる(guru guru)

## Demo

![Demo Animation](./examples/demo.gif)

The demo shown in the gif can be run with all available symbols.

```sh
cargo run --example demo --release
```

## Features

- Render throbber
- With label
- Random or specified step, also negative is possible.

## Getting Started

MSRV: `throbber-widgets-tui` requires rustc 1.56.1 or newer.

```sh
cargo add throbber-widgets-tui
```

Default is [ratatui] as tui.

<details>
<summary>OR use [tui-rs] instead of [ratatui]. use `tui` features like clicked here.</summary>

```sh
cargo add throbber-widgets-tui --no-default-features --features tui
```

</details>

Example code:

```rust
#[cfg(feature = "ratatui")]
use ratatui as tui;
// :
// :
struct App {
    throbber_state: throbber_widgets_tui::ThrobberState,
}
impl App {
    fn on_tick(&mut self) {
        self.throbber_state.calc_next();
    }
}
// :
// :
fn ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>, app: &mut App) {
    let chunks = tui::layout::Layout::default()
        .direction(tui::layout::Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                tui::layout::Constraint::Percentage(50),
                tui::layout::Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Simple random step
    let simple = throbber_widgets_tui::Throbber::default();
    f.render_widget(simple, chunks[0]);

    // Set full with state
    let full = throbber_widgets_tui::Throbber::default()
        .label("Running...")
        .style(tui::style::Style::default().fg(tui::style::Color::Cyan))
        .throbber_style(tui::style::Style::default().fg(tui::style::Color::Red).add_modifier(tui::style::Modifier::BOLD))
        .throbber_set(throbber_widgets_tui::CLOCK)
        .use_type(throbber_widgets_tui::WhichUse::Spin);
    f.render_stateful_widget(full, chunks[1], &mut app.throbber_state);
}
```

## Apps using throbber-widgets-tui

- [mntime](https://github.com/arkbig/mntime): Execute "m" commands "n" times to calculate mean of usage time and memory.  As an alternative to "time", "gnu-time" is used internally.

## Dependencies (By default)

Direct dependencies crates:

```sh
cargo license --direct-deps-only --avoid-build-deps --avoid-dev-deps | awk -F ":" 'BEGIN {printf "|License|crate|\n|-|-|\n"} {printf "|%s|%s|\n", $1, $2}'
```

|License|crate|
|-|-|
|Apache-2.0 OR MIT (1)| rand|
|MIT (2)| crossterm, ratatui|
|Zlib (1)| throbber-widgets-tui|

Chain dependencies crates:

```sh
cargo license --avoid-build-deps --avoid-dev-deps | awk -F ":" 'BEGIN {printf "|License|crate|\n|-|-|\n"} {printf "|%s|%s|\n", $1, $2}'
```

|License|crate|
|-|-|
|Apache-2.0 OR Apache-2.0 WITH LLVM-exception OR MIT (1)| wasi|
|Apache-2.0 OR MIT (35)| bitflags, bitflags, cassowary, cfg-if, getrandom, indoc, libc, lock_api, log, parking_lot, parking_lot_core, paste, ppv-lite86, rand, rand_chacha, rand_core, scopeguard, signal-hook, signal-hook-mio, signal-hook-registry, smallvec, unicode-segmentation, unicode-width, winapi, winapi-i686-pc-windows-gnu, winapi-x86_64-pc-windows-gnu, windows-sys, windows-targets, windows_aarch64_gnullvm, windows_aarch64_msvc, windows_i686_gnu, windows_i686_msvc, windows_x86_64_gnu, windows_x86_64_gnullvm, windows_x86_64_msvc|
|MIT (5)| crossterm, crossterm_winapi, mio, ratatui, redox_syscall|
|Zlib (1)| throbber-widgets-tui|

## License

This repository's license is zlib. Please feel free to use this, but no warranty.
