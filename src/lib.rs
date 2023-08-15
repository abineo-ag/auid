use std::fmt::{Display, Formatter};
use std::ops::Deref;

use bs58::Alphabet;
use chrono::Utc;
use rand::Rng;

/// 64 bit timestamp-first unique identifier
///
/// _(40 bit timestamp followed by 24 random bits)_
pub struct Uid(i64);

impl Uid {
    pub fn new() -> Self {
        let timestamp = Utc::now().timestamp() << 24;

        let mut random_bytes = [0u8; 8];
        rand::thread_rng().fill(&mut random_bytes[5..8]);
        let random_bytes = i64::from_be_bytes(random_bytes);

        Uid(timestamp | random_bytes)
    }

    pub fn to_hex(&self) -> String {
        format!("{:x}", self.0)
    }
}

impl Deref for Uid {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Uid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            bs58::encode(self.0.to_be_bytes())
                .with_alphabet(Alphabet::BITCOIN)
                .into_string()
        )
    }
}

impl From<i64> for Uid {
    fn from(value: i64) -> Self {
        Uid(value)
    }
}
