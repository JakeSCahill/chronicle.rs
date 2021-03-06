use chronicle_cql::frame::{
    decoder::ColumnDecoder,
    encoder::ColumnEncoder,
};
use serde::{
    self,
    Deserialize,
    Serialize,
};
#[derive(Copy, Clone)]
pub struct Trytes81(pub [u8; 81]);
pub const BE_81_BYTES_LENGTH: [u8; 4] = [0, 0, 0, 81];

impl Serialize for Trytes81 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str = std::str::from_utf8(&self.0).unwrap();
        serializer.serialize_str(str)
    }
}

impl<'de> Deserialize<'de> for Trytes81 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Trytes81;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str("a string with 81-bytes length")
            }

            fn visit_str<E>(self, value: &str) -> Result<Trytes81, E>
            where
                E: ::serde::de::Error,
            {
                if value.len() == 81 {
                    let mut trytes81 = [0; 81];
                    trytes81.copy_from_slice(value.as_bytes());
                    Ok(Trytes81(trytes81))
                } else {
                    Err(E::custom(format!("require 81, invalid length: {}", value.len())))
                }
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl ColumnEncoder for &Trytes81 {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend(&BE_81_BYTES_LENGTH);
        buffer.extend(&self.0[..]);
    }
}

impl ColumnDecoder for Trytes81 {
    fn decode(slice: &[u8], _: usize) -> Self {
        let mut trytes81 = [0; 81];
        trytes81.copy_from_slice(&slice[..81]);
        Trytes81(trytes81)
    }
}

#[derive(Copy, Clone)]
pub struct Trytes27(pub [u8; 27]);
pub const BE_27_BYTES_LENGTH: [u8; 4] = [0, 0, 0, 27];

impl Serialize for Trytes27 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str = std::str::from_utf8(&self.0).unwrap();
        serializer.serialize_str(str)
    }
}

impl<'de> Deserialize<'de> for Trytes27 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Trytes27;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str("a string with 27-bytes length")
            }

            fn visit_str<E>(self, value: &str) -> Result<Trytes27, E>
            where
                E: ::serde::de::Error,
            {
                if value.len() == 27 {
                    let mut trytes27 = [0; 27];
                    trytes27.copy_from_slice(value.as_bytes());
                    Ok(Trytes27(trytes27))
                } else {
                    Err(E::custom(format!("require 27, invalid length: {}", value.len())))
                }
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

impl ColumnEncoder for &Trytes27 {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend(&BE_27_BYTES_LENGTH);
        buffer.extend(&self.0[..]);
    }
}

impl ColumnDecoder for Trytes27 {
    fn decode(slice: &[u8], _: usize) -> Self {
        let mut trytes27 = [0; 27];
        trytes27.copy_from_slice(&slice[..27]);
        Trytes27(trytes27)
    }
}
