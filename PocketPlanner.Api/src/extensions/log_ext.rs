use anyhow::Error;
use log::error;

pub trait LogExt {
    fn log_err(self);
}

impl LogExt for Result<(), Error> {
    fn log_err(self) {
        match self {
            Ok(_) => (),
            Err(error) => error!("{error}"),
        }
    }
}
