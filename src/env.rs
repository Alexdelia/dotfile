use ansi::abbrev::{B, C, D};
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

impl std::fmt::Display for Symlink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{B}{C}{}{D} -> {B}{}{D}",
            self.path.display(),
            self.target.display()
        )
    }
}

impl Symlink {
    pub fn create(&self) -> std::io::Result<()> {
        std::os::unix::fs::symlink(&self.target, &self.path)
    }

    pub fn print_action(&self, action: &str) {
        let termsize::Size { cols, .. } =
            termsize::get().unwrap_or(termsize::Size { cols: 0, rows: 0 });

        let f = format!("{self}");
        let w: u16 = if f.len() + action.len() > cols as usize {
            0
        } else {
            cols - f.len() as u16 - action.len() as u16
        };

        println!("{f}{ 
    }
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
