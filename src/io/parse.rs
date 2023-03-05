use crate::escape::{EHS, EHE, W, V, M, D, H, C, YEL, B, BLU};
use crate::io::{URL};
use crate::symlink::{Env, EnvType, Symlink, DEFAULT_SYMLINK_FILE};
use miette::{SourceSpan, Diagnostic};
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
        help("the file {M}{file}{D} should be a {V}valid toml{D} file
you can check the example file {V}{DEFAULT_SYMLINK_FILE}{D} {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}
or the {V}toml{D} documentation {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}")
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
		help("the file {M}{file}{D} should be a {V}valid toml{D} that define the {V}symlink{D}
you can check the example file {V}{DEFAULT_SYMLINK_FILE}{D} {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}")
	)]
    ParseTOMLToEnv {
        #[source_code]
        file: String,
    },

    #[error("{reason}

could not {W}parse {M}{file}{D} to valid list of {W}symlink{D}")]
    #[diagnostic(
		code(parse::value_to_env),
		url("{}{}", URL, file!()),
		help("{advice}

the file {M}{file}{D} should be a {V}valid toml{D} that define the {V}symlink{D}

{H}example:{D}

{C}# optional title{D}
{B}{YEL}[title]{D}
{C}# optional update frequency{D}
{C}# key (string) = value ('always' | 'never' | 'optional' | [ 'ListOfComputerNameToAlwaysUpdate', 'OtherComputerName' ]){D}
update = 'always'
{C}# symlink list{D}
{C}# key (string) = value (string){D}
\"~/path/to/dotfile/where/symlink/will/be\" = \"path/of/actual/dotfile/stored/in/data/directory\"


you can check the example file {V}{DEFAULT_SYMLINK_FILE}{D} {EHS}{URL}{DEFAULT_SYMLINK_FILE}{EHE}")
	)]
    ParseSymlinkWrongType {
        #[source_code]
        file_name: String,
		#[source_code]
		file: String,
		#[label]
		wrong_bit: SourceSpan,

        reason: String,
        advice: String,

		pub fn new(file: String, key: &str, reason: String, advice: String) -> Self {
			let content = fs::read_to_string(&file).unwrap_or_else(|_| String::from("ENABLE TO READ FILE\n"));
		
			// will not work if the key is repeated in the file
			// and the first one is not the one that cause the error
			let s = content.find(key).unwrap_or(0);
			let e = content[s..].find('\n').unwrap_or(content.len());
		
			Self {
				file_name: file,
				file: content,
				wrong_bit: (s..e).into(),
				reason,
				advice,
			}
		}
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
                return Err(ParseError::ParseSymlinkWrongType::new(
					file.to_string(),
					k,
					"{W}value {v}{D} for key {M}{k}{D} is not a {W}string{D} or {W}table{D}".to_string(),
					"the {W}value{D} should either be:
	- a {V}string{D} that represent the path of the actual dotfile
	- a {V}table{D} that represent a list of symlink".to_string(),
				))
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
