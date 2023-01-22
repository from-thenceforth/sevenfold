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
            Value::Procedure(p) => write!(f, "{}", p),
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
/// The Display trait is used to print the contents of a Procedure struct.
impl std::fmt::Display for Procedure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = "    ";
        write!(
            f,
            "(lambda ({params})\n{indent}{body})",
            params = self.params.join(" "),
            indent = indent,
            body = self
                .body
                .iter()
                .map(|v| format!("{val}\n{indent}", indent = indent, val = v).to_owned())
                .collect::<String>()
        )
    }
}
impl std::fmt::Debug for Procedure {
    // Similar to Display but we show the environments.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = "    ";
        write!(
            f,
            "(lambda ({params})\n{envs}\n{indent}{body})",
            params = self.params.join(" "),
            indent = indent,
            envs = self
                .env
                .keys()
                .iter()
                .map(
                    |k| format!("; {key}: {val}", key = k, val = self.env.get(k).unwrap())
                        .to_owned()
                )
                .collect::<Vec<String>>()
                .join(format!("\n{indent}", indent = indent).as_str()),
            body = self
                .body
                .iter()
                .map(|v| format!("{val}\n{indent}", indent = indent, val = v).to_owned())
                .collect::<String>(),
        )
    }
}

pub struct Pair {
    car: Value,
    cdr: Value,
}
impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} . {})", self.car, self.cdr)
    }
}

pub struct Port;
impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#<port>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Value::Boolean(true).to_string(), "#t");
        assert_eq!(Value::Boolean(false).to_string(), "#f");
        assert_eq!(Value::Character('a').to_string(), "#\\a");
        assert_eq!(Value::Null.to_string(), "()");
        assert_eq!(
            Value::Pair(Box::new(Pair {
                car: Value::Boolean(true),
                cdr: Value::Boolean(false)
            }))
            .to_string(),
            "(#t . #f)"
        );
        assert_eq!(
            Value::Procedure(Box::new(Procedure {
                env: Environment::new(),
                params: vec!["a".to_string(), "b".to_string()],
                body: vec![
                    Box::new(Value::Boolean(true)),
                    Box::new(Value::Boolean(false))
                ]
            }))
            .to_string(),
            "(lambda (a b)\n    #t\n    #f\n    )"
        );
        assert_eq!(Value::Symbol("a".to_string()).to_string(), "a");
        assert_eq!(Value::Bytevector(vec![1, 2, 3]).to_string(), "#u8(1 2 3)");
        assert_eq!(Value::EofObject.to_string(), "#<eof>");
        assert_eq!(Value::Number(Number::Integer(1)).to_string(), "1");
        assert_eq!(Value::Number(Number::Real(1.0)).to_string(), "1");
        assert_eq!(Value::Number(Number::Real(1.23)).to_string(), "1.23");
        assert_eq!(
            Value::Number(Number::Rational {
                numerator: 1,
                denominator: 2
            })
            .to_string(),
            "1/2"
        );
        assert_eq!(
            Value::Number(Number::Rational {
                numerator: -1,
                denominator: 2
            })
            .to_string(),
            "-1/2"
        );
        assert_eq!(
            Value::Number(Number::Complex {
                real: 1.0,
                imaginary: 2.0
            })
            .to_string(),
            "1+2i"
        );
        assert_eq!(
            Value::Number(Number::Complex {
                real: 1.0,
                imaginary: -2.0
            })
            .to_string(),
            "1-2i"
        );
        assert_eq!(Value::Port(Box::new(Port)).to_string(), "#<port>");
        assert_eq!(Value::String("a".to_string()).to_string(), "\"a\"");
        assert_eq!(
            Value::Vector(vec![Value::Boolean(true), Value::Boolean(false)]).to_string(),
            "#(#t #f)"
        );
    }
}
