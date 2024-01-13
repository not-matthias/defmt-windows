use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;
use defmt_parser::Level;

pub const DEFMT_VERSIONS: &[&str] = &["3", "4"];

/// Specifies the origin of a format string
#[derive(PartialEq, Eq, Debug)]
pub enum Tag {
    /// Defmt-controlled format string for primitive types.
    Prim,
    /// Format string created by `#[derive(Format)]`.
    Derived,
    /// Format string created by `defmt::bitflags!`.
    Bitflags,
    /// A user-defined format string from a `write!` invocation.
    Write,
    /// An interned string, for use with `{=istr}`.
    Str,
    /// Defines the global timestamp format.
    Timestamp,

    /// `static` containing a possible value of a bitflags type.
    BitflagsValue,
    /// Format string created by `defmt::println!`.
    Println,

    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Tag {
    fn to_level(&self) -> Option<Level> {
        match self {
            Tag::Trace => Some(Level::Trace),
            Tag::Debug => Some(Level::Debug),
            Tag::Info => Some(Level::Info),
            Tag::Warn => Some(Level::Warn),
            Tag::Error => Some(Level::Error),
            _ => None,
        }
    }
}

/// Entry in [`Table`] combining a format string with its raw symbol
#[derive(Debug, Eq, PartialEq)]
pub struct TableEntry {
    string: StringEntry,
    raw_symbol: String,
}

impl TableEntry {
    pub fn new(string: StringEntry, raw_symbol: String) -> Self {
        Self { string, raw_symbol }
    }

    #[cfg(test)]
    fn new_without_symbol(tag: Tag, string: String) -> Self {
        Self {
            string: StringEntry::new(tag, string),
            raw_symbol: "<unknown>".to_string(),
        }
    }
}

/// A format string and it's [`Tag`]
#[derive(Debug, Eq, PartialEq)]
pub struct StringEntry {
    tag: Tag,
    string: String,
}

impl StringEntry {
    pub fn new(tag: Tag, string: String) -> Self {
        Self { tag, string }
    }
}

/// Data that uniquely identifies a `defmt::bitflags!` invocation.
#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct BitflagsKey {
    /// Name of the bitflags struct (this is really redundant with `disambig`).
    pub(crate) ident: String,
    pub(crate) package: String,
    pub(crate) disambig: String,
    pub(crate) crate_name: Option<String>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Encoding {
    Raw,
    Rzcobs,
}

impl FromStr for Encoding {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "raw" => Ok(Encoding::Raw),
            "rzcobs" => Ok(Encoding::Rzcobs),
            _ => anyhow::bail!("Unknown defmt encoding '{}' specified. This is a bug.", s),
        }
    }
}

impl Encoding {
    pub const fn can_recover(&self) -> bool {
        match self {
            Encoding::Raw => false,
            Encoding::Rzcobs => true,
        }
    }
}

/// Internal table that holds log levels and maps format strings to indices
#[derive(Debug, Eq, PartialEq)]
pub struct Table {
    pub(crate) timestamp: Option<TableEntry>,
    pub(crate) entries: BTreeMap<usize, TableEntry>,
    pub(crate) bitflags: HashMap<BitflagsKey, Vec<(String, u128)>>,
    pub(crate) encoding: Encoding,
}
