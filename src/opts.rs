use clap::Clap;

#[derive(Clap, Debug)]
pub struct Opts {
    /// Search query
    pub query: String,

    /// Case sensitive
    #[clap(short = 'i')]
    pub case_insensitive: bool,

    /// Search as text
    #[clap(short = 'a', alias = "a")]
    pub text: bool,

    /// Match whole words (implies -a)
    #[clap(short)]
    pub words: bool,

    /// Show context
    #[clap(short = 'C')]
    pub context: bool,

    /// Lanuage filter
    #[clap(short)]
    pub lang: Vec<String>,

    /// Repository filter
    #[clap(short)]
    pub repo: Option<String>,

    /// Path filter
    #[clap(short = 'P')]
    pub path: Option<String>,

    /// Color output
    #[clap(short, arg_enum, default_value = "auto")]
    pub color: Color,

    /// Disable line numbers
    #[clap(short = 'N')]
    pub no_line_numbers: bool,

    /// Show available filters
    #[clap(short = 'f')]
    pub show_filters: bool,

    /// How many pages to show (0 means all)
    #[clap(short, default_value = "5")]
    pub pages: i64,
}

#[derive(Clap, PartialEq, Debug)]
pub enum Color {
    #[clap(alias = "A")]
    Auto,
    #[clap(alias = "a")]
    Always,
    #[clap(alias = "n")]
    Never,
}
