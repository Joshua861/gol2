use serde::Serialize;
use serde::{
    de::{Deserializer, Error, Visitor},
    Deserialize,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TinyStr {
    pub bytes: [u8; 20],
}

#[macro_export]
macro_rules! tiny_str {
    ($s:literal) => {{
        const S: &[u8] = $s.as_bytes();
        const _ASSERT: () = assert!(S.len() <= 20, "String literal must not exceed 20 bytes");
        let mut bytes = [0u8; 20];
        let mut i = 0;
        // Manual const initialization
        {
            while i < S.len() {
                bytes[i] = S[i];
                i += 1;
            }
            while i < 20 {
                bytes[i] = 0;
                i += 1;
            }
        }
        assert!(S.is_ascii(), "String must be ASCII-only");
        TinyStr { bytes }
    }};
}

impl Serialize for TinyStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl TryFrom<String> for TinyStr {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if s.len() > 20 {
            return Err("String too long for TinyStr (max 20 bytes)");
        }
        if !s.is_ascii() {
            return Err("String contains non-ASCII characters");
        }

        let mut bytes = [0u8; 20];
        bytes[..s.len()].copy_from_slice(s.as_bytes());
        Ok(TinyStr { bytes })
    }
}

impl TinyStr {
    pub fn new() -> Self {
        TinyStr { bytes: [0; 20] }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() > 20 || !s.is_ascii() {
            return None;
        }

        let mut bytes = [0u8; 20];
        bytes[..s.len()].copy_from_slice(s.as_bytes());
        Some(TinyStr { bytes })
    }

    pub fn as_str(&self) -> &str {
        let len = self
            .bytes
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.bytes.len());

        // Safe because we know these are valid ASCII bytes
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..len]) }
    }

    pub fn len(&self) -> usize {
        self.bytes
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.bytes.len())
    }

    pub fn is_empty(&self) -> bool {
        self.bytes[0] == 0
    }
}

impl std::fmt::Display for TinyStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'de> Deserialize<'de> for TinyStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        TinyStr::try_from(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiny_str_macro() {
        let s = tiny_str!("Hello");
        assert_eq!(s.as_str(), "Hello");
    }

    #[test]
    fn test_from_str() {
        let s = TinyStr::from_str("Hello").unwrap();
        assert_eq!(s.as_str(), "Hello");
        assert_eq!(s.len(), 5);
    }

    #[test]
    fn test_serde() {
        let s = tiny_str!("Hello");
        let json = serde_json::to_string(&s).unwrap();
        let deserialized: TinyStr = serde_json::from_str(&json).unwrap();
        println!("{:?} // {:?}", s, deserialized);
        assert_eq!(s, deserialized);
    }

    // #[test]
    // #[should_panic(expected = "String literal must not exceed 20 bytes")]
    // fn test_too_long() {
    //     let _s = tiny_str!("This string is way too long to fit");
    // }
}
