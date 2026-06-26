use std::fmt;

#[derive(Debug)]
pub enum NautileError {
    Parse(String),
    Network(String),
    Dom(String),
    Js(String),
    Layout(String),
    Io(std::io::Error),
}

impl fmt::Display for NautileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(msg)   => write!(f, "parse error: {msg}"),
            Self::Network(msg) => write!(f, "network error: {msg}"),
            Self::Dom(msg)     => write!(f, "dom error: {msg}"),
            Self::Js(msg)      => write!(f, "js error: {msg}"),
            Self::Layout(msg)  => write!(f, "layout error: {msg}"),
            Self::Io(e)        => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for NautileError {}

pub type NautileResult<T> = Result<T, NautileError>;
