use crate::unix::Symlink;
use std::str::FromStr;

pub type Env = Vec<EnvType>;

pub enum EnvType {
    Grouped(Grouped),
    Alone(Symlink),
}

pub struct Grouped {
    pub title: String,
    pub update: Update,
    pub symlink: Vec<Symlink>,
}

pub enum Update {
    Always,
    Never,
    Optional,
    Specific(Vec<String>),
}

impl FromStr for Update {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "always" => Ok(Update::Always),
            "never" => Ok(Update::Never),
            "optional" => Ok(Update::Optional),
            _ => Err(format!("{s} is not a valid update value")),
        }
    }
}
