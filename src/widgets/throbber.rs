use rand::Rng as _;

/// State to be used for Throbber render.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ThrobberState {
    /// Index of Set.symbols used when Spin is specified for WhichUse.
    ///
    /// If out of range, it is normalized at render time.
    index: i8,
}

impl ThrobberState {
    /// Get a index.
    pub fn index(&self) -> i8 {
        self.index
    }

    /// Increase index.
    ///
    /// # Examples:
    /// ```
    /// let mut throbber_state = throbber_widgets_tui::ThrobberState::default();
    /// assert_eq!(throbber_state.index(), 0);
    /// throbber_state.calc_next();
    /// assert_eq!(throbber_state.index(), 1);
    /// ```
    pub fn calc_next(&mut self) {
        self.calc_step(1);
    }

    /// Calculate the index by specifying step.
    ///
    /// Negative numbers can also be specified for step.
    ///
    /// If step is 0, the index is determined at random.
    ///
    /// # Examples:
    /// ```
    /// let mut throbber_state = throbber_widgets_tui::ThrobberState::default();
    /// assert_eq!(throbber_state.index(), 0);
    /// throbber_state.calc_step(2);
    /// assert_eq!(throbber_state.index(), 2);
    /// throbber_state.calc_step(-3);
    /// assert_eq!(throbber_state.index(), -1);
    /// throbber_state.calc_step(0); // random
    /// assert!((std::i8::MIN..=std::i8::MAX).contains(&throbber_state.index()))
    /// ```
    pub fn calc_step(&mut self, step: i8) {
        self.index = if step == 0 {
            let mut rng = rand::thread_rng();
            rng.gen()
        } else {
            self.index.checked_add(step).unwrap_or(0)
        }
    }

    /// Set the index to the range of throbber_set.symbols.len().
    ///
    /// This is called from render function automatically.
    ///
    /// # Examples:
    /// ```
    /// let mut throbber_state = throbber_widgets_tui::ThrobberState::default();
    /// let throbber = throbber_widgets_tui::Throbber::default();
    /// let len = 6; //throbber.throbber_set.symbols.len() as i8;
    ///
    /// throbber_state.normalize(&throbber);
    /// assert_eq!(throbber_state.index(), 0);
    ///
    /// throbber_state.calc_step(len + 2);
    /// assert_eq!(throbber_state.index(), len + 2);
    /// throbber_state.normalize(&throbber);
    /// assert_eq!(throbber_state.index(), 2);
    ///
    /// // Negative numbers are indexed from backward
    /// throbber_state.calc_step(-3 - len);
    /// assert_eq!(throbber_state.index(), -1 - len);
    /// throbber_state.normalize(&throbber);
    /// assert_eq!(throbber_state.index(), len - 1);
    /// ```
    pub fn normalize(&mut self, throbber: &Throbber) {
        let len = throbber.throbber_set.symbols.len() as i8;
        if len <= 0 {
            //ng but it's not used, so it stays.
        } else {
            self.index %= len;
            if self.index < 0 {
                // Negative numbers are indexed from the tail
                self.index += len;
            }
        }
    }
}

/// A compact widget to display a throbber.
///
/// A throbber may also be called:
/// - activity indicator
/// - indeterminate progress bar
/// - loading icon
/// - spinner
/// - guru guru
///
/// # Examples:
///
/// ```
/// let throbber = throbber_widgets_tui::Throbber::default()
///     .throbber_style(ratatui::style::Style::default().fg(ratatui::style::Color::White).bg(ratatui::style::Color::Black))
///     .label("NOW LOADING...");
/// // frame.render_widget(throbber, chunks[0]);
/// let throbber_state = throbber_widgets_tui::ThrobberState::default();
/// // frame.render_stateful_widget(throbber, chunks[0], &mut throbber_state);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Throbber<'a> {
    label: Option<ratatui::text::Span<'a>>,
    style: ratatui::style::Style,
    throbber_style: ratatui::style::Style,
    throbber_set: crate::symbols::throbber::Set,
    use_type: crate::symbols::throbber::WhichUse,
}

impl<'a> Default for Throbber<'a> {
    fn default() -> Self {
        Self {
            label: None,
            style: ratatui::style::Style::default(),
            throbber_style: ratatui::style::Style::default(),
            throbber_set: crate::symbols::throbber::BRAILLE_SIX,
            use_type: crate::symbols::throbber::WhichUse::Spin,
        }
    }
}

impl<'a> Throbber<'a> {
    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<ratatui::text::Span<'a>>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn style(mut self, style: ratatui::style::Style) -> Self {
        self.style = style;
        self
    }

    pub fn throbber_style(mut self, style: ratatui::style::Style) -> Self {
        self.throbber_style = style;
        self
    }

    pub fn throbber_set(mut self, set: crate::symbols::throbber::Set) -> Self {
        self.throbber_set = set;
        self
    }

    pub fn use_type(mut self, use_type: crate::symbols::throbber::WhichUse) -> Self {
        self.use_type = use_type;
        self
    }

    /// Convert symbol only to Span with state.
    pub fn to_symbol_span(&self, state: &ThrobberState) -> ratatui::text::Span<'a> {
        let symbol = match self.use_type {
            crate::symbols::throbber::WhichUse::Full => self.throbber_set.full,
            crate::symbols::throbber::WhichUse::Empty => self.throbber_set.empty,
            crate::symbols::throbber::WhichUse::Spin => {
                let mut state = state.clone();
                state.normalize(self);
                let len = self.throbber_set.symbols.len() as i8;
                if 0 <= state.index && state.index < len {
                    self.throbber_set.symbols[state.index as usize]
                } else {
                    self.throbber_set.empty
                }
            }
        };
        let symbol_span = ratatui::text::Span::styled(format!("{} ", symbol), self.style)
            .patch_style(self.throbber_style);
        symbol_span
    }

    /// Convert symbol and label to Line with state.
    pub fn to_line(&self, state: &ThrobberState) -> ratatui::text::Line<'a> {
        let mut line = ratatui::text::Line::default().style(self.style);
        line.spans.push(self.to_symbol_span(state));
        if let Some(label) = &self.label.clone() {
            line.spans.push(label.clone());
        }
        line
    }
}

impl<'a> ratatui::widgets::Widget for Throbber<'a> {
    /// Render random step symbols.
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let mut state = ThrobberState::default();
        state.calc_step(0);
        ratatui::widgets::StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl<'a> ratatui::widgets::StatefulWidget for Throbber<'a> {
    type State = ThrobberState;

    /// Render specified index symbols.
    fn render(
        self,
        area: ratatui::layout::Rect,
        buf: &mut ratatui::buffer::Buffer,
        state: &mut Self::State,
    ) {
        buf.set_style(area, self.style);

        let throbber_area = area;
        if throbber_area.height < 1 {
            return;
        }

        // render a symbol.
        let symbol = match self.use_type {
            crate::symbols::throbber::WhichUse::Full => self.throbber_set.full,
            crate::symbols::throbber::WhichUse::Empty => self.throbber_set.empty,
            crate::symbols::throbber::WhichUse::Spin => {
                state.normalize(&self);
                let len = self.throbber_set.symbols.len() as i8;
                if 0 <= state.index && state.index < len {
                    self.throbber_set.symbols[state.index as usize]
                } else {
                    self.throbber_set.empty
                }
            }
        };
        let symbol_span = ratatui::text::Span::styled(format!("{} ", symbol), self.throbber_style);
        let (col, row) = buf.set_span(
            throbber_area.left(),
            throbber_area.top(),
            &symbol_span,
            symbol_span.width() as u16,
        );

        // render a label.
        if let Some(label) = self.label {
            if throbber_area.right() <= col {
                return;
            }
            buf.set_span(col, row, &label, label.width() as u16);
        }
    }
}

/// Convert symbol only to Span without state(mostly random index).
///
/// If you want to specify a state, use `Throbber::to_symbol_span()`.
impl<'a> From<Throbber<'a>> for ratatui::text::Span<'a> {
    fn from(throbber: Throbber<'a>) -> ratatui::text::Span<'a> {
        let mut state = ThrobberState::default();
        state.calc_step(0);
        throbber.to_symbol_span(&state)
    }
}

/// Convert symbol and label to Line without state(mostly random index).
///
/// If you want to specify a state, use `Throbber::to_line()`.
impl<'a> From<Throbber<'a>> for ratatui::text::Line<'a> {
    fn from(throbber: Throbber<'a>) -> ratatui::text::Line<'a> {
        let mut state = ThrobberState::default();
        state.calc_step(0);
        throbber.to_line(&state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn throbber_state_calc_step() {
        let mut throbber_state = ThrobberState::default();
        assert_eq!(throbber_state.index(), 0);

        // random test
        // The probability of failure is not zero, but it is as low as possible.
        let mut difference = false;
        for _ in 0..100 {
            throbber_state.calc_step(0);
            assert!((std::i8::MIN..=std::i8::MAX).contains(&throbber_state.index()));

            if 0 != throbber_state.index() {
                difference = true;
            }
        }
        assert!(difference);
    }

    #[test]
    fn throbber_state_normalize() {
        let mut throbber_state = ThrobberState::default();
        let throbber = Throbber::default();
        let len = throbber.throbber_set.symbols.len() as i8;
        let max = len - 1;

        // check upper
        throbber_state.calc_step(max);
        throbber_state.normalize(&throbber);
        assert_eq!(throbber_state.index(), max);

        // check overflow
        throbber_state.calc_next();
        throbber_state.normalize(&throbber);
        assert_eq!(throbber_state.index(), 0);

        // check underflow
        throbber_state.calc_step(-1);
        throbber_state.normalize(&throbber);
        assert_eq!(throbber_state.index(), max);

        // check negative out of range
        throbber_state.calc_step(len * -2);
        throbber_state.normalize(&throbber);
        assert_eq!(throbber_state.index(), max);
    }

    #[test]
    fn throbber_converts_to_span() {
        let throbber = Throbber::default().use_type(crate::symbols::throbber::WhichUse::Full);
        let span: ratatui::text::Span = throbber.into();
        assert_eq!(span.content, "⠿ ");
    }

    #[test]
    fn throbber_converts_to_line() {
        let throbber = Throbber::default().use_type(crate::symbols::throbber::WhichUse::Full);
        let line: ratatui::text::Line = throbber.into();
        assert_eq!(line.spans[0].content, "⠿ ");
    }

    #[test]
    fn throbber_reaches_upper_limit_step_resets_to_zero() {
        let mut throbber_state = ThrobberState::default();

        for _ in 0..i8::MAX {
            throbber_state.calc_next();
        }
        throbber_state.calc_next();
        assert!(throbber_state.index() != i8::MAX);
    }
}
