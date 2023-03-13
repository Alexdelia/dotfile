use super::error::{ParseTOMLToEnvError, ParseTomlError};

use crate::ansi::{BE, BW, E, M, V, W};
use crate::symlink::{Env, EnvType, Exist, Grouped, Symlink, Target, Update};
use ansi::abbrev::D;
// use miette::Result;
use std::path::PathBuf;
use std::str::FromStr;

pub fn toml_to_env(file: &str, toml: toml::Value) -> Result<Env, ParseTomlError> {
    let table = toml.as_table().unwrap();
    // .ok_or_else(|| ParseTOMLToEnvError {
    //     file: file.to_string(),
    //     origin_file: file!().to_string(),
    // })?;

    let mut env = Env::new();

    for (k, v) in table {
        match v {
            toml::Value::Table(table) => env.push(EnvType::Grouped(table_to_grouped(
                file,
                k.to_owned(),
                table.to_owned(),
            )?)),
            toml::Value::String(string) => env.push(EnvType::Alone(Symlink {
                path: PathBuf::from(string),
                target: Target {
                    path: PathBuf::from(k),
                    exist: Exist::Not,
                },
            })),
            _ => {
                return Err(ParseTomlError::new(
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
                    file!().to_string(),
                    None,
                )
                .into());
            }
        }
    }

    Ok(env)
}

fn table_to_grouped(
    file: &str,
    title: String,
    table: toml::Table,
) -> Result<Grouped, ParseTomlError> {
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
                target: Target {
                    path: PathBuf::from(k),
                    exist: Exist::Not,
                },
            }),
            _ => {
                return Err(ParseTomlError::new(
                    file.to_string(),
                    Some(&title),
                    &k,
                    format!("{E}value {BE}{v}{D} for {W}key {BW}{k}{D} is not a {BW}string{D}"),
                    format!(
						"the {E}value{D} should be a {V}string{D} that represent the path to the actual dotfile"
					),
                    file!().to_string(),
                    None,
                )
                .into());
            }
        }
    }

    Ok(Grouped {
        title,
        symlink,
        update,
    })
}

fn to_update(file: &str, title: &str, value: toml::Value) -> Result<Update, ParseTomlError> {
    let u = match value {
		toml::Value::String(ref string) => Update::from_str(string.as_str()).map_err(|_| ()),
        toml::Value::Array(ref array) => {
            let mut name = Vec::new();

			let mut err = false;
            for value in array {
                match value {
                    toml::Value::String(string) => name.push(string.to_owned()),
					_ => err = true,
                    // _ => {
                    //     return Err(ParseError::update(
                    //         file.to_string(),
                    //         title.to_string(),
                    //         format!("{:?}", value),
                    //     ))
                    // }
                }
            }

			if err {
				Err(())
			} else {

            Ok(Update::Specific(name))
			}
        }
        _ => Err(()),
		// Err(ParseError::update(
        //     file.to_string(),
        //     title.to_string(),
        //     format!("{:?}", value),
        // )),
    }.map_err(|()| ParseTomlError::new(
		file.to_string(),
		Some(title),
		"update",
		format!(
			"in {M}{title}{D}, {E}value {BE}{value}{D} for {W}key {BW}update{D} does not represent a {BW}valid update frequency{D}"
		),
		format!(
			"the {E}value{D} should represent a {V}valid update frequency{D}"
		),
		file!().to_string(),
		None,
	))?;

    Ok(u)
}
