use rbs::Value;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use crate::Error;

#[derive(serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Time")]
pub struct Time(pub fastdate::Time);

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time({})", self.0)
    }
}

impl Debug for Time{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time({})", self.0)
    }
}

impl From<Time> for Value {
    fn from(arg: Time) -> Self {
        Value::Ext("Time", Box::new(Value::String(arg.0.to_string())))
    }
}

impl FromStr for Time{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Time(fastdate::Time::from_str(s)?))
    }
}
