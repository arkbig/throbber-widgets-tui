use rand::Rng as _;
use ratatui as tui;

/// State to be used for Throbber render.
#[derive(Debug, Clone, Default)]
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
            self.index.saturating_add(step)
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
///     .throbber_style(tui::style::Style::default().fg(tui::style::Color::White).bg(tui::style::Color::Black))
///     .label("NOW LOADING...");
/// // frame.render_widget(throbber, chunks[0]);
/// let throbber_state = throbber_widgets_tui::ThrobberState::default();
/// // frame.render_stateful_widget(throbber, chunks[0], &mut throbber_state);
/// ```
#[derive(Debug, Clone)]
pub struct Throbber<'a> {
    label: Option<tui::text::Span<'a>>,
    style: tui::style::Style,
    throbber_style: tui::style::Style,
    throbber_set: crate::symbols::throbber::Set,
    use_type: crate::symbols::throbber::WhichUse,
}

impl<'a> Default for Throbber<'a> {
    fn default() -> Self {
        Self {
            label: None,
            style: tui::style::Style::default(),
            throbber_style: tui::style::Style::default(),
            throbber_set: crate::symbols::throbber::BRAILLE_SIX,
            use_type: crate::symbols::throbber::WhichUse::Spin,
        }
    }
}

impl<'a> Throbber<'a> {
    pub fn label<T>(mut self, label: T) -> Self
    where
        T: Into<tui::text::Span<'a>>,
    {
        self.label = Some(label.into());
        self
    }

    pub fn style(mut self, style: tui::style::Style) -> Self {
        self.style = style;
        self
    }

    pub fn throbber_style(mut self, style: tui::style::Style) -> Self {
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
}

impl<'a> tui::widgets::Widget for Throbber<'a> {
    /// Render random step symbols.
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let mut state = ThrobberState::default();
        state.calc_step(0);
        tui::widgets::StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl<'a> tui::widgets::StatefulWidget for Throbber<'a> {
    type State = ThrobberState;

    /// Render specified index symbols.
    fn render(
        self,
        area: tui::layout::Rect,
        buf: &mut tui::buffer::Buffer,
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
        let symbol_span = tui::text::Span::styled(format!("{} ", symbol), self.throbber_style);
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
}
