use serde::{Deserialize, Serialize};
use std::{str::FromStr, fmt::Display};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Language {
    EN,
    JA
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::EN => write!(f, "EN"),
            Language::JA => write!(f, "JA")
        }
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(language: &str) -> Result<Language, Self::Err> {
        match language {
            "EN" => Ok(Language::EN),
            "JA" => Ok(Language::JA),
            _ => Err(())
        }
    } 
}