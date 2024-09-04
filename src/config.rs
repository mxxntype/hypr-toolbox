use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error: {0:?}")]
    Io(#[from] io::Error),
    #[error("(De)serialization error: {0:?}")]
    Serde(#[from] serde_json::Error),
}

#[allow(clippy::module_name_repetitions)]
pub trait ExternalConfig {
    /// The type that should hold the actual configuration options.
    type Config: Default + Serialize + for<'de> Deserialize<'de>;

    /// The name of the file that holds the config for this tool.
    const FILENAME: &'static str;

    /// Get the path to the directory that should contain the configuration file.
    #[must_use]
    fn directory() -> PathBuf {
        dirs::config_dir()
            .map(|mut path| {
                path.push(env!("CARGO_CRATE_NAME"));
                path
            })
            .expect("Couldn't find where to store config files - where's ~/.config?")
    }

    /// Get the full path to the configuration file.
    #[must_use]
    fn full_path() -> PathBuf {
        let mut path = Self::directory();
        path.push(Self::FILENAME);
        path
    }

    /// Write the default configuration to disk as JSON.
    ///
    /// # Panics
    ///
    /// Panics if an error occurs while serializing [`Self::Config`] to
    /// JSON (inside [`serde`]), which shouldn't happen realistically.
    ///
    /// # Errors
    ///
    /// This function will return an error if an I/O error occurs while
    /// creating the crate's configuration subdirectory or writing the
    /// JSON to [`Self::FILENAME`] inside.
    fn write_default_to_disk() -> io::Result<()> {
        let config = Self::Config::default();
        let json = serde_json::to_string_pretty(&config).unwrap();
        let _ = fs::create_dir_all(Self::directory());
        fs::write(Self::full_path(), json)
    }

    /// Read the configuration file.
    ///
    /// # Errors
    ///
    /// This function will return an error if an I/O error occurs while
    /// reading the configuration file or it contains invalid syntax or
    /// options (if deserialization fails).
    fn read_from_disk() -> Result<Self::Config, self::Error> {
        let path = Self::full_path();
        let file_contents = fs::read_to_string(path)?;
        let config = serde_json::from_str::<Self::Config>(&file_contents)?;
        Ok(config)
    }

    /// Read the configuration or setup a default one.
    ///
    /// # Errors
    ///
    /// This function will propagate errors from [`Self::read_from_disk()`].
    fn setup_default_or_read_existing() -> Result<Self::Config, self::Error> {
        match fs::exists(Self::full_path()) {
            // Config file already exists, load it.
            Ok(true) => Self::read_from_disk(),

            // Config file does NOT exist, write a default one and load it.
            Ok(false) => {
                let _ = Self::write_default_to_disk().inspect_err(|error| {
                    eprintln!("Could not setup default configuration file: {error:?}");
                });

                Ok(Self::Config::default())
            }

            // Can't figure out what's with the config file.
            Err(_) => Ok(Self::Config::default()),
        }
    }
}
