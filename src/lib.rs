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
//! - Random or specified step, also negative is possible.
//!
//! ## Getting Started
//!
//! ```sh
//! cargo add throbber-widgets-tui
//! ```
//!
//! ```rust
//! // :
//! // :
//! struct App {
//!     throbber_state: throbber_widgets_tui::ThrobberState,
//! }
//! impl App {
//!     fn on_tick(&mut self) {
//!         self.throbber_state.calc_next();
//!     }
//! }
//! // :
//! // :
//! fn ui<B: tui::backend::Backend>(f: &mut tui::Frame<B>, app: &mut App) {
//!     let chunks = tui::layout::Layout::default()
//!         .direction(tui::layout::Direction::Horizontal)
//!         .margin(1)
//!         .constraints(
//!             [
//!                 tui::layout::Constraint::Percentage(50),
//!                 tui::layout::Constraint::Percentage(50),
//!             ]
//!             .as_ref(),
//!         )
//!         .split(f.size());
//!
//!     // Simple random step
//!     let simple = throbber_widgets_tui::Throbber::default();
//!     f.render_widget(simple, chunks[0]);
//!
//!     // Set full with state
//!     let full = throbber_widgets_tui::Throbber::default()
//!         .label("Running...")
//!         .style(tui::style::Style::default().fg(tui::style::Color::Cyan))
//!         .throbber_style(tui::style::Style::default().fg(tui::style::Color::Red).add_modifier(tui::style::Modifier::BOLD))
//!         .throbber_set(throbber_widgets_tui::CLOCK)
//!         .use_type(throbber_widgets_tui::WhichUse::Spin);
//!     f.render_stateful_widget(full, chunks[1], &mut app.throbber_state);
//! }
//! ```
//!
//! ## Apps using throbber-widgets-tui
//!
//! - [mntime](https://github.com/arkbig/mntime): Execute "m" commands "n" times to calculate mean of usage time and memory.  As an alternative to "time", "gnu-time" is used internally.
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
