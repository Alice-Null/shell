#![warn(clippy::all)]

use std::collections::BTreeMap;
use std::{ffi::OsString, path::PathBuf};
use std::ops::Add as stdAdd;

/// Every valid type
enum Types {
    Bool(bool),
    String(OsString),
    Int(i64),
    Float(f64),
    Path(PathBuf),
    Array(Vec<Types>),
    Dictionary(BTreeMap<OsString, Types>),
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
impl From<Types> for SimpleDiscriminant {
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
    // bool
    Not,
    // more bit operations to come
    // ,,,eventually

    // math
    Negation, // -, -= is Subtraction
    Add, AddAssign, // a + b, a += b
    Sub, SubAssign, // a - b, a -= b
    Mul, MulAssign, // a * b, a *= b
    Div, DivAssign, // a / b, a /= b
    // Collections
    ReadKey, // key is an int in arrays
    Assign, // assign value at a key
    // arrays only, maps don't have a clear next value
    // nor is inserting meaningfully different from assigning
        Append,
        Prepend,
        Insert,
    // more to come eventually, probably
}

#[derive(Copy, Clone)]
enum ErrorVariant {
    // This is gonna be evenwhere lol
    TypeErr,
    // mostly math
    OverflowErr,
    DivideByZeroErr
}
#[derive(Clone)]
struct Error <'a> {
    operator: Operator,
    err_type: ErrorVariant,
    types: Vec<&'a Types>
}

// Boolean operations
// (bitwise operatons, but only bools are supported)
impl Types {
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

// Math functions                                       //
// anything that isn't a float or int throws a typerror //
// only implements things that already exist in rust    //
// mostly because it's exhaustive as far as i can tell  //
impl Types {
    /// Negation. Syntax of `-self`.
    /// Errors for anything other than a float or int.
    /// Mathematically equivalent to `0 - self`. 
    /// (It actually just flips a sign bit).
    /// Errors only when `self` is neither a Float(_) nor Err(_),
    /// returning a basic typerror.
    /// Otherwise infallible
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

    /// Add two numbers and return the fallible output.
    /// Output retains the type of self.
    /// # Example
    /// ```
    /// let int = &Int(983_i64);
    /// let float = &Float(17.0_f64)
    /// 
    /// assert_eq!(add(float, int), &Float(1000.0_f64));
    /// assert_eq!(add(int, float), &Int(1000_i64)));
    /// ```
    /// `add` returns a type error when one or more inputs are not numbers.
    /// A number here is either an `Int` or a `Float`.
    /// # Example
    /// ```
    /// let int = &Int(5_i64);
    /// let string = &String("4.96");
    /// let type_error = Err(
    ///     Error {
    ///         operator: Operator::Add,
    ///         err_type: TypeErr,
    ///         types: vec![int, string]
    ///      }
    /// );
    /// 
    /// assert_eq!(add(int, string), Err(type_error));
    /// ```
    /// When both types are the same (two `Ints` or two `Floats`),
    /// numbers are added saturatingly.
    /// # Example
    /// ```
    /// // Int
    /// let small = &Int(5);
    /// let huge_positive = &Int(i64::MAX - 3);
    /// let huge_negative = &Int(i64::MAX + 3);
    /// 
    /// assert_eq!(add(small, huge_positive), &Int(i64::MAX));
    /// assert_eq!(add(small, huge_negative), &Int(i64::MIN));
    /// 
    /// // Float
    /// let small = &Float(398.1);
    /// let huge_positive = &Float(f64::MAX - 28.19);
    /// let huge_negative = &Float(f64::MIN + 28.19)
    /// assert_eq!(add(small, huge_positive), &Float(f64::MAX));
    /// assert_eq!(add(small, huge_negative), &Float(f64::MIN));
    /// ```
    ///
    /// When adding a Float to an `Int` (`add(Int(_), Float(_)`),
    /// the `Float` is cast to an `Int`, such that;
    /// `f64::NAN` is `0_i64`, anything larger than `i64::MAX` is `i64::MAX`,
    /// and anything less than `i64::MIN` is `i64::MIN`.
    /// Anything else is cast to an `Int`.
    /// # Example
    /// ```
    /// let boring_int = &Int(9);
    /// let very_large_float = &Float(f64::MAX);
    /// let very_small_float = &Float(f64::MIN);
    /// let float_nan = &Float(f64::NAN):
    /// 
    /// assert_eq!(add(boring_int, very_large_float), &Int(i64::MAX));
    /// assert_eq!(add(boring_int, very_small_float), &Int(i64::MIN));
    /// assert_eq!(add(boring_int, float_nan), boring_int));
    /// ```
    /// 
    /// When adding an `Int` to a `Float`, it casts `Int` to `Float`.
    /// Float encompasses all values of `Int`,
    /// although some precision may be lost due to floating point error.
    /// It then performs the same as if given two `Float`s
    pub fn add(&self, other: &Types) -> Result<Self, Error> {
        match (self, other) {
            (Int(left_int), Int(right_int)) => {Ok(
                Int(
                    left_int + right_int
                )
            )},
            (Int(int), Float(float)) => {
                let as_int = *float as i64;
                Ok(
                Int(
                    int + as_int
                )
            )},
            (Float(left_float), Float(right_float)) => {Ok(
                Float(
                    left_float + right_float
                )
            )},
            (Float(float), Int(int)) => {Ok(
                Float(
                    float + *int as f64
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
    /// Assigns the result of `self` + `other` to `self`.
    /// This function will Error only if given a non number type.
    /// A number here is either an `Int` or `Float`.
    /// # Example
    /// ```
    /// let number_one = &Int(1);
    /// let meaning_of_life = &Float(42.42);
    /// let meaning_of_uhhh_something = add_assign(meaning_of_life, number_one);
    /// 
    /// // other remains unchanged
    /// assert_eq!(number_one, Ok(&Int(1)));
    /// // self is updated to the resultant value
    /// assert_eq!(meaning_of_life, Ok(&Float(43.42)));
    /// // the returned value of the function is the same resultant value.
    /// assert_eq!(meaning_of_uhhh_something, meaning_of_life);
    /// ```
    /// 
    /// When given an invalid type, returns a `TypeErr`
    /// containing the given values and operator.
    /// # Example
    /// ```
    /// let moby_dick = String("Call me Ishmael.".to_string());
    /// let and_the_universe = Int(42);
    /// 
    /// let wait_wrong_genre = Error {
    ///     operator: Operator::AddAssign,
    ///     err_type: TypeErr,
    ///     types: vec![moby_dick, and_the_universe]
    /// }:
    /// assert_eq!(add_assign(moby_dick, and_the_universe), Err(wait_wrong_genre)):
    /// ```
    /// For precise behavior of addition, see [Types::add].
    /// Mathematically equivalent to `self = add(self, other)`.
    fn add_assign(&mut self, other: &Types) -> Result<&Self, Error> {
        // propagate errors from the underlying add
        *self = self.add(other)?;
        // return a pointer to the updated value
        Ok(&self)
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
impl Types {
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
                    operator: Operator::ReadKey,
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