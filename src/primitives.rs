// all supported operators
use crate::macros::debug;
debug!(mod InfixOps {
    
    // so the compiler yells at me if i forget to implement something
    // this trait is otherwise unused

    trait InfixOps: 
        // negation
            std::ops::Neg + // [-]
            std::ops::Not + // [!]
        // arithmetic
            std::ops::Add + std::ops::AddAssign + // [+], [+=]
            std::ops::Sub + std::ops::SubAssign + // [-], [-=]
            std::ops::Mul + std::ops::MulAssign + // [*], [*=] 
            std::ops::Rem + std::ops::RemAssign + // [%], [%=]
            std::ops::Div + std::ops::DivAssign + // [/], [/=]
            std::ops::Shl + std::ops::ShlAssign + // [<<], [<<=]
            std::ops::Shr + std::ops::ShrAssign + // [>>], [>>=]
        // equality
            std::cmp::PartialEq + // [==], [!=]
        // ordering
            std::cmp::PartialOrd + // [>], [<], [>=], [<=]
        // Sized is required for other traits
            Sized
    {} // obligatory empty closure

    impl InfixOps for Primitive {};
    prinln!("InfixOps implemented")
});


use std::{convert::Infallible, path::PathBuf};
#[derive(PartialEq, PartialOrd, Ord, Eq)]
enum Primitive {
    // Ordered by parsing complexity
    // RegEx pseudo syntax used
    Bool(bool), // ["true"|"false"]
    Int(usize), // [0-9]+
    Signed(isize), // [+-][0-9]+
    // Match numbers with no leading 0
    // but throw an error (propogate up to the user)
    Float(f64), // [+-]?[0-9]*.[0-9]+
    FilePath(PathBuf), // depends on operating system, use .parse()
    Str(String),
}

enum Warn {
    FormatError {
        err: &str, 
        bypass: T
    }
}
enum Error {
    // Discouraged behavior
    Warn(Warn),
    // Example, floor of a String
    InvalidOperation(&'static str)
}
use Error::*;

impl FromStr for  Primitive {
    // falls back to String
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::primitives::Primitive::*;
        match s {
            // (true|false)
            _ if let Ok(parsed) = s.parse::<bool>() => Ok(Bool(parsed)),
            // [0-9]+
            _ if let Ok(parsed) = s.parse::<usize>() => Ok(Int(parsed)),
            // [-][0-9]+
            _ if let Ok(parsed) = s.parse::<isize>() => Ok(Signed(parsed)),
            // [+-][0-9]*[.][0-9]+
            _ if let Ok(parsed) = s.parse::<f64>() => {
                if matches!(s, ".") {
                    Err(Warn::Warn(Float(parsed)))
                } else {
                    Ok(Float(parsed))
                }
            },
            // platform specific
            _ if let Ok(parsed) = s.parse::<PathBuf>()  => {
                Ok(FilePath(parsed))
            }
            // always parseable
            _ => Ok(Str(s.to_string()))
        }
    }
}

#[cfg(feature = "extended_math")]
impl Primitive {
    // probably implemented as custom commands later
    // all the other operations that aren't infix operations
    fn floor(&self) -> Result<Primitive, Error> {
            use crate::primitives::Primitive::*;
            use crate::primitives::Error;
            use crate::primitives::Warn;
        match self {
            Primitive::Bool(_) => Err(InvalidOperation("Cannot find floor of bool")),
            Primitive::Int(num) => Err(Error(Warn(Warn::FormatError {
                err: "Already an Integer",
                bypass: Int(*num)
            }))),
            Primitive::Signed(num) => Error(Warn::FormatError {
                err: "Already a signed number",
                bypass: Signed(*num)
            }),
            Primitive::Float(num) => Ok(Float(num.floor())),
            Primitive::FilePath(_) => Err(InvalidOperation("Cannot find floor of string")),
            Primitive::Str(_) => Err(InvalidOperation("Cannot find floor of string")),
        }
    }
    fn ceil(&self) -> Result<Primitive, Error> {
        match self {
            Primitive::Bool(_) => Err(InvalidOperation("Cannot find ceiling of bool")),
            Primitive::Int(num) => Err(Error::Warn::FormatError {
                err: "Already an Integer",
                bypass: Int(*num)
            }),
            Primitive::Signed(num) => Err(Error::Warn::FormatError {
                err: "Already an Integer",
                bypass: Signed(*num)
            }),
            Primitive::Float(num) => Ok(Float(num.ceil())),
            Primitive::FilePath(_) => Err(InvalidOperation("Cannot find ceiling of FilePath")),
            Primitive::Str(_) => Err(InvalidOperation("Cannot find ceiling of string")),
        }
    }
}