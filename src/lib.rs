//! # Throbber widget of [tui-rs]
//!
//! [tui-rs]: https://github.com/fdehau/tui-rs
//!
//! `throbber-widgets-tui` is a [tui-rs] widget that displays throbber.
//!
//! A throbber may also be called:
//!
//! - activity indicator
//! - indeterminate progress bar
//! - loading icon
//! - spinner
//! - guru guru
//!
//! ## Demo
//!
//! ![Demo Animation](https://raw.githubusercontent.com/arkbig/throbber-widgets-tui/main/examples/demo.gif)
//!
//! The demo shown in the gif can be run with all available symbols.
//!
//! ```sh
//! cargo run --example demo --release
//! ```
//!
//! ## Features
//!
//! - Render throbber
//! - With label
//! - Random or specified step
//!
//! ## Getting Started
//!
//! ```sh
//! cargo add --git https://github.com/arkbig/throbber-widgets-tui
//! ```
//!
//! ```rust
//! :
//! impl App {
//!     fn on_tick(&mut self) {
//!         throbber_state.calc_next();
//!     }
//! }
//! :
//! fn ui<B: Backend>(f: &mut Frame<B>, cursor_y: &mut u16, app: &mut App) {
//!     :
//!     // Simple random step
//!     let simple = throbber_widgets_tui::Throbber::default();
//!     f.render_widget(simple, chunks[0]);
//!
//!     // Set full with state
//!     let full = throbber_widgets_tui::Throbber::default()
//!         .label("Running...")
//!         .style(Style::default().fg(Color::Cyan))
//!         .throbber_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
//!         .throbber_set(throbber_widgets_tui::CLOCK)
//!         .use_type(throbber_widgets_tui::WhichUse::Spin);
//!     f.render_stateful_widget(full, chunks[1], &mut app.throbber_state);
//! }
//! ```
//!
//! ## Dependencies
//!
//! Direct dependencies crates:
//!
//! | License           | crate |
//! |-------------------|-------|
//! | Apache-2.0 OR MIT | rand  |
//! | MIT               | tui   |
//!
//! ## License
//!
//! This repository's license is zlib. Please feel free to use this, but no warranty.

pub mod symbols;
pub mod widgets;

pub use self::symbols::throbber::*;
pub use self::widgets::*;
