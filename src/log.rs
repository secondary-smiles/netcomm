use crate::{Args};
use info_utils::prelude::*;

pub trait LogUtil {
    fn log<T>(&self, msg: T) where T: std::fmt::Display;
    fn log_v<T>(&self, msg: T) where T: std::fmt::Display;
    fn warn<T>(&self, msg: T) where T: std::fmt::Display;
    fn warn_v<T>(&self, msg: T) where T: std::fmt::Display;
}

impl LogUtil for Args {
    fn log<T>(&self, msg: T) where T: std::fmt::Display {
        if !self.quiet {
            log!("{}", msg);
        }
    }

    fn log_v<T>(&self, msg: T) where T: std::fmt::Display {
        if self.verbose {
            log!("{}", msg);
        }
    }

    fn warn<T>(&self, msg: T) where T: std::fmt::Display {
        if !self.quiet {
            warn!("{}", msg);
        }
    }

    fn warn_v<T>(&self, msg: T) where T: std::fmt::Display {
        if self.verbose {
            warn!("{}", msg);
        }
    }
}