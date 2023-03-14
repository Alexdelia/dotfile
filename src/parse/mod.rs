mod error;

mod toml_to_env;
use toml_to_env::toml_to_env;

mod symlink;

use crate::symlink::Env;
use std::fs;
use std::path::Path;

use miette::Result;

pub fn parse<P>(file: P) -> Result<Env>
where
    P: AsRef<Path> + std::fmt::Display,
{
    let name = file.to_string();
    let mut env = toml_to_env(&name, read(file).unwrap())?;

    symlink::end_build(name, &mut env)?;

    Ok(env)
}

fn read<P>(file: P) -> miette::Result<toml::Value>
where
    P: AsRef<Path> + std::fmt::Display,
{
    let s = toml::from_str(&fs::read_to_string(&file).map_err(|e| error::ReadError {
        source: e,
        file: file.to_string(),
        origin_file: file!().to_string(),
    })?)
    .map_err(|e| error::ParseStrToTOMLError {
        source: e,
        file: file.to_string(),
        origin_file: file!().to_string(),
    })?;
    Ok(s)
}
