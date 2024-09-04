#[derive(Default)]
struct App {
    states: Vec<throbber_widgets_tui::ThrobberState>,
}

impl App {
    fn on_tick(&mut self) {
        self.states.iter_mut().for_each(|state| state.calc_next());
    }
}

fn main() -> std::io::Result<()> {
    // setup terminal
    let mut terminal = ratatui::init();

    // create app and run it
    let tick_rate = std::time::Duration::from_millis(250);
    let app = App::default();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    ratatui::restore();

    res
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut ratatui::Terminal<B>,
    mut app: App,
    tick_rate: std::time::Duration,
) -> std::io::Result<()> {
    let mut last_tick = std::time::Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));
        if ratatui::crossterm::event::poll(timeout)? {
            if let ratatui::crossterm::event::Event::Key(key) = ratatui::crossterm::event::read()? {
                if let ratatui::crossterm::event::KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = std::time::Instant::now();
        }
    }
}
fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let all_sets = [
        ("ASCII", throbber_widgets_tui::ASCII),
        ("BOX_DRAWING", throbber_widgets_tui::BOX_DRAWING),
        ("ARROW", throbber_widgets_tui::ARROW),
        ("DOUBLE_ARROW", throbber_widgets_tui::DOUBLE_ARROW),
        ("VERTICAL_BLOCK", throbber_widgets_tui::VERTICAL_BLOCK),
        ("HORIZONTAL_BLOCK", throbber_widgets_tui::HORIZONTAL_BLOCK),
        ("QUADRANT_BLOCK", throbber_widgets_tui::QUADRANT_BLOCK),
        (
            "QUADRANT_BLOCK_CRACK",
            throbber_widgets_tui::QUADRANT_BLOCK_CRACK,
        ),
        ("WHITE_SQUARE", throbber_widgets_tui::WHITE_SQUARE),
        ("WHITE_CIRCLE", throbber_widgets_tui::WHITE_CIRCLE),
        ("BLACK_CIRCLE", throbber_widgets_tui::BLACK_CIRCLE),
        ("CLOCK", throbber_widgets_tui::CLOCK),
        ("BRAILLE_ONE", throbber_widgets_tui::BRAILLE_ONE),
        ("BRAILLE_SIX", throbber_widgets_tui::BRAILLE_SIX),
        ("BRAILLE_EIGHT", throbber_widgets_tui::BRAILLE_EIGHT),
        ("BRAILLE_DOUBLE", throbber_widgets_tui::BRAILLE_DOUBLE),
        (
            "BRAILLE_SIX_DOUBLE",
            throbber_widgets_tui::BRAILLE_SIX_DOUBLE,
        ),
        (
            "BRAILLE_EIGHT_DOUBLE",
            throbber_widgets_tui::BRAILLE_EIGHT_DOUBLE,
        ),
        ("OGHAM_A", throbber_widgets_tui::OGHAM_A),
        ("OGHAM_B", throbber_widgets_tui::OGHAM_B),
        ("OGHAM_C", throbber_widgets_tui::OGHAM_C),
        ("PARENTHESIS", throbber_widgets_tui::PARENTHESIS),
        ("CANADIAN", throbber_widgets_tui::CANADIAN),
    ];
    let horizontal_num = 4;
    // why +1? because the first line is for title default throbber.
    let vertical_num = 1 + (all_sets.len() + horizontal_num - 1) / horizontal_num;
    let verticals = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints(&vec![ratatui::layout::Constraint::Length(1); vertical_num])
        .split(f.area());
    let default_throbber = throbber_widgets_tui::Throbber::default()
        .label("Press q to exit. This line is a default throbber (random step). The followings are incremental step.")
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));
    f.render_widget(default_throbber, verticals[0]);

    let mut chunks: Option<_> = None;
    for (i, kvp) in all_sets.iter().enumerate() {
        let (name, set) = kvp;
        let row = i / horizontal_num;
        let col = i % horizontal_num;
        if col == 0 {
            chunks = Some(
                ratatui::layout::Layout::default()
                    .direction(ratatui::layout::Direction::Horizontal)
                    .constraints(&vec![
                        ratatui::layout::Constraint::Percentage(
                            100 / (horizontal_num as u16)
                        );
                        horizontal_num
                    ])
                    .split(verticals[row + 1]),
            );
        }
        if app.states.len() <= i {
            app.states
                .push(throbber_widgets_tui::ThrobberState::default());
        }
        let throbber = throbber_widgets_tui::Throbber::default()
            .label(name.to_string())
            .throbber_set(set.clone());
        f.render_stateful_widget(throbber, chunks.clone().unwrap()[col], &mut app.states[i]);
    }
}
