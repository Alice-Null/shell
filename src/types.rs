#![warn(clippy::all)]

use std::collections::BTreeMap;
use std::{ffi::OsString, path::PathBuf};
use std::ops::Add as stdAdd;

/// Every valid type
enum Types <'a> {
    Bool(bool),
    String(OsString),
    Int(i64),
    Float(f64),
    Path(PathBuf),
    Array(Vec<&'a Types<'a>>),
    Dictionary(BTreeMap<OsString, Types<'a>>),
}
use Types::*;
use ErrorVariant::*;
/// Every valid type, without data 
#[derive(Clone, Copy)]
enum SimpleDiscriminant {
    Bool,
    String,
    Int,
    Float,
    Path,
    Array,
    Dictionary
}
// for when the actual value isn't important
impl From<Types <'_>> for SimpleDiscriminant {
    fn from(value: Types) -> Self {
        match value {
            Bool(_) => SimpleDiscriminant::Bool,
            String(_) => SimpleDiscriminant::String,
            Int(_) => SimpleDiscriminant::Int,
            Float(_) => SimpleDiscriminant::Float,
            Path(_) => SimpleDiscriminant::Path,
            Array(_) => SimpleDiscriminant::Array,
            Dictionary(_) => SimpleDiscriminant::Dictionary
            
        }
    }
}

/// every possible operation
/// mostly for error propogation
#[derive(Clone, Copy)]
enum Operator {
    // typecasting
    FloatCastToInt,
    // basically everything
    Assign,
    // bool
    Not,
    // math
    Negation,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    // Array
    ReadIndex,
    AssignIndex,
    Append,
    Prepend,
    Insert,
    // more to come eventually
}

#[derive(Copy, Clone)]
enum ErrorVariant {
    // very common error
    TypeErr,
    // mostly math
    OverflowErr,
    NanErr,
    DivideByZeroErr
}
#[derive(Clone)]
struct Error <'a> {
    operator: Operator,
    err_type: ErrorVariant,
    types: Vec<&'a Types<'a>>
}

// Boolean operations
// (bitwise operatons, but only bools are supported)
impl Types <'_> {
    // !self
    // true -> false; false -> true
    // only works on booleans
    // bitwise operations not supported for anything else
    fn not(&self) -> Result<Self, Error> {
        match self {
            Bool(boolean) => {Ok(Bool(!boolean))},
            _ => {Err(
                Error {
                    operator: Operator::Not,
                    err_type: TypeErr,
                    types: vec![self]
                })
            }
        }
    }
}

fn float_to_int_or_err<'a>(float: f64) -> Result<i64, Error<'a>> {
    if i64::MIN as f64 <= float && float <= i64::MAX as f64 && !float.is_nan() {
        unsafe {
            // SAFETY
            // float is within bounds of i64 & is a number
            Ok(float.to_int_unchecked())
        }
    } else {
        match float.signum() {
            1.0 => {Ok(i64::MAX)},
            -1.0 => {Ok(i64::MIN)},
            _ => {
                // assuming signum docs are correct
                // at this point, must be NaN
                Err(Error{
                    operator: Operator::FloatCastToInt,
                    err_type: NanErr,
                    types: vec![&Float(float)],
                })
            }
        }
    }        
}
// Math functions                                       //
// anything that isn't a float or int throws a typerror //
// only implements things that already exist in rust    //
// mostly because it's exhaustive as far as i can tell  //
impl Types <'_> {
    // floats being NaN or 
    // negationm
    // -self
    // errors for anything other than a float or int
    // equivalent to `0 - number`
    // if self isn't a number, return a TypeError
    // otherwise infallible
    fn negative(&self) -> Result<Self, Error> {
        match self {
            Int(num) => {Ok(Types::Int(-num))},
            Float(num) => {Ok(Types::Float(-num))},
            _ => {Err(
                Error {
                    operator: Operator::Negation,
                    err_type: TypeErr,
                    types: vec![self]
                })
            }
        }
    }

    // distinct lack of negative_assign
    // syntax would logically be `-=`which is assigned to
    // the much more important subtract_assign

    ////////////////////////////////////////////////////////
    // the four most simple math operations (+, -, *, / ) //
    // they all have a type that returns the result and   //
    // a type that sets self to the result                //
    ////////////////////////////////////////////////////////

    // compare types of two inputs.
    // if they can be added, add them together 
    // retain type of self
    // syntax is `self + other`
    pub fn add(&self, other: &Types) -> Result<Self, Error> {
        match (self, other) {
            (Int(left_int), Int(right_int)) => {Ok(
                Int(
                    left_int + right_int
                )
            )},
            (Int(int), Float(float)) => {
                let float_as_int = float_to_int_or_err(*float)?; // have to make function to try and turn a float into an int, and throw the right error if not
                // like the custom error type and all
                Ok(
                Int(
                    int + float_as_int
                )
            )},
            (Float(left_float), Float(right_float)) => {Ok(
                Float(
                    left_float + right_float
                )
            )},
            (Float(float), Int(int)) => {Ok(
                Float(
                    float + int.into()
                )
            )},
            _ => {Err(
                Error {
                    operator: Operator::Add,
                    err_type: TypeErr,
                    types: vec![self, other]
                })
            }
        }
    }
    // syntax is `self += other`
    // same as add, but assigns to self after calculation 
    // returns a result, error has information, but Ok is always None
    // it could be Option<Error>, but Result has nicer syntax
    // (most notably control flow operator `?`)
    fn add_assign(&mut self, other: &Types) -> Result<_, Error> {
        *self = self.add(other)?;
        Ok(None)
    }

    // compare types of two inputs.
    // if they can be subtracted, subtract left by right
    // retain type of self
    // syntax is `self - other`
    fn sub(&self, other: &Types) -> Result<Self, Error> {
        match (self, other) {
            (Int(left_int), Int(right_int)) => {Ok(
                Int(
                    left_int - right_int
                )
            )},
            (Int(int), Float(float)) => {Ok(
                Int(
                    int - float.into()
                )
            )},
            (Float(left_float), Float(right_float)) => {Ok(
                Float(
                    left_float - right_float
                )
            )},
            (Float(float), Int(int)) => {Ok(
                Float(
                    float - int.into()
                )
            )},
            _ => {Err(
                Error {
                    operator: Operator::Sub,
                    err_type: TypeErr, 
                    types: vec![self, other]
                })
             }
         }
    }
    // self -= other
    // same as above, but assigns to self after calculation
    // returns a result, error has information, but Ok is always None
    // it could be Option<Error>, but Result has nicer syntax
    // (most notably control flow operator `?`)
    fn sub_assign(&mut self, other: &Types) -> Result<_, Error> {
        *self = self.sub(other)?;
        Ok(None)
    }
    
    // division has to fail on 0
    // type checking is probably going to fail more often than a divide by 0
    // so making a function that 
    // compare types of two inputs.
    // if they can be divided, divide self by other 
    // retain type of self
    // self / other
    fn div(&self, other: &Types) -> Result<Self, Error> {
        match other {
            Int(0) => {
                return Err(
                    Error {
                        err_type: DivideByZeroErr,
                        operator: Operator::Div,
                        types: vec![self, other],
                    }
                )
            },
            Float(0.0) => {
                return Err(Error{
                    operator: todo!(),
                    err_type: todo!(),
                    types: todo!(),
                })
            },
            _ => {
                // usually 0 isn't involved
                // so just continue on if it isn't
            }
        }
        match (self, other) {
            (Int(left_int), Int(right_int)) => {Ok(
                Int(
                    left_int / right_int
                )
            )},
            (Int(int), Float(float)) => {Ok(
                Int(
                    int / float.into()
                )
            )},
            (Float(left_float), Float(right_float)) => {Ok(
                Float(
                    left_float / right_float
                )
            )},
            (Float(float), Int(int)) => {Ok(
                Float(
                    float / int.into()
                )
            )},
            _ => {Err(
                Error {
                    operator: Operator::Div,
                    err_type: TypeErr,
                    types: vec![self, other]
                })
            }
        }
    }
    // self /= other
    // same as above, but assigns to self after calculation 
    // returns a result, error has information, but Ok is always None
    // it could be Option<Error>, but Result has nicer syntax
    // (most notably control flow operator `?`)
    fn div_assign(&mut self, other: &Types) -> Result<_, Error> {
        *self = self.div(other)?;
        Ok(None)
    }

    // compare types of two inputs.
    // if they can be muled, mul them together 
    // retain type of self
    // self * other
    fn mul(&self, other: &Types) -> Result<Self, Error> {
        match (self, other) {
            (Int(left_int), Int(right_int)) => {Ok(
                Int(
                    left_int * right_int
                )
            )},
            (Int(int), Float(float)) => {Ok(
                Int(
                    int * float.into()
                )
            )},
            (Float(left_float), Float(right_float)) => {Ok(
                Float(
                    left_float * right_float
                )
            )},
            (Float(float), Int(int)) => {Ok(
                Float(
                    float * int.into()
                )
            )},
            _ => {Err(
                Error {
                    operator: Operator::Mul,
                    err_type: TypeErr,
                    types: vec![self, other],
                })
            }
        }
    }
    // self *= other
    // same as above, but assigns to self after calculation
    // returns a result, error has information, but Ok is always None
    // it could be Option<Error>, but Result has nicer syntax
    // (most notably control flow operator `?`)
    fn mul_assign(&mut self, other: &Types) -> Result<_, Error> {
        *self = self.mul(other)?;
        Ok(None)
    }
}

// Functions for arrays & dictionaries
impl Types <'_> {
    // get value at index or key
    // value can be any variable
    fn read_index(&self, index: &Types) -> Result<Types, Error> {   
        match (self, index) {
            // standard array
            (Array(array), Int(index)) => {
                Ok(
                    array[index]
                )
            },
            // dictionary
            (Dictionary(dict), String(key)) => {
                Ok(
                    dict[key]
                )
            },
            _ => {
                Err(Error{
                    operator: Operator::ReadIndex,
                    err_type: TypeErr,
                    types: vec![self, index],
                })
            }
        } 
        // { // index has to be an int
        //     return Err(Error {
        //         operator: ReadIndex,
        //         err_type: TypeErr,
        //         types: vec![self, &index]
        //     });
        // }
    }
}