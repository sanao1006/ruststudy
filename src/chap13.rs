use std::fmt::Display;

pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: impl Display);
}

pub struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: impl Display) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

pub struct VerbosityFilter {
    pub(crate) max_verbosity: u8,
    pub(crate) inner: StderrLogger,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: impl Display) {
        let len = message.to_string().len();
        if self.max_verbosity  >= verbosity {
            self.inner.log(verbosity, message)
        }
    }
}

pub fn do_things(logger: &impl Logger) {
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}
