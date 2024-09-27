use log::info;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub enum Channel {
    A,
    B,
    C,
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Channel::A => "A",
            Channel::B => "B",
            Channel::C => "C",
        };
        f.write_str(val)
    }
}

impl From<&str> for Channel {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            _ => Self::A,
        }
    }
}

impl FromStr for Channel {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            _ => Err("Invalid channel. Only channels A,B and C are supported".into()),
        }
    }
}

#[derive(Debug)]
pub struct LapTime {
    pub seq_number: usize,
    pub channel: Channel,
    pub time: String,
}

impl Display for LapTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[SEQ_NUMBER: {}, CHANNEL: {}, TIME: {}]",
            self.seq_number, self.channel, self.time
        )
    }
}

impl From<String> for LapTime {
    fn from(value: String) -> Self {
        info!("RAW: \"{}\"", &value.trim());
        let mut chunks = value.split_whitespace();
        let seq_number = chunks
            .next()
            .expect("Invalid timestamp")
            .trim_matches(char::from(0))
            .parse()
            .unwrap();

        let channel = Channel::from_str(chunks.next().unwrap())
            .expect("Invalid input: expected a channel in the second place.");

        let time = chunks.next().expect("No timestamp found").to_owned();

        Self {
            seq_number,
            channel,
            time,
        }
    }
}
