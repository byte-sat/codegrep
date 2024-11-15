use terminfo::{capability as cap, Database};

use super::opts::{Color, Opts};

const NOCOLOR_TERMS: &[&str] = &["dumb"];

pub struct Colors {
    pub file: String,
    pub line_no: String,
    pub pmatch: String,
    pub reset: String,
}

impl Colors {
    // determine what colors to use
    pub fn get(opts: &Opts) -> Colors {
        match opts.color {
            Color::Auto => {
                if !atty::is(atty::Stream::Stdout) {
                    return Colors::none();
                }


                let term = std::env::var("TERM").unwrap_or_default().to_lowercase();
                if NOCOLOR_TERMS.contains(&term.as_str()) {
                    return Colors::none();
                }

                let info = match Database::from_env() {
                    Ok(info) => info,
                    Err(_) => return Colors::none(),
                };

                if let Some(cap::MaxColors(n)) = info.get::<cap::MaxColors>() {
                    if n >= 8 {
                        return Colors::default();
                    }
                }

                Colors::none()
            }
            Color::Always => Colors::default(),
            Color::Never => Colors::none(),
        }
    }

    fn new(file: &str, line_no: &str, pmatch: &str) -> Self {
        Self {
            file: col(file),
            line_no: col(line_no),
            pmatch: col(pmatch),
            reset: "\x1b[0m".to_string(),
        }
    }

    fn default() -> Self {
        Self::new("35", "32", "31;1")
    }

    fn none() -> Self {
        Self {
            file: "".to_string(),
            line_no: "".to_string(),
            pmatch: "".to_string(),
            reset: "".to_string(),
        }
    }
}

fn col(col: &str) -> String {
    format!("\x1b[{}m", col)
}
