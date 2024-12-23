use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use Field::{Domain, GitHub, Token};

pub enum ConfigError {
    NonExistent(String),
    ReadError(std::io::Error),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::ReadError(e) => write!(f, "Error while reading config: {}", e),
            ConfigError::NonExistent(e) => write!(f, "File is probably non existent: {}", e),
        }
    }
}

pub enum Field {
    Token,
    Domain,
    GitHub,
}

#[derive(Default, Debug)]
pub struct Config {
    pub token: String,
    pub domain: String,
    pub github: String,
}

impl Config {
    pub fn new(token: String, domain: String, github: String) -> Self {
        Self {
            token,
            domain,
            github,
        }
    }

    pub fn set(&mut self, data: String, field: Field) -> Result<(), ConfigError> {
        match field {
            Token => self.token = data,
            Domain => self.domain = data,
            GitHub => self.github = data,
        };

        Ok(())
    }

    pub fn read(&mut self, path: PathBuf, field: Field) -> Result<(), ConfigError> {
        let data = match self.parse_file(path) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        match field {
            Token => self.token = data,
            Domain => self.domain = data,
            GitHub => self.github = data,
        };

        Ok(())
    }

    fn parse_file(&self, path: PathBuf) -> Result<String, ConfigError> {
        if !(path.is_absolute()) {
            return Err(ConfigError::NonExistent(
                "Given path is not absolute".to_string(),
            ));
        }

        if !(path.is_file()) {
            return Err(ConfigError::NonExistent(
                "This file probably doesn't exist".to_string(),
            ));
        }

        let result = match std::fs::read_to_string(path) {
            Ok(d) => d,
            Err(e) => return Err(ConfigError::ReadError(e)),
        };

        Ok(result)
    }
}
