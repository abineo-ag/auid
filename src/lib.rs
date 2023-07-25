use std::fmt::{Display, Formatter};
use std::ops::Deref;

use bs58::Alphabet;
use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use simd_adler32::adler32;

/// 128 bit timestamp-first unique identifier
///
/// _(32 bit unix timestamp followed by 96 random bits)_
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Uid(pub String);

impl Uid {
    pub fn new() -> Self {
        let mut bytes = [0u8; 16];

        // 32 bit unix timestamp
        let unix_ts = Utc::now().timestamp().to_be_bytes();
        bytes[0..4].copy_from_slice(&unix_ts[4..8]);

        // 96 random bits
        rand::thread_rng().fill(&mut bytes[4..16]);

        Uid(bs58::encode(&bytes)
            .with_alphabet(Alphabet::BITCOIN)
            .into_string())
    }

    pub fn to_string(self) -> String {
        self.0
    }

    pub fn as_string(&self) -> &String {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// generates checksum using `Adler-32` hashing algorithm
    pub fn checksum(&self) -> String {
        format!("{:x?}", adler32(&self.0.as_str()))
    }
}

impl<T: Into<String>> From<T> for Uid {
    fn from(value: T) -> Self {
        Uid(value.into())
    }
}

impl Display for Uid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Uid {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let id = Uid::new();
        assert!(!id.0.is_empty());
        assert!(!id.checksum().is_empty());
    }
}
