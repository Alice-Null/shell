#![warn(clippy::all)]

use std::{convert::Infallible, str::FromStr};
use crate::types::{ShType, ShType::*, /*Error*/};

impl FromStr for ShType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // it's only a string if it can't be anything else
        // since you can losslessly convert between string
        // and other types
        // bool is simplest, so if it can be, it's a bool
        // then integers, then floats
        // since those are pretty unique
        // paths actually accept everything like string
        // those won't be parsed here, but converted from strings
        // later, as needed
        if let Ok(b) = bool::from_str(s) { // bools have no overlap, check first
            Ok(Bool(b))
        } else if let Ok(n) = i64::from_str(s){ // ints are better than floats
            Ok(Int(n))
        } else if let Ok(f) = f64::from_str(s) { // but floats exist too i guess
            Ok(Float(f))
        } else { // any &str can be a String, so if nothing else works it's a string
            Ok(ShString(s.into()))
        } // like strings, anything can be a path, so default to string
        // and become path later, as needed
    }
}

