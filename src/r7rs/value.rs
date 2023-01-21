use crate::r7rs::env::Environment;

/// The Value enum represents all possible values in the R7RS Scheme language.
/// - boolean
/// - character
/// - null
/// - pair
/// - procedure
/// - symbol
/// - bytevector
/// - eof-object
/// - number
/// - port
/// - string
/// - vector
pub enum Value {
    Boolean(bool),
    Character(char),
    Null,
    Pair(Box<Pair>),
    Procedure(Box<Procedure>),
    Symbol(String),
    Bytevector(Vec<u8>),
    EofObject,
    Number(Number),
    Port(Box<Port>),
    String(String),
    Vector(Vec<Value>),
}

pub enum Number {
    Integer(i64),
    Real(f64),
    Rational { numerator: i64, denominator: i64 },
    Complex { real: f64, imaginary: f64 },
}
pub struct Procedure {
    env: Environment,
    params: Vec<String>,
    body: Vec<Box<Value>>,
}

pub struct Pair {
    car: Value,
    cdr: Value,
}

pub struct Port;
