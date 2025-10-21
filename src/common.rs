use std::fmt::Display;
use std::str::FromStr;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub fn deserialize_as_u64_from_number_or_string<'de, T, D>(de: D) -> Result<T, D::Error>
where
    T: FromStr + Default,
    <T as FromStr>::Err: Display,
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(de)?;

    match value {
        Value::Number(n) => {
            n.as_u64()
                .ok_or_else(|| Error::custom("invalid number"))?
                .to_string()
                .parse::<T>()
                .map_err(Error::custom)
        }
        Value::String(s) => {
            if s.is_empty() {
                Ok(T::default())
            } else {
                s.parse::<T>().map_err(Error::custom)
            }
        }
        _ => Ok(T::default()),
    }
}
