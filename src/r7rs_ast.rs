use crate::parser::Rule;

pub enum AST {
    Program {
        imports: Vec<Import>,
        cdefs: Vec<CDef>,
    },
}
impl AST {
    fn from(program: pest::iterators::Pair<Rule>) -> AST {
        let mut imports = Vec::new();
        let mut cdefs = Vec::new();
        for pair in program.into_inner() {
            match pair.as_rule() {
                // Rule::import_declaration => {
                //     imports.push(Import::from(pair));
                // }
                Rule::command_or_definition => {
                    cdefs.push(CDef::from(pair));
                }
                _ => unreachable!(),
            }
        }
        AST::Program { imports, cdefs }
    }
}

pub enum CDef {
    Command(Expression),
    Definition,
}
impl CDef {
    fn from(pair: pest::iterators::Pair<Rule>) -> CDef {
        match pair.as_rule() {
            Rule::command_or_definition => {
                let mut inner = pair.into_inner();
                let command_or_definition = inner.next().unwrap();
                match command_or_definition.as_rule() {
                    Rule::command => {
                        let mut inner = command_or_definition.into_inner();
                        let expression = inner.next().unwrap();
                        CDef::Command(Expression::from(expression))
                    }
                    Rule::definition => CDef::Definition,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Literal {
    String(String),
    Bool(bool),
    Char(char),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    ProcedureCall(Operator, Vec<Operand>),
    Lambda,
    Conditional,
    Assignment,
    DerivedExpression,
    MacroUse,
    MacroBlock,
    Includer,
}
impl Expression {
    // Given a pest Pair, return an Expression or Error.
    fn from(pair: pest::iterators::Pair<Rule>) -> Expression {
        match pair.as_rule() {
            Rule::expression => {
                let mut inner = pair.into_inner();
                let expression = inner.next().unwrap();
                dbg!(&expression);
                match expression.as_rule() {
                    Rule::identifier => {
                        Expression::Identifier(expression.as_span().as_str().to_string())
                    }
                    Rule::literal => {
                        // match the types of literals
                        let literal = expression.into_inner().next().unwrap();
                        match literal.as_rule() {
                            Rule::string => Expression::Literal(Literal::String(
                                literal.as_span().as_str().to_string(),
                            )),
                            Rule::boolean => Expression::Literal(Literal::Bool(
                                literal.as_span().as_str().to_string() == "#t",
                            )),
                            // handle character, which has three cases:
                            // any_character, named_character, hex_character
                            Rule::character => {
                                let mut inner = literal.into_inner();
                                let character = inner.next().unwrap();
                                // Each character literal has several styles
                                // any_character prefixed with a #\, e.g. #\a; a named_character, e.g. #\newline; a hex_character code, e.g. #\x0A; a character name, e.g. #\NUL.
                                match character.as_rule() {
                                    // strip the #\ prefix from the any_character
                                    Rule::any_character => {
                                        dbg!(&character.as_span().as_str().chars().nth(2).unwrap());
                                        Expression::Literal(Literal::Char(
                                            character.as_span().as_str().chars().nth(2).unwrap(),
                                        ))
                                    }
                                    Rule::named_character => {
                                        let mut inner = character.into_inner();
                                        let named_character = inner.next().unwrap();
                                        // named characters are defined in the R7RS standard as follows:
                                        // "alarm" | "backspace" | "delete" | "escape" | "newline" | "null" | "return" | "space" | "tab"
                                        // So the names have to be translated from #\escape to #\x1B etc, they're not pest Rules but strings
                                        // to be translated.
                                        match named_character.as_str() {
                                            "alarm" => Expression::Literal(Literal::Char('\x07')),
                                            "backspace" => {
                                                Expression::Literal(Literal::Char('\x08'))
                                            }
                                            "delete" => Expression::Literal(Literal::Char('\x7F')),
                                            "escape" => Expression::Literal(Literal::Char('\x1B')),
                                            "newline" => Expression::Literal(Literal::Char('\x0A')),
                                            "null" => Expression::Literal(Literal::Char('\x00')),
                                            "return" => Expression::Literal(Literal::Char('\x0D')),
                                            "space" => Expression::Literal(Literal::Char('\x20')),
                                            "tab" => Expression::Literal(Literal::Char('\x09')),
                                            _ => unreachable!(),
                                        }
                                    }
                                    Rule::hex_character => {
                                        let mut inner = character.into_inner();
                                        let hex_character = inner.next().unwrap();
                                        Expression::Literal(Literal::Char(
                                            char::from_u32(
                                                u32::from_str_radix(hex_character.as_str(), 16)
                                                    .unwrap(),
                                            )
                                            .unwrap(),
                                        ))
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            /*
                            Rule::character => {
                                let mut inner = literal.into_inner();
                                let character = inner.next().unwrap();
                                // Each character literal has several styles
                                // any_character prefixed with a #\, e.g. #\a; a named_character, e.g. #\newline; a hex_character code, e.g. #\x0A; a character name, e.g. #\NUL.
                                match character.as_rule() {
                                    Rule::any_character => {
                                        Expression::Literal(Literal::Char(
                                            character.as_span().as_str().chars().next().unwrap(),
                                        ))
                                    }
                                    Rule::named_character => {
                                        let mut inner = character.into_inner();
                                        let named_character = inner.next().unwrap();
                                        // named characters are defined in the R7RS standard as follows:
                                        // "alarm" | "backspace" | "delete" | "escape" | "newline" | "null" | "return" | "space" | "tab"
                                        // So the names have to be translated from #\escape to #\x1B etc, they're not pest Rules but strings
                                        // to be translated.
                                        match named_character.as_str() {
                                            "alarm" => Expression::Literal(Literal::Char('\x07')),
                                            "backspace" => Expression::Literal(Literal::Char('\x08')),
                                            "delete" => Expression::Literal(Literal::Char('\x7F')),
                                            "escape" => Expression::Literal(Literal::Char('\x1B')),
                                            "newline" => Expression::Literal(Literal::Char('\x0A')),
                                            "null" => Expression::Literal(Literal::Char('\x00')),
                                            "return" => Expression::Literal(Literal::Char('\x0D')),
                                            "space" => Expression::Literal(Literal::Char('\x20')),
                                            "tab" => Expression::Literal(Literal::Char('\x09')),
                                            _ => unreachable!(),
                                        }

                            }
                            */
                            _ => unreachable!(),
                        }
                    }
                    Rule::procedure_call => {
                        let mut inner = expression.into_inner();
                        let operator = inner.next().unwrap();
                        let operands = inner.collect::<Vec<_>>();
                        Expression::ProcedureCall(
                            Operator(Box::new(Expression::from(operator))),
                            operands
                                .iter()
                                .map(|operand| Operand(Box::new(Expression::from(operand.clone()))))
                                .collect(),
                        )
                    }
                    Rule::lambda_expression => Expression::Lambda,
                    Rule::conditional => Expression::Conditional,
                    Rule::assignment => Expression::Assignment,
                    Rule::derived_expression => Expression::DerivedExpression,
                    Rule::macro_use => Expression::MacroUse,
                    Rule::macro_block => Expression::MacroBlock,
                    Rule::includer => Expression::Includer,
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Expression;
    use crate::parser::R7RSParser as parser;
    use crate::parser::Rule;
    use pest::Parser;
    #[test]
    fn test_expression() {
        let input = "foo";
        let mut pairs = parser::parse(Rule::expression, input).unwrap();
        let pair = pairs.next().unwrap();
        let expression = Expression::from(pair);
        assert_eq!(expression, Expression::Identifier("foo".to_string()));
    }

    #[test]
    fn test_literal_string() {
        let input = "\"foo\"";
        let mut pairs = parser::parse(Rule::expression, input).unwrap();
        let pair = pairs.next().unwrap();
        dbg!(&pair);
        let expression = Expression::from(pair);
        assert_eq!(
            expression,
            Expression::Literal(super::Literal::String("\"foo\"".to_string()))
        );
    }

    #[test]
    fn test_literal_bools_short_or_long_and_true_or_false() {
        let tests = [
            ("#t", true),
            ("#f", false),
            ("#true", true),
            ("#false", false),
        ];
        for (input, expected) in tests.iter() {
            let mut pairs = parser::parse(Rule::expression, input).unwrap();
            let pair = pairs.next().unwrap();
            let expression = Expression::from(pair);
            assert_eq!(
                expression,
                Expression::Literal(super::Literal::Bool(*expected))
            );
        }
    }

    #[test]
    fn test_literal_chars() {
        let tests = [
            ("#\\a", 'a'),
            ("#\\A", 'A'),
            ("#\\space", ' '),
            ("#\\newline", '\n'),
            // hex
            ("#\\x20", ' '),
            ("#\\x0A", '\n'),
            // long hex
            ("#\\x00000020", ' '),
            ("#\\x0000000A", '\n'),
            // some emoji
            ("#\\x1F600", '😀'),
            ("#\\x1F601", '😁'),
            ("#\\x1F602", '😂'),
            ("#\\x1F603", '😃'),
            ("#\\x1F604", '😄'),
            ("#\\x1F605", '😅'),
            ("#\\x1F606", '😆'),
            ("#\\x1F607", '😇'),
            ("#\\x1F608", '😈'),
            ("#\\x1F609", '😉'),
            ("#\\x1F60A", '😊'),
            ("#\\x1F60B", '😋'),
            ("#\\x1F60C", '😌'),
            ("#\\x1F60D", '😍'),
            ("#\\x1F60E", '😎'),
            ("#\\x1F60F", '😏'),
            ("#\\x1F610", '😐'),
            ("#\\x1F611", '😑'),
            ("#\\x1F612", '😒'),
            ("#\\x1F613", '😓'),
            ("#\\x1F614", '😔'),
            ("#\\x1F615", '😕'),
            ("#\\x1F616", '😖'),
            ("#\\x1F617", '😗'),
            ("#\\x1F618", '😘'),
            ("#\\x1F619", '😙'),
            ("#\\x1F61A", '😚'),
            ("#\\x1F61B", '😛'),
            ("#\\x1F61C", '😜'),
            ("#\\x1F61D", '😝'),
            ("#\\x1F61E", '😞'),
            ("#\\x1F61F", '😟'),
            ("#\\x1F620", '😠'),
            ("#\\x1F621", '😡'),
            ("#\\x1F622", '😢'),
            ("#\\x1F623", '😣'),
            ("#\\x1F624", '😤'),
            // Wow copilot that was a lot of emoji
        ];
        for (input, expected) in tests.iter() {
            let mut pairs = parser::parse(Rule::expression, input).unwrap();
            let pair = pairs.next().unwrap();
            let expression = Expression::from(pair);
            assert_eq!(
                expression,
                Expression::Literal(super::Literal::Char(*expected))
            );
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Operator(Box<Expression>);

#[derive(Debug, PartialEq, Eq)]
pub struct Operand(Box<Expression>);
pub enum Import {
    Library {
        name: String,
    },
    Only {
        imports: Vec<Box<Import>>,
        identifiers: Vec<String>,
    },
    Except {
        imports: Vec<Box<Import>>,
        identifiers: Vec<String>,
    },
    Prefix {
        imports: Vec<Box<Import>>,
        prefix: String,
    },
    Rename {
        imports: Vec<Box<Import>>,
        identifiers: Vec<(String, String)>,
    },
}
/*
impl Import {
    /// Given a pest pair, return an Import or Error. The import_set rule gives either:
    /// - library_name
    /// - import_only
    /// - import_except
    /// - import_prefix
    /// - import_rename
    ///

    fn from(pair: pest::iterators::Pair<Rule>) -> Result<Import, String> {
        match pair.as_rule() {
            Rule::import_set => {
                let mut inner = pair.into_inner();
                let import = inner.next().unwrap();
                match import.as_rule() {
                    Rule::library_name => {
                        let mut inner = import.into_inner();
                        let name = inner.next().unwrap().as_str().to_string();
                        Ok(Import::Library { name })
                    }
                    Rule::import_only => {
                        let mut imports = Vec::new();
                        let mut identifiers = Vec::new();
                        for pair in import.into_inner() {
                            match pair.as_rule() {
                                Rule::import_set => {
                                    imports.push(Box::new(Import::from(pair).unwrap()));
                                }
                                Rule::identifier => {
                                    identifiers.push(pair.as_str().to_string());
                                }
                                _ => unreachable!(),
                            }
                        }
                        Ok(Import::Only {
                            imports,
                            identifiers,
                        })
                    }
                    Rule::import_except => {
                        let mut imports = Vec::new();
                        let mut identifiers = Vec::new();
                        for pair in import.into_inner() {
                            match pair.as_rule() {
                                Rule::import_set => {
                                    imports.push(Box::new(Import::from(pair).unwrap()));
                                }
                                Rule::identifier => {
                                    identifiers.push(pair.as_str().to_string());
                                }
                                _ => unreachable!(),
                            }
                        }
                        Ok(Import::Except {
                            imports,
                            identifiers,
                        })
                    }
                    Rule::import_prefix => {
                        let mut imports = Vec::new();
                        let mut prefix = String::new();
                        for pair in import.into_inner() {
                            match pair.as_rule() {
                                Rule::import_set => {
                                    imports.push(Import::from(pair).unwrap());
                                }
                                Rule::identifier => {
                                    prefix = pair.as_str().to_string();
                                }
                                _ => unreachable!(),
                            }
                        }
                        Ok(Import::Prefix { imports, prefix })
                    }
                    Rule::import_rename => {
                        let mut imports = Vec::new();
                        let mut identifiers = Vec::new();
                        for pair in import.into_inner() {
                            match pair.as_rule() {
                                Rule::import_set => {
                                    imports.push(Import::from(pair).unwrap());
                                }
                                Rule::identifier => {
                                    identifiers.push(pair.as_str().to_string());
                                }
                                _ => unreachable!(),
                            }
                        }
                        Ok(Import::Rename {
                            imports,
                            identifiers,
                        })
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}
 */
