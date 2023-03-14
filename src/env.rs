use std::path::PathBuf;
use std::str::FromStr;

pub type Env = Vec<EnvType>;

#[derive(Debug)] // TODO: remove
pub enum EnvType {
    Grouped(Grouped),
    Alone(Symlink),
}

#[derive(Debug)] // TODO: remove
pub struct Grouped {
    pub title: String,
    pub update: Update,
    pub symlink: Vec<Symlink>,
}

#[derive(Debug)] // TODO: remove
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

#[derive(Debug)] // TODO: remove
pub struct Symlink {
    pub path: PathBuf,
    pub exist: Exist,
    pub target: PathBuf,
}

#[derive(Debug)] // TODO: remove
pub enum Exist {
    Yes(FileType),
    No,
}

#[derive(Debug)] // TODO: remove
pub enum FileType {
    File,
    Dir,
    Symlink(bool),
}
