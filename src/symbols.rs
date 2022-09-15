pub mod throbber {
    /// A set of symbols to be rendered by throbber.
    #[derive(Debug, Clone)]
    pub struct Set {
        pub full: &'static str,
        pub empty: &'static str,
        pub symbols: &'static [&'static str],
    }

    /// Rendering object.
    ///
    /// If Spin is specified, ThrobberState.index is used.
    #[derive(Debug, Clone)]
    pub enum WhichUse {
        Full,
        Empty,
        Spin,
    }

    /// ["|", "/", "-", "\\"]
    pub const ASCII: Set = Set {
        full: "*",
        empty: " ",
        symbols: &["|", "/", "-", "\\"],
    };

    /// ["│", "╱", "─", "╲"]
    pub const BOX_DRAWING: Set = Set {
        full: "┼",
        empty: "　",
        symbols: &["│", "╱", "─", "╲"],
    };

    /// ["↑", "↗", "→", "↘", "↓", "↙", "←", "↖"]
    pub const ARROW: Set = Set {
        full: "↔",
        empty: "　",
        symbols: &["↑", "↗", "→", "↘", "↓", "↙", "←", "↖"],
    };

    /// ["⇑", "⇗", "⇒", "⇘", "⇓", "⇙", "⇐", "⇖"]
    pub const DOUBLE_ARROW: Set = Set {
        full: "⇔",
        empty: "　",
        symbols: &["⇑", "⇗", "⇒", "⇘", "⇓", "⇙", "⇐", "⇖"],
    };

    /// ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"]
    pub const VERTICAL_BLOCK: Set = Set {
        full: "█",
        empty: "　",
        symbols: &["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"],
    };

    /// ["▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"]
    pub const HORIZONTAL_BLOCK: Set = Set {
        full: "█",
        empty: "　",
        symbols: &["▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"],
    };

    /// ["▝", "▗", "▖", "▘"]
    pub const QUADRANT_BLOCK: Set = Set {
        full: "█",
        empty: "　",
        symbols: &["▝", "▗", "▖", "▘"],
    };

    /// ["▙", "▛", "▜", "▟"]
    pub const QUADRANT_BLOCK_CRACK: Set = Set {
        full: "█",
        empty: "　",
        symbols: &["▙", "▛", "▜", "▟"],
    };

    /// ["◳", "◲", "◱", "◰"]
    pub const WHITE_SQUARE: Set = Set {
        full: "⊞",
        empty: "　",
        symbols: &["◳", "◲", "◱", "◰"],
    };

    /// ["◷", "◶", "◵", "◴"]
    pub const WHITE_CIRCLE: Set = Set {
        full: "⊕",
        empty: "　",
        symbols: &["◷", "◶", "◵", "◴"],
    };

    /// ["◑", "◒", "◐", "◓"]
    pub const BLACK_CIRCLE: Set = Set {
        full: "●",
        empty: "　",
        symbols: &["◑", "◒", "◐", "◓"],
    };

    /// ["🕛", "🕧", "🕐", "🕜", "🕑", ..., "🕚", "🕦"]
    pub const CLOCK: Set = Set {
        full: "🕛",
        empty: "　",
        symbols: &[
            "🕛", "🕧", "🕐", "🕜", "🕑", "🕝", "🕒", "🕞", "🕓", "🕟", "🕔", "🕠", "🕕", "🕡",
            "🕖", "🕢", "🕗", "🕣", "🕘", "🕤", "🕙", "🕥", "🕚", "🕦",
        ],
    };

    /// ["⠈", "⠐", "⠠", "⠄", "⠂", "⠁"]
    pub const BRAILLE_ONE: Set = Set {
        full: "⠿",
        empty: "　",
        symbols: &["⠈", "⠐", "⠠", "⠄", "⠂", "⠁"],
    };

    /// ["⠷", "⠯", "⠟", "⠻", "⠽", "⠾"]
    pub const BRAILLE_SIX: Set = Set {
        full: "⠿",
        empty: "　",
        symbols: &["⠷", "⠯", "⠟", "⠻", "⠽", "⠾"],
    };

    /// ["⣷", "⣯", "⣟", "⡿", "⢿", "⣻", "⣽", "⣾"]
    pub const BRAILLE_EIGHT: Set = Set {
        full: "⣿",
        empty: "　",
        symbols: &["⣷", "⣯", "⣟", "⡿", "⢿", "⣻", "⣽", "⣾"],
    };

    /// [" ", "ᚐ", "ᚑ", "ᚒ", "ᚓ", "ᚔ"]
    pub const OGHAM_A: Set = Set {
        full: "ᚔ",
        empty: "　",
        symbols: &[" ", "ᚐ", "ᚑ", "ᚒ", "ᚓ", "ᚔ"],
    };

    /// [" ", "ᚁ", "ᚂ", "ᚃ", "ᚄ", "ᚅ"]
    pub const OGHAM_B: Set = Set {
        full: "ᚅ",
        empty: "　",
        symbols: &[" ", "ᚁ", "ᚂ", "ᚃ", "ᚄ", "ᚅ"],
    };

    /// [" ", "ᚆ", "ᚇ", "ᚈ", "ᚉ", "ᚊ"]
    pub const OGHAM_C: Set = Set {
        full: "ᚊ",
        empty: "　",
        symbols: &[" ", "ᚆ", "ᚇ", "ᚈ", "ᚉ", "ᚊ"],
    };
}
