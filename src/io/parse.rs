use crate::escape::{BWARNING as W, GRE as G, MUT as M, RESET as D};
use crate::io::{ESCAPE_HYPERLINK_E as EHE, ESCAPE_HYPERLINK_S as EHS, URL};
use crate::symlink::{Env, EnvType, Symlink, DEFAULT_SYMLINK_FILE};
use miette::Diagnostic;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error("could not {W}read {M}{file}{D}")]
    #[diagnostic(
        code(parse::read),
        url("{}{}", URL, file!()),
        help("the file {M}{file}{D} is the file that define symlink")
    )]
    Read {
        #[source]
        source: std::io::Error,
        #[source_code]
        file: String,
    },

    #[error("could not {W}parse {M}{file}{D} to {W}toml{D}")]
    #[diagnostic(
        code(parse::read),
        url("{}{}", URL, file!()),
        help("the file {M}{file}{D} should be a {G}valid toml{D} file
you can check the example file \x1b[32;1m{DEFAULT_SYMLINK_FILE}\x1b[0m {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}
or the {G}toml{D} documentation {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}")
    )]
    ParseStrToTOML {
        #[source]
        source: toml::de::Error,
        #[source_code]
        file: String,
    },

    #[error("could not {W}parse {M}{file}{D} to valid list of {W}symlink{D}")]
    #[diagnostic(
		code(parse::toml_to_env),
		url("{}{}", URL, file!()),
		help("the file {M}{file}{D} should be a {G}valid toml{D} that define the {G}symlink{D}
you can check the example file \x1b[32;1m{DEFAULT_SYMLINK_FILE}\x1b[0m {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}")
	)]
    ParseTOMLToEnv {
        #[source_code]
        file: String,
    },
}

/*
pub fn parse<P>(file: P) -> Result<Env, ParseError>
where
    P: AsRef<Path> + std::fmt::Display,
{
    let table = read(file)?;
}
*/

fn read<P>(file: P) -> Result<toml::Value, ParseError>
where
    P: AsRef<Path> + std::fmt::Display,
{
    Ok(
        toml::from_str(&fs::read_to_string(&file).map_err(|e| ParseError::Read {
            source: e,
            file: file.to_string(),
        })?)
        .map_err(|e| ParseError::ParseStrToTOML {
            source: e,
            file: file.to_string(),
        })?,
    )
}

pub fn test<P>(file: P) -> Result<toml::Value, ParseError>
where
    P: AsRef<Path> + std::fmt::Display,
{
    Ok(
        toml::from_str(&fs::read_to_string(&file).map_err(|e| ParseError::Read {
            source: e,
            file: file.to_string(),
        })?)
        .map_err(|e| ParseError::ParseStrToTOML {
            source: e,
            file: file.to_string(),
        })?,
    )
}

fn toml_to_env(file: &str, toml: toml::Value) -> Result<Env, ParseError> {
    let table = toml.as_table().ok_or_else(|| ParseError::ParseTOMLToEnv {
        file: file.to_string(),
    })?;

    let mut env = Env::new();

    for (k, v) in table {
        match v {
            toml::Value::Table(table) => {
                env.push(EnvType::Grouped((k, table_to_grouped(file, table)?)))
            }
            toml::Value::String(string) => env.push(EnvType::Alone(Symlink {
                path: PathBuf::from(string),
                target: PathBuf::from(k),
            })),
            _ => {
                return Err(ParseError::ParseTOMLToEnv {
                    file: file.to_string(),
                })
            }
        }
    }

    Ok(env)

    /*
    match toml {
        toml::Value::Table(table) => Ok(table_to_env(table)?),
        _ => Err(ParseError::ParseEnv {
            file: file.to_string(),
        }),
    }
    Ok(toml
        .into_iter()
        .map(|(k, v)| match v {
            toml::Value::Table(table) => Ok(EnvType::Grouped((k, table_to_env(table)?))),
            toml::Value::Array(array) => Ok(EnvType::Grouped((k, array_to_env(array)?))),
            toml::Value::String(string) => Ok(EnvType::Alone(Symlink {
                path: PathBuf::from(string),
                target: PathBuf::from(k),
            })),
            _ => Err(ParseError::ParseError {
                source: toml::de::Error::custom(""),
                file: file.to_string(),
            }),
        })
        .collect::<Result<Env, ParseError>>()?)
        */
}

fn table_to_grouped(file: &str, table: toml::Table) -> Result<Vec<Symlink>, ParseError> {
    todo!()
}
