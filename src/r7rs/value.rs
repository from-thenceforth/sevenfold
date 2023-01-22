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
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Boolean(b) => write!(
                f,
                "{}",
                match b {
                    true => "#t",
                    false => "#f",
                }
            ),
            Value::Character(c) => write!(f, "#\\{}", c),
            Value::Null => write!(f, "()"),
            Value::Pair(p) => write!(f, "({} . {})", p.car, p.cdr),
            Value::Procedure(_) => write!(f, "#<procedure>"),
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Bytevector(b) => write!(
                f,
                "#u8({})",
                b.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Value::EofObject => write!(f, "#<eof>"),
            Value::Number(n) => write!(f, "{}", n),
            Value::Port(_) => write!(f, "#<port>"),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Vector(v) => write!(
                f,
                "#({})",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
        }
    }
}

pub enum Number {
    Integer(i64),
    Real(f64),
    Rational { numerator: i64, denominator: u64 },
    Complex { real: f64, imaginary: f64 },
}
impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{}", i),
            Number::Real(r) => write!(f, "{}", r),
            Number::Rational {
                numerator,
                denominator,
            } => write!(f, "{}/{}", numerator, denominator),
            Number::Complex { real, imaginary } => {
                if imaginary < &0.0 {
                    write!(f, "{}{}i", real, imaginary)
                } else {
                    write!(f, "{}+{}i", real, imaginary)
                }
            }
        }
    }
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
