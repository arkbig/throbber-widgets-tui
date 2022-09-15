# Throbber widget of [tui-rs]

[tui-rs]: https://github.com/fdehau/tui-rs

> **_NOTE:_** I'm a newbie Rust. Therefore, there may be breaking changes in the future.

`throbber-widgets-tui` is a [tui-rs] widget that displays throbber.

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
- Random or specified step

## Getting Started

```sh
cargo add --git https://github.com/arkbig/throbber-widgets-tui
```

```rust
:
impl App {
    fn on_tick(&mut self) {
        throbber_state.calc_next();
    }
}
:
fn ui<B: Backend>(f: &mut Frame<B>, cursor_y: &mut u16, app: &mut App) {
    :
    // Simple random step
    let simple = throbber_widgets_tui::Throbber::default();
    f.render_widget(simple, chunks[0]);

    // Set full with state
    let full = throbber_widgets_tui::Throbber::default()
        .label("Running...")
        .style(Style::default().fg(Color::Cyan))
        .throbber_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .throbber_set(throbber_widgets_tui::CLOCK)
        .use_type(throbber_widgets_tui::WhichUse::Spin);
    f.render_stateful_widget(full, chunks[1], &mut app.throbber_state);
}
```

## Dependencies

Direct dependencies crates:

| License           | crate |
|-------------------|-------|
| Apache-2.0 OR MIT | rand  |
| MIT               | tui   |

All dependencies:

> $ cargo license   
> Apache-2.0 OR Apache-2.0 WITH LLVM-exception OR MIT (1): wasi
> Apache-2.0 OR MIT (30): autocfg, bitflags, cassowary, cfg-if, getrandom, libc, lock_api, log, parking_lot, parking_lot_core, ppv-lite86, rand, rand_chacha, rand_core, scopeguard, signal-hook, signal-hook-mio, signal-hook-registry, smallvec, unicode-segmentation, unicode-width, winapi, winapi-i686-pc-windows-gnu, winapi-x86_64-pc-windows-gnu, windows-sys, windows_aarch64_msvc, windows_i686_gnu, windows_i686_msvc, windows_x86_64_gnu, windows_x86_64_msvc
> MIT (5): crossterm, crossterm_winapi, mio, redox_syscall, tui
> Zlib (1): throbber-widgets-tui

## License

This repository's license is zlib. Please feel free to use this, but no warranty.
