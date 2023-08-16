use std::fmt::{Display, Formatter};
use std::ops::Deref;

use chrono::Utc;
use rand::Rng;

/// 64 bit timestamp-first unique identifier
///
/// _(40 bit timestamp followed by 24 random bits)_
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Uid(pub i64);

impl Uid {
    pub fn new() -> Self {
        let timestamp = Utc::now().timestamp() << 24;

        let mut random_bytes = [0u8; 8];
        rand::thread_rng().fill(&mut random_bytes[5..8]);
        let random_bytes = i64::from_be_bytes(random_bytes);

        Uid(timestamp | random_bytes)
    }

    #[cfg(feature = "bs58")]
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
    #[cfg(feature = "bs58")]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            bs58::encode(self.0.to_be_bytes())
                .with_alphabet(Alphabet::BITCOIN)
                .into_string()
        )
    }

    #[cfg(not(feature = "bs58"))]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl From<i64> for Uid {
    fn from(value: i64) -> Self {
        Uid(value)
    }
}
