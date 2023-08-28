//! 64 bit timestamp-first unique identifier
//!
//! # Usage
//!
//! ```
//! # use auid::Uid;
//! let id = Uid::new();
//! println!("{id}");
//! ```
//!
//! # Features
//!
//! name | description | default?
//! :--- |:--- | :---:
//! `serde` | [`serde`] support | ✓
//! `base16` | convert from/to base16 using [`faster-hex`] | 𐄂
//! `hex` | alias for `base16` |
//! `base32` | convert from/to base32 using [`data-encoding`] | 𐄂
//! `base58` | convert from/to base58 using [`bs58`] | 𐄂
//! `base64` | convert from/to base64 using [`data-encoding`] | 𐄂
//!
//! [`serde`]: https://docs.rs/serde
//! [`faster-hex`]: https://docs.rs/faster-hex
//! [`data-encoding`]: https://docs.rs/data-encoding
//! [`bs58`]: https://docs.rs/bs58

#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate core;

use std::fmt::{Display, Formatter};

use chrono::Utc;
use rand::Rng;
use thiserror::Error;

use crate::Error::Decoding;

/// 64 bit timestamp-first unique identifier
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Uid(i64);

impl Uid {
    /// Creates a new uid using a 40 bit timestamp followed by 24 random bits
    pub fn new() -> Self {
        let timestamp = Utc::now().timestamp() << 24;

        let mut random_bytes = [0u8; 8];
        rand::thread_rng().fill(&mut random_bytes[5..8]);
        let random_bytes = i64::from_be_bytes(random_bytes);

        Uid(timestamp | random_bytes)
    }
}

impl From<Uid> for i64 {
    fn from(value: Uid) -> Self {
        value.0
    }
}

impl From<i64> for Uid {
    fn from(value: i64) -> Self {
        Uid(value)
    }
}

impl From<Uid> for [u8; 8] {
    fn from(value: Uid) -> Self {
        value.0.to_be_bytes()
    }
}

impl TryFrom<&[u8]> for Uid {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 8 {
            let err = format!("expected len to be 8, but was {}", value.len());
            return Err(Decoding(err));
        }

        let mut arr = [0u8; 8];
        arr.copy_from_slice(value);

        let id = i64::from_be_bytes(arr);
        Ok(Uid(id))
    }
}

impl Display for Uid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "base16")]
#[cfg_attr(docsrs, doc(cfg(feature = "base16")))]
/// Convert from/to base16 using [`faster-hex`]
///
/// [`faster-hex`]: https://docs.rs/faster-hex
impl Uid {
    pub fn to_base16(&self) -> String {
        _faster_hex::hex_string(&self.0.to_be_bytes())
    }

    /// alias to [`to_base16`]
    ///
    /// [`to_base16`]: Uid::to_base16
    pub fn to_hex(&self) -> String {
        self.to_base16()
    }

    pub fn from_base16(value: &str) -> Result<Self, Error> {
        let mut arr = [0u8; 8];
        _faster_hex::hex_decode(value.as_bytes(), &mut arr)
            .map_err(|err| Decoding(err.to_string()))?;

        let id = i64::from_be_bytes(arr);
        Ok(Uid(id))
    }

    /// alias to [`from_base16`]
    ///
    /// [`from_base16`]: Uid::from_base16
    pub fn from_hex(value: &str) -> Result<Self, Error> {
        Self::from_base16(value)
    }
}

#[cfg(feature = "base32")]
#[cfg_attr(docsrs, doc(cfg(feature = "base32")))]
/// Convert from/to base32 using [`data-encoding`]
///
/// [`data-encoding`]: https://docs.rs/data-encoding
impl Uid {
    pub fn to_base32(&self) -> String {
        _data_encoding::BASE32.encode(&self.0.to_be_bytes())
    }

    pub fn to_unpadded_base32(&self) -> String {
        _data_encoding::BASE32_NOPAD.encode(&self.0.to_be_bytes())
    }

    pub fn from_base32(value: &str) -> Result<Self, Error> {
        let bytes = _data_encoding::BASE32
            .decode(value.as_bytes())
            .map_err(|err| Decoding(err.to_string()))?;
        Uid::try_from(bytes.as_slice())
    }

    pub fn from_unpadded_base32(value: &str) -> Result<Self, Error> {
        let bytes = _data_encoding::BASE32_NOPAD
            .decode(value.as_bytes())
            .map_err(|err| Decoding(err.to_string()))?;
        Uid::try_from(bytes.as_slice())
    }
}

#[cfg(feature = "base58")]
#[cfg_attr(docsrs, doc(cfg(feature = "base58")))]
/// Convert from/to base58 using [`bs58`]
///
/// [`bs58`]: https://docs.rs/bs58
impl Uid {
    pub fn to_base58(&self) -> String {
        _bs58::encode(self.0.to_be_bytes())
            .with_alphabet(_bs58::Alphabet::BITCOIN)
            .into_string()
    }

    pub fn from_base58(value: &str) -> Result<Self, Error> {
        let bytes = _bs58::decode(value)
            .into_vec()
            .map_err(|err| Decoding(err.to_string()))?;
        Uid::try_from(bytes.as_slice())
    }
}

#[cfg(feature = "base64")]
#[cfg_attr(docsrs, doc(cfg(feature = "base64")))]
/// Convert from/to base64 using [`data-encoding`]
///
/// [`data-encoding`]: https://docs.rs/data-encoding
impl Uid {
    pub fn to_base64(&self) -> String {
        _data_encoding::BASE64.encode(&self.0.to_be_bytes())
    }

    pub fn to_unpadded_base64(&self) -> String {
        _data_encoding::BASE64_NOPAD.encode(&self.0.to_be_bytes())
    }

    pub fn from_base64(value: &str) -> Result<Self, Error> {
        let bytes = _data_encoding::BASE64
            .decode(value.as_bytes())
            .map_err(|err| Decoding(err.to_string()))?;
        Uid::try_from(bytes.as_slice())
    }

    pub fn from_unpadded_base64(value: &str) -> Result<Self, Error> {
        let bytes = _data_encoding::BASE64_NOPAD
            .decode(value.as_bytes())
            .map_err(|err| Decoding(err.to_string()))?;
        Uid::try_from(bytes.as_slice())
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("{0}")]
    Decoding(String),
}
