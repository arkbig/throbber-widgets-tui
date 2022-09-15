use rand::Rng as _;

#[derive(Debug, Clone, Default)]
pub struct ThrobberState {
    index: i8,
}

impl ThrobberState {
    pub fn index(&self) -> i8 {
        self.index
    }

    pub fn calc_next(&mut self) {
        self.calc_step(1);
    }

    pub fn calc_step(&mut self, step: i8) {
        self.index = if step == 0 {
            let mut rng = rand::thread_rng();
            rng.gen_range(0..std::i8::MAX)
        } else {
            self.index + step
        }
    }

    pub fn normalize(&mut self, throbber: &Throbber) {
        let len = throbber.throbber_set.symbols.len() as i8;
        if 0 <= self.index && self.index < len {
            //ok
        } else if len <= 0 {
            //ng
        } else if self.index < 0 {
            self.index = -self.index % len;
        } else {
            self.index = self.index % len;
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
/// Throbber::default()
///     .throbber_style(Style::default().fg(Color::White).bg(Color::Black))
///     .throbber_set(symbols::throbber::BRAILLE_SIX)
///     .label("NOW LOADING...");
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
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let mut state = ThrobberState::default();
        state.calc_step(0);
        tui::widgets::StatefulWidget::render(self, area, buf, &mut state);
    }
}

impl<'a> tui::widgets::StatefulWidget for Throbber<'a> {
    type State = ThrobberState;

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
