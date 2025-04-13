use clap::ValueEnum;
use std::fmt;
use std::str::FromStr;

#[derive(Default, Debug, ValueEnum, Clone)]
pub enum Algorithm {
    #[default]
    Md5,
    Sha256,
    Sha512,
    Scrypt,
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "md5" => Ok(Algorithm::Md5),
            "sha256" => Ok(Algorithm::Sha256),
            "sha512" => Ok(Algorithm::Sha512),
            "scrypt" => Ok(Algorithm::Scrypt),
            _ => Err("invalid algorithm".to_string()), // Handle invalid input
        }
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alg = match self {
            Algorithm::Md5 => "md5",
            Algorithm::Sha256 => "sha256",
            Algorithm::Sha512 => "sha512",
            Algorithm::Scrypt => "scrypt",
        };
        write!(f, "{}", alg)
    }
}

impl Algorithm {
    pub fn len(&self) -> usize {
        self.as_bytes().len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Algorithm::Md5 => b"md5",
            Algorithm::Sha256 => b"sha256",
            Algorithm::Sha512 => b"sha512",
            Algorithm::Scrypt => b"scrypt",
        }
    }
}