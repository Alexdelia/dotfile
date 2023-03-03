// read toml file and parse it

use crate::io::url as u;
use miette::Diagnostic;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error("could not read {file}")]
    #[diagnostic(
        code(parse::read),
        url(u(file!())),
        help("the file {file} is the file that define symlink")
    )]
    ReadError {
        #[source]
        source: std::io::Error,
        #[source_code]
        file: String,
    },

    #[error("could not parse file")]
    ParseError(#[from] toml::de::Error),
}

pub fn read<P>(file: P) -> Result<toml::map::Map<String, toml::Value>, ParseError>
where
    P: AsRef<Path> + std::fmt::Display,
{
    Ok(toml::from_str(&fs::read_to_string(&file).map_err(
        |e| ParseError::ReadError {
            source: e,
            file: file.to_string(),
        },
    )?)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_read() {
        dbg!(read("no.toml"));
        dbg!(read("symlink.toml"));
    }
}
