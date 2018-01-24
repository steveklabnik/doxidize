#[derive(Debug, Default)]
pub struct Ui {
    verbosity: Verbosity,
}

impl Ui {
    pub fn new(verbosity: Verbosity) -> Ui {
        Ui { verbosity }
    }

    pub fn warn(&self, message: &str) {
        if self.verbosity > Verbosity::Quiet {
            eprintln!("warning: {}", message);
        }
    }

    pub fn verbosity(&self) -> &Verbosity {
        &self.verbosity
    }
}

/// The verbosity of the output displayed to the user.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    /// No output.
    Quiet,

    /// Normal output, with spinners.
    Normal,

    /// Verbose output. No spinners are displayed, and all intermediate output is printed.
    Verbose,
}

impl Default for Verbosity {
    fn default() -> Verbosity {
        Verbosity::Normal
    }
}