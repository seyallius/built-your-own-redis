//! <this file needs docstring! AI, please help me write it too!
//! Wheter top level doc, or values, enums, fns, etc., everything,
//! even private one's!>

use anyhow::Result;

// ----------------------------------- Types, Variables & Constants ----------------------------- //

const SIMPLE: u8 = b'+';
const ERROR: u8 = b'-';
const INTEGER: u8 = b':';
const BULK: u8 = b'$';
const ARRAY: u8 = b'*';

pub(crate) enum Value {
    /// The `+` value.
    Simple(String),
    /// The `-` value.
    Error(String),
    /// The `:` value.
    Integer(i64),
    /// The `$` value. Bulk strings are binary, not UTF-8. Redis keys and values are arbitrary bytes,
    /// that is why the usage of `Vec<u8>`.
    Bulk(Option<Vec<u8>>),
    /// The `*` value.
    Array(Vec<Value>),
}

// ------------------------------------- Public (crate) API ------------------------------------- //

pub(crate) fn parse(input: &[u8]) -> Result<Option<(Value, usize)>> {
    match input[0] {
        BULK => parse_bulk(input),
        SIMPLE => parse_simple(input),
        INTEGER => parse_integer(input),
        ERROR => parse_error(input),
        ARRAY => parse_array(input),
        _ => anyhow::bail!("unknown type byte: {:?}", input[0]),
    }
}

// -------------------------------------- Internal Helpers -------------------------------------- //

fn parse_bulk(_input: &[u8]) -> Result<Option<(Value, usize)>> {
    unimplemented!("implement me!")
}

fn parse_simple(_input: &[u8]) -> Result<Option<(Value, usize)>> {
    unimplemented!("implement me!")
}

fn parse_integer(_input: &[u8]) -> Result<Option<(Value, usize)>> {
    unimplemented!("implement me!")
}

fn parse_error(_input: &[u8]) -> Result<Option<(Value, usize)>> {
    unimplemented!("implement me!")
}

fn parse_array(_input: &[u8]) -> Result<Option<(Value, usize)>> {
    unimplemented!("implement me!")
}
