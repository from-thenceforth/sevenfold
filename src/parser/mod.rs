use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/r7rs.pest"]
pub struct R7RSParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_blank_string() {
        // Programs can't be blank.
        let parsing = R7RSParser::parse(Rule::program, "  ");
        assert!(parsing.is_err());
    }

    #[test]
    fn test_a_literal() {
        let parsing = R7RSParser::parse(Rule::program, "a-symbol");
        assert!(parsing.is_ok());
        let mut parsing = parsing.unwrap();
        assert_eq!("a-symbol", parsing.next().unwrap().as_str());
    }
    #[test]
    fn test_a_form() {
        let parsing = R7RSParser::parse(Rule::program, "(+ 2 2)");
        match parsing {
            Ok(mut parsing) => {
                dbg!(&parsing);
                let form = parsing.next().unwrap();
                assert_eq!("(+ 2 2)", form.as_str());
            }
            Err(e) => {
                println!("{}", e);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_a_program() {
        let parsing = R7RSParser::parse(Rule::program, "(+ 2 2)\n(* 3 3)");
        match parsing {
            Ok(mut parsing) => {
                dbg!(&parsing);
                let form = parsing.next().unwrap();
                assert_eq!("(+ 2 2)\n(* 3 3)", form.as_str());
            }
            Err(e) => {
                println!("{}", e);
                assert!(false);
            }
        }
    }
}
