use crate::ansi::{BE, BW, C, E, EHE, EHS, H, M, V, W};
use crate::io::URL;
use crate::symlink::{Env, EnvType, Grouped, Symlink, Update, DEFAULT_SYMLINK_FILE};
use ansi::abbrev::{B, BLU, D, F, YEL};
use const_format::formatcp;
use miette::{Diagnostic, SourceSpan};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use thiserror::Error;

const HELP_EXAMPLE: &str = formatcp!("you can check the example file {V}{DEFAULT_SYMLINK_FILE}{D} {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}");

#[derive(Error, Diagnostic, Debug)]
pub enum ParseError {
    #[error("could not {BW}read {M}{file}{D}")]
    #[diagnostic(
        code(parse::read),
        url("{}{}", URL, file!()),
        help("the file {M}{file}{D} is the file that define symlink")
    )]
    Read {
        #[source]
        source: std::io::Error,
        file: String,
    },

    #[error("could not {BW}parse {M}{file}{D} to {BW}toml{D}")]
    #[diagnostic(
        code(parse::read),
        url("{}{}", URL, file!()),
        help("the file {M}{file}{D} should be a {V}valid toml{D} file
{HELP_EXAMPLE}
or the {V}toml{D} documentation {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}")
    )]
    ParseStrToTOML {
        #[source]
        source: toml::de::Error,
        file: String,
    },

    #[error("could not {BW}parse {M}{file}{D} to valid list of {BW}symlink{D}")]
    #[diagnostic(
		code(parse::toml_to_env),
		url("{}{}", URL, file!()),
		help("the file {M}{file}{D} should be a {V}valid toml{D} that define the {V}symlink{D}s
{HELP_EXAMPLE}")
	)]
    ParseTOMLToEnv { file: String },

    #[error(
        "{reason}

could not {BW}parse {M}{file_name}{D} to valid list of {BW}symlink{D}"
    )]
    #[diagnostic(
		code(parse::value_to_T),
		url("{}{}", URL, file!()),
		help("{advice}

the file {M}{file_name}{D} should be a {V}valid toml{D} that define the {V}symlink{D}s

{H}example:{D}
{F}```{D}
{C}# optional title{D}
{B}{YEL}[title]{D}
{C}# optional update frequency (default: 'always'){D}
{C}# key (string) = value ('always' | 'never' | 'optional' | [ 'ListOfComputerNameToAlwaysUpdate', 'OtherComputerName' ]){D}
{BLU}update{D} = 'always'
{C}# symlink list{D}
{C}# key (string) = value (string){D}
\"~/path/to/dotfile/where/symlink/will/be\" = \"path/of/actual/dotfile/stored/in/data/directory\"
{F}```{D}

{HELP_EXAMPLE}")
	)]
    ParseSymlinkWrongType {
        file_name: String,
        #[source_code]
        file: String,
        #[label]
        wrong_bit: SourceSpan,

        reason: String,
        advice: String,
    },

    #[error("{E}value {BE}{value}{D} for {W}key {BW}update{D} does not represent a {BW}valid update frequency{D}")]
    #[diagnostic(
		code(parse::to_update),
		url("{}{}", URL, file!()),
        help("the {E}value{D} should represent a {V}valid update frequency{D}

'always' | 'never' | 'optional' | [ 'ListOfComputerNameToAlwaysUpdate', 'OtherComputerName' ]
default: 'always'

{HELP_EXAMPLE}")
	)]
    ParseUpdate {
        file_name: String,
        #[source_code]
        file: String,
        #[label]
        wrong_bit: SourceSpan,

        value: String,
        title: String,
    },
}

impl ParseError {
    pub fn wrong_type(file: String, key: &str, reason: String, advice: String) -> Self {
        let content =
            fs::read_to_string(&file).unwrap_or_else(|_| String::from("ENABLE TO READ FILE\n"));

        // will not work if the key is repeated in the file
        // and the first one is not the one that cause the error
        let s = content.find(key).unwrap_or(0);
        let e = content[s..].find('\n').unwrap_or(content.len() - s) + s;

        Self::ParseSymlinkWrongType {
            file_name: file,
            file: content,
            wrong_bit: (s..e).into(),
            reason,
            advice,
        }
    }
}

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
            toml::Value::Table(table) => env.push(EnvType::Grouped(table_to_grouped(file, k.to_owned(), table.to_owned())?)),
            toml::Value::String(string) => env.push(EnvType::Alone(Symlink {
                path: PathBuf::from(string),
                target: PathBuf::from(k),
            })),
            _ => {
                return Err(ParseError::wrong_type(
                    file.to_string(),
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
            update = to_update(file, v)?;
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
                    k,
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
        update: Update::Always,
    })
}

fn to_update(file: &str, value: toml::Value) -> Result<Update, ParseError> {
    const ADVICE: String = format!(
        "the {E}value{D} should represent a {V}valid update frequency{D}
'always' | 'never' | 'optional' | [ 'ListOfComputerNameToAlwaysUpdate', 'OtherComputerName' ]
default: 'always'
{HELP_EXAMPLE}"
    );

    match value {
		toml::Value::String(string) => match Update::from_str(string.as_str()) {
			Ok(update) => Ok(update),
			Err(_) => Err(ParseError::wrong_type(
				file.to_string(),
				string.as_str(),
				format!(
					"{E}value {BE}{string}{D} for {W}key {BW}update{D} is not a {BW}string{D} that represent a {BW}valid update frequency{D}"
				),
				ADVICE,
			))
		},
		toml::Value::Array(array) => {
			let mut name = Vec::new();

			for value in array {
				match value {
					toml::Value::String(string) => name.push(string),
					_ => {
						return Err(ParseError::wrong_type(
							file.to_string(),
							string.as_str(),
							format!(
								"{E}value {BE}{string}{D} for {W}key {BW}update{D} is not a {BW}string{D} that represent a {BW}valid update frequency{D}"
							),
							ADVICE,
						))
					}
				}
			}
		}
		_ => {
			return Err(ParseError::wrong_type(
				file.to_string(),
				string.as_str(),
				format!(
					"{E}value {BE}{string}{D} for {W}key {BW}update{D} is not a {BW}string{D} that represent a {BW}valid update frequency{D}"
				),
				ADVICE,
			))
		}
	}
}
