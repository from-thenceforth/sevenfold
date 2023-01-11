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
