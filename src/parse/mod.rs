mod error;
use error::ParseError;

mod toml_to_env;
use toml_to_env::toml_to_env;

mod symlink;

use crate::symlink::Env;
use std::fs;
use std::path::Path;

pub fn parse<P>(file: P) -> Result<Env, ParseError>
where
    P: AsRef<Path> + std::fmt::Display,
{
    let env = toml_to_env(file.to_string().as_str(), read(file)?)?;

    symlink::validate(&env)?;

    Ok(env)
}

fn read<P>(file: P) -> Result<toml::Value, ParseError>
where
    P: AsRef<Path> + std::fmt::Display,
{
    toml::from_str(&fs::read_to_string(&file).map_err(|e| ParseError::Read {
        source: e,
        file: file.to_string(),
    })?)
    .map_err(|e| ParseError::ParseStrToTOML {
        source: e,
        file: file.to_string(),
    })
}
