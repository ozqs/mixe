use crate::mixword::MIXWord;
use crate::Unit;
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

mod ordering_serde {
    use serde::{Serializer, Deserializer, Deserialize};
    use std::cmp::Ordering;

    pub fn serialize<S>(ordering: &Ordering, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match ordering {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        serializer.serialize_i8(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Ordering, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i8::deserialize(deserializer)?;
        match value {
            -1 => Ok(Ordering::Less),
            0 => Ok(Ordering::Equal),
            1 => Ok(Ordering::Greater),
            _ => Err(serde::de::Error::custom("Invalid value for Ordering")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MIXComputer {
    // A J1 J2 J3 J4 J5 J6 X J
    #[serde(with = "serde_arrays")]
    pub register: [MIXWord; 9],
    pub overflow: bool,
    #[serde(with = "ordering_serde")]
    pub comp: Ordering, // -1 0 1
    #[serde(with = "serde_arrays")]
    pub units: [Unit; 16],
    #[serde(with = "serde_arrays")]
    pub memory: [MIXWord; 4000],
}

impl Default for MIXComputer {
    fn default() -> Self {
        Self::new()
    }
}

impl MIXComputer {
    pub fn new() -> Self {
        MIXComputer {
            register: [0u32.into(); 9],
            overflow: false,
            comp: Ordering::Less,
            units: std::array::from_fn(|i| Unit::new(i as u32)),
            memory: [0u32.into(); 4000],
        }
    }
}
