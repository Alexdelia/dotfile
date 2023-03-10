mod error;
use error::ParseError;

use crate::ansi::{BE, BW, E, V, W};
use crate::symlink::{Env, EnvType, Grouped, Symlink, Update};
use ansi::abbrev::D;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn parse<P>(file: P) -> Result<Env, ParseError>
where
    P: AsRef<Path> + std::fmt::Display,
{
    let env = toml_to_env(file.to_string().as_str(), read(file)?)?;

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

fn toml_to_env(file: &str, toml: toml::Value) -> Result<Env, ParseError> {
    let table = toml.as_table().ok_or_else(|| ParseError::ParseTOMLToEnv {
        file: file.to_string(),
    })?;

    let mut env = Env::new();

    for (k, v) in table {
        match v {
            toml::Value::Table(table) => env.push(EnvType::Grouped(table_to_grouped(file, k.to_owned(), table.to_owned())?)),
            toml::Value::String(string) => env.push(EnvType::Alone(Symlink {
                path: PathBuf::from(string),
                target: PathBuf::from(k),
            })),
            _ => {
                return Err(ParseError::wrong_type(
                    file.to_string(),
					None,
                    k,
                    format!(
                        "{E}value {BE}{v}{D} for {W}key {BW}{k}{D} is not a {BW}string{D} or {BW}table{D}"
                    ),
                    format!(
                        "the {E}value{D} should either be:
	- a {V}string{D} that represent the path to the actual dotfile
	- a {V}table{D} that represent a list of symlink"
                    ),
                ))
            }
        }
    }

    Ok(env)
}

fn table_to_grouped(file: &str, title: String, table: toml::Table) -> Result<Grouped, ParseError> {
    let mut symlink: Vec<Symlink> = Vec::new();
    let mut update = Update::Always;

    for (k, v) in table {
        if k == "update" {
            update = to_update(file, &title, v)?;
            continue;
        }

        match v {
            toml::Value::String(string) => symlink.push(Symlink {
                path: PathBuf::from(string),
                target: PathBuf::from(k),
            }),
            _ => {
                return Err(ParseError::wrong_type(
                    file.to_string(),
                    Some(&title),
                    &k,
                    format!("{E}value {BE}{v}{D} for {W}key {BW}{k}{D} is not a {BW}string{D}"),
                    format!(
						"the {E}value{D} should be a {V}string{D} that represent the path to the actual dotfile"
					),
                ))
            }
        }
    }

    Ok(Grouped {
        title,
        symlink,
        update,
    })
}

fn to_update(file: &str, title: &str, value: toml::Value) -> Result<Update, ParseError> {
    match value {
        toml::Value::String(string) => match Update::from_str(string.as_str()) {
            Ok(update) => Ok(update),
            Err(_) => Err(ParseError::update(
                file.to_string(),
                title.to_string(),
                string,
            )),
        },
        toml::Value::Array(array) => {
            let mut name = Vec::new();

            for value in array {
                match value {
                    toml::Value::String(string) => name.push(string),
                    _ => {
                        return Err(ParseError::update(
                            file.to_string(),
                            title.to_string(),
                            format!("{:?}", value),
                        ))
                    }
                }
            }

            Ok(Update::Specific(name))
        }
        _ => Err(ParseError::update(
            file.to_string(),
            title.to_string(),
            format!("{:?}", value),
        )),
    }
}
