use std::path::PathBuf;

use crate::error::Error;

pub fn config_root() -> Result<PathBuf, Error> {
    dirs::home_dir()
        .map(|path| path.join(".config").join("irgen"))
        .ok_or_else(|| Error::ConfigInitialization {
            message: "Failed to resolve home directory for user config.".into(),
        })
}

pub fn templates_dir() -> Result<PathBuf, Error> {
    Ok(config_root()?.join("templates"))
}
