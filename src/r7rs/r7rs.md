use std::str::FromStr;

grammar;

pub Term: i32 = {
    <n:Num> => n,
    "(" <t:Term> ")" => t,
};

Num: i32 = <s:r"[0-9]+"> => i32::from_str(s).unwrap();

r7rs.lalrpop

/*
# R7RS BNF Grammar

*/
const S7F_EBNF: &str = r#"
  <token> := <identifier> | <boolean> | <number> | <character> | <string> |
    "("
    | ")"
    | '#('
    | '#u8('
    | "'"
    | "`"
    | ","
    | ",@"
    | . ;
   <delimiter> := <whitespace> | <vertical-line> | "(" | ")" | '"' | ';' ;
   <intraline whitespace> := " " | "\t" ;
   <whitespace> := <intraline whitespace> | <line-ending> ;
    <line-ending> := <newline> | <return> | <return> <newline> ;
    <newline> := "\n" ;
    <return> := "\r" ;
    <comment> := ";" <char>* <line-ending>
        | <nested-comment>
        | '#;' <intertoken space> <datum> ;
    <intertoken space> := <atomosphere>* ;
    <atomosphere> := <whitespace> | <comment> | <directive> ;
    <directive> := '#!fold-case' | '#!no-fold-case' ;
    <nested-comment> := '#|' <comment text> <comment cont>* "|#" ;
    <comment text> := <char>* - ('#|' | '|#') ;
    <comment cont> := <nested comment> <comment text> ;
    <vertical line> := "|" ;
    <char> := /./ ;
    <identifier> := <initial> <subsequent>* 
        | <vertical line> <symbol element>* <vertical line> ;
        | <peculiar identifier> ;
    <initial> := <letter> | <special initial> ;
    <letter> := /[a-zA-Z]/ ;
    <subsequent> := <initial> | <digit> | <special subsequent> ;
    <special initial> := /[#!$%&*\/:<=>\?\^_~]/ ;
    <digit> := /[0-9]/ ;
    <hex digit> := <digit> | /[a-fA-F]/ ;
    <explicit sign> := "+" | "-" ;
    <special subsequent> := <explicit sign> | /[.@]/ ;
    <inline hex escape> := "\x" <hex scalar value> ";" ; # TODO make this more specific for 2-4 hex digits
    <hex scalar value> := <hex digit>+ ;
    <mnemonic escape> := "\a" | "\b" | "\t" | "\n" | "\r" ;
    <peculiar identifier> := <explicit sign>
        | <explicit sign> <sign subsequent> <subsequent>*
        | <explicit sign> "." <dot subsequent> <subsequent>*
        | "." <dot subsequent> <subsequent>* ;
    <dot subsequent> := <sign subsequent> | "."  ;
    <sign subsequent> := <initial> | <explicit sign> | "@" ;
    <symbol element> := <char>* - (<vertical line> | "\") 
    | <inline hex escape>
    | <mnemonic escape>
    | "\|"
    ;
    <boolean> := '#t' | '#f' | '#true' | '#false' ;
    <character> := '#\\' <char> 
        | '#\\' <character name>
        | '#\x' <hex scalar value> ;
    <character name> := "alarm" | "backspace" | "delete" | "escape" | "newline" | "null" | "return" | "space" | "tab" ;
    <string> := '"' <string element>* '"' ;
    <string element> := <char>* - ('"' | "\") 
    | <inline hex escape> 
    | <mnemonic escape> | "\"" | "\\" | "\|" 
    | "\" <intraline whitespace>* <line-ending> <intraline whitespace>*
    ;
    <bytevector> := '#u8(' <byte>* ')' ;
    <byte> := <hex scalar value> | <binary scalar value> ; # TODO make this more specific integers 0..255
    <number> := <num 2> | <num 8> | <num 10> | <num 16> ;
    <num 2> := <prefix 2> <complex 2>    
    <complex 2> := <real 2> 
    | <real 2> "@" <real 2> 
    | <real 2> "+" <ureal 2> "i"
    | <real 2> "-" <ureal 2> "i"
    | <real 2> "+" "i"
    | <real 2> "-" "i"
    | <real 2> <infnan> "i"
    | "+" <ureal 2> "i"
    | "-" <ureal 2> "i"
    | <infan> "i"
    | "+i"
    | "-i"  
    ;
    <real 2> := <sign> <ureal 2>  | <infan> ;
    <ureal 2> := <uinteger 2>
    | <uinteger 2> "/" <uinteger 2>
    | <decimal 2>
    ;
"#;

/// The individual characters that can be used in a S7F identifier
const S7F_IDENTIFIER_CHARACTERS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!$%&*+-./:<=>?@^_~";
const S7F_INITIAL_IDENTIFIER_CHARACTERS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const S7F_DIGITS: &str = "0123456789";
const S7F_EXPLICIT_SIGN: &str = "+-";
const S7F_SPECIAL_SUBSEQUENT_IDENTIFIER_CHARACTERS: &str = "+-.@";
const S7F_SPECIAL_INITIAL_IDENTIFIER_CHARACTERS: &str = "!$%&*+-./:<=>?@^_~";
// Create a Rust regex with the escaped characters above
const S7F_IDENTIFIER_REGEX: &str = r"[a-zA-Z0-9!$%&*+-\./:<=>?@^_~]+";
const S7F_VERTICAL_CHARACTER_IDENTIFIER_REGEX: &str = r"\|[^\\\|]+\|";

struct S7FMachine {
    stack: Vec<S7FCell>,
    memory: Vec<S7FCell>,
    program_counter: usize,
}

enum S7FAST {
    Empty,
    Period,
    Boolean(bool),
    Comment(String),
    String(String),
    Character(S7FCharacter),
    Comma,
    CommaAt,
    Backtick,
    ParenthesesOpen,
    ParenthesesClose,
    VectorStart,
    ByteVectorStart,
    SingleQuote,
    Number(NumberTower),
    Identifier(String),
}
enum NumberTower {
    Number2(S7FNumberExactness, String),
}
enum S7FCharacter {
    Character(char),
    CharacterName(S7FNamedCharacters),
    CharacterHex(String),
}
enum S7FNamedCharacters {
    Alarm,
    Backspace,
    Delete,
    Escape,
    Newline,
    Null,
    Return,
    Space,
    Tab,
}
enum S7FNumberExactness {
    Exact,
    Inexact,
    Unspecified,
}
struct S7FParser {}
impl S7FParser {
    fn new() -> Self {
        Self {}
    }
    pub fn parse(&self, input: &str) -> Result<S7FAST, String> {
        match Self::parse_string(input) {
            Ok((_, s)) => Ok(s),
            Err(e) => Err(format!("Error: {:?}", e)),
        }
    }

    /// Parse a Scheme token using nom
    pub fn parse_token(input: &str) -> IResult<&str, S7FAST> {
        let (input, a) = alt((
            Self::parse_identifier,
            Self::parse_boolean,
            Self::parse_number,
            Self::parse_character,
            Self::parse_string,
            Self::parse_parentheses_open,
            Self::parse_parentheses_close,
            Self::parse_vector_start,
            Self::parse_byte_vector_start,
            Self::parse_single_quote,
            Self::parse_back_quote,
            Self::parse_comma,
            Self::parse_comma_at,
            Self::parse_period,
        ))(input)?;
        Ok((input, S7FAST::Empty))
    }
    /// Parse a Scheme vector start using nom
    pub fn parse_vector_start(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag("#(")(input)?;
        Ok((input, S7FAST::VectorStart))
    }
    /// Parse a Scheme byte vector start using nom
    pub fn parse_byte_vector_start(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag("#u8(")(input)?;
        Ok((input, S7FAST::ByteVectorStart))
    }
    /// Parse a Scheme single quote using nom
    pub fn parse_single_quote(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag("'")(input)?;
        Ok((input, S7FAST::SingleQuote))
    }
    /// Parse a '(' using nom
    pub fn parse_parentheses_open(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag("(")(input)?;
        Ok((input, S7FAST::ParenthesesOpen))
    }
    /// Parse a ')' using nom
    pub fn parse_parentheses_close(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag(")")(input)?;
        Ok((input, S7FAST::ParenthesesClose))
    }
    /// Parse a '.' using nom
    pub fn parse_period(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag(".")(input)?;
        Ok((input, S7FAST::Period))
    }
    /// Parse a Scheme boolean from [#t #f #true #false] using nom
    pub fn parse_boolean(input: &str) -> IResult<&str, S7FAST> {
        let (input, boolean) = alt((tag("#t"), tag("#f"), tag("#true"), tag("#false")))(input)?;
        Ok((
            input,
            S7FAST::Boolean(match boolean {
                "#t" | "#true" => true,
                "#f" | "#false" => false,
                _ => unreachable!(),
            }),
        ))
    }
    /// Parse a Scheme character using nom
    fn parse_character(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag("#\\")(input)?;
        let (input, character) = alt((
            Self::parse_character_name,
            Self::parse_character_hex,
            Self::parse_character_character,
        ))(input)?;
        Ok((input, S7FAST::Character(character)))
    }
    /// Parse a Scheme character name which is one of [alarm backspace delete escape newline null return space tab] using nom
    fn parse_character_name(input: &str) -> IResult<&str, S7FCharacter> {
        let (input, character) = alt((
            tag("alarm"),
            tag("backspace"),
            tag("delete"),
            tag("escape"),
            tag("newline"),
            tag("null"),
            tag("return"),
            tag("space"),
            tag("tab"),
        ))(input)?;
        Ok((
            input,
            S7FCharacter::CharacterName(match character {
                "alarm" => S7FNamedCharacters::Alarm,
                "backspace" => S7FNamedCharacters::Backspace,
                "delete" => S7FNamedCharacters::Delete,
                "escape" => S7FNamedCharacters::Escape,
                "newline" => S7FNamedCharacters::Newline,
                "null" => S7FNamedCharacters::Null,
                "return" => S7FNamedCharacters::Return,
                "space" => S7FNamedCharacters::Space,
                "tab" => S7FNamedCharacters::Tab,
                _ => unreachable!(),
            }),
        ))
    }
    /// Parse a Scheme character hex using nom
    /// #\\x[0-9a-fA-F]+
    pub fn parse_character_hex(input: &str) -> IResult<&str, S7FCharacter> {
        let (input, _) = tag("x")(input)?;
        let (input, hex) = take_while(|c: char| c.is_ascii_hexdigit())(input)?;
        Ok((input, S7FCharacter::CharacterHex(hex.to_string())))
    }
    /// Parse a Scheme character taking just one character using nom
    pub fn parse_character_character(input: &str) -> IResult<&str, S7FCharacter> {
        let (input, character) = take_till1(|c: char| true)(input)?;
        Ok((
            input,
            S7FCharacter::Character(character.chars().nth(0).unwrap()),
        ))
    }
    /// Parse a Scheme comma using nom
    pub fn parse_comma(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag(",")(input)?;
        Ok((input, S7FAST::Comma))
    }
    /// Parse a Scheme comma at using nom
    /// ,@
    pub fn parse_comma_at(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag(",@")(input)?;
        Ok((input, S7FAST::CommaAt))
    }
    /// Parse a Scheme line comment using nom
    pub fn parse_line_comment(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag(";")(input)?;
        let (input, content) = take_while(|c| c != '\n')(input)?;
        Ok((input, S7FAST::Comment(content.to_string())))
    }
    /// Parse a string into a S7FCell using nom
    pub fn parse_string(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag("\"")(input)?;
        let (input, string) = take_while(|c| c != '"')(input)?;
        let (input, _) = tag("\"")(input)?;
        Ok((input, S7FAST::String(string.to_string())))
    }
    /// Parse a backtick using nom
    pub fn parse_back_quote(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag("`")(input)?;
        Ok((input, S7FAST::Backtick))
    }

    /// Parse a Scheme nubmer for 2 8 10 and 16 bases using nom
    pub fn parse_number(input: &str) -> IResult<&str, S7FAST> {
        let (input, number) = alt((
            Self::parse_number_2,
            //Self::parse_number_8,
            //Self::parse_number_10,
            //Self::parse_number_16,
        ))(input)?;
        // output the S7FAST Number with the NumberTower
        Ok((input, S7FAST::Number(number)))
    }
    /// Parse a Scheme number in base 2 using nom for using #b prefix and 0 and 1 characters
    /// The marker can indicate exactness or inexactness in either order and is optional
    /// For example: #i#b1010 or #e#b1010 or #b1010 or #b#i1010 or #b#e1010
    pub fn parse_number_2(input: &str) -> IResult<&str, NumberTower> {
        let (input, exactness_prefix) = opt(Self::parse_exactness)(input)?;
        let (input, _) = tag("#b")(input)?;
        let (input, exactness_suffix) = opt(Self::parse_exactness)(input)?;
        let (input, number) = take_while(|c| c == '0' || c == '1')(input)?;
        Ok((
            input,
            NumberTower::Number2(
                exactness_prefix
                    .unwrap_or(exactness_suffix.unwrap_or(S7FNumberExactness::Unspecified)),
                number.to_string(),
            ),
        ))
    }
    /// Parse Scheme exactness using nom for #i and #e
    pub fn parse_exactness(input: &str) -> IResult<&str, S7FNumberExactness> {
        let (input, exactness) = alt((tag("#i"), tag("#e")))(input)?;
        Ok((
            input,
            match exactness {
                "#i" => S7FNumberExactness::Inexact,
                "#e" => S7FNumberExactness::Exact,
                _ => unreachable!(),
            },
        ))
    }
    /// Parse a Scheme identifier using nom supporting two syntaxes:
    /// a. standard
    /// b. pipe-delimited
    pub fn parse_identifier(input: &str) -> IResult<&str, S7FAST> {
        let (input, identifier) = alt((
            Self::parse_identifier_standard,
            Self::parse_identifier_pipe_delimited,
        ))(input)?;
        Ok((input, identifier))
    }
    /// Parse a Scheme identifier using nom supporting the standard syntax of the S7F_IDENTIFIER_CHARS
    pub fn parse_identifier_standard(input: &str) -> IResult<&str, S7FAST> {
        // get initial character
        let (input, initial) =
            take_till1(|c: char| S7F_INITIAL_IDENTIFIER_CHARACTERS.contains(c))(input)?;
        // get the rest of the identifier which can be the initial, digits and special subsequent characters
        let (input, rest) = take_while(|c: char| {
            S7F_INITIAL_IDENTIFIER_CHARACTERS.contains(c)
                || S7F_SPECIAL_SUBSEQUENT_IDENTIFIER_CHARACTERS.contains(c)
                || c.is_digit(10)
        })(input)?;
        // combine the initial and rest into a single identifier
        let identifier = format!("{}{}", initial, rest);
        Ok((input, S7FAST::Identifier(identifier)))
    }
    /// recognize the initial character of a Scheme identifier
    pub fn recognize_identifier_initial(input: &str) -> IResult<&str, &str> {
        take_till1(|c: char| S7F_IDENTIFIER_CHARACTERS.contains(c))(input)
    }
    /// Parse a Scheme identifier using nom supporting the pipe-delimited syntax of the S7F_IDENTIFIER_CHARACTERS
    pub fn parse_identifier_pipe_delimited(input: &str) -> IResult<&str, S7FAST> {
        let (input, _) = tag("|")(input)?;
        let (input, identifier) =
            take_while(|c: char| S7F_IDENTIFIER_CHARACTERS.contains(c) && c != '|')(input)?;
        let (input, _) = tag("|")(input)?;
        Ok((input, S7FAST::Identifier(identifier.to_string())))
    }
}

enum S7FCell {
    Int(i64),
    Float(f64),
    Complex(f64, f64),
    String(String),
    Symbol(String),
    List(Vec<S7FCell>),
    Vector(Vec<S7FCell>),
    // Map(BTreeMap<S7FCell, S7FCell>),
    // Function(S7FFunction),
    // Builtin(S7FBuiltin),
    // Port(S7FPort),
    // Error(S7FError),
    // S7FInstruction(S7FInstruction),
}

enum S7FInstruction {
    Nop,
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    /// Test the creation of a regex from the constants above
    #[test]
    fn test_simple_identifier_regex() {
        let re = Regex::new(S7F_IDENTIFIER_REGEX).unwrap();
        assert!(re.is_match("abc_123"));
        assert!(re.is_match("abc_123"));
        assert!(re.is_match("abc-123"));
        assert!(re.is_match("abc-123"));
        assert!(re.is_match("abc:123"));
        assert!(re.is_match("abc!123"));
        assert!(re.is_match("abc?123"));
        assert!(re.is_match("abc.123"));
        assert!(re.is_match("abc.123"));
        assert!(re.is_match("abc.123"));
        assert!(re.is_match("abc"));
        assert!(re.is_match("abc@123"));
        assert!(re.is_match("abc*123"));
        assert!(re.is_match("abc/123"));
        assert!(re.is_match("abc&123"));
        assert!(re.is_match("abc%123"));
        assert!(re.is_match("abc^123"));
        assert!(re.is_match("abc+123"));
        assert!(re.is_match("abc<123"));
        assert!(re.is_match("abc<123"));
        assert!(re.is_match("abc=123"));
        assert!(re.is_match("abc>123"));
        assert!(re.is_match("abc~123"));
        assert!(re.is_match("abc$123"));
        assert!(re.is_match("abc123"));
    }
    #[test]
    fn test_vertical_identifier_regex() {
        let re = Regex::new(S7F_VERTICAL_CHARACTER_IDENTIFIER_REGEX).unwrap();
        assert!(re.is_match("|abc_123|"));
        assert!(re.is_match("|abc_123|"));
        assert!(re.is_match("|abc-123|"));
        assert!(re.is_match("|abc-123|"));
        assert!(re.is_match("|abc:123|"));
        assert!(re.is_match("|abc!123|"));
        assert!(re.is_match("|abc?123|"));
        assert!(re.is_match("|abc.123|"));
        assert!(re.is_match("|abc.123|"));
        assert!(re.is_match("|abc.123|"));
        assert!(re.is_match("|abc|"));
        assert!(re.is_match("|abc@123|"));
        assert!(re.is_match("|abc*123|"));
        assert!(re.is_match("|abc/123|"));
        assert!(re.is_match("|abc&123|"));
        assert!(re.is_match("|abc%123|"));
        assert!(re.is_match("|abc^123|"));
        assert!(re.is_match("|abc+123|"));
        assert!(re.is_match("|abc<123|"));
        assert!(re.is_match("|abc<123|"));
        assert!(re.is_match("|abc=123|"));
        assert!(re.is_match("|abc>123|"));
        assert!(re.is_match("|abc~123|"));
        assert!(re.is_match("|abc$123|"));
        assert!(re.is_match("|abc123|"));
    }
    #[test]
    fn test_parse_of_scheme_source() {
        let source = "(define (f x) (+ x 1))";
        let result = S7FParser::parse(source);
        assert!(result.is_ok());
        let (remaining, ast) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            ast,
            S7FAST::List(vec![
                S7FAST::Identifier("define".to_string()),
                S7FAST::List(vec![
                    S7FAST::Identifier("f".to_string()),
                    S7FAST::Identifier("x".to_string())
                ]),
                S7FAST::List(vec![
                    S7FAST::Identifier("+".to_string()),
                    S7FAST::Identifier("x".to_string()),
                    S7FAST::Int(1)
                ])
            ])
        );
    }
}

// A macro that converts EBNF to a nom parser:
// - A named rule is enclosed by angle brackets
// - A definition of a named rule follows the rule name and :=.
//   The definition is a sequence of rules separated by spaces or newlines.
//   Rule definitions can have literals, regexes, and other rules.
//   Choices are separated by pipes.
//   The definition can be repeated with a star, plus or question mark.
//   A plus defines repeition of at least one, a star defines zero or more, and a question mark defines zero or one.
//   A negative rule excludes a sequence of characters that match the rule this uses a - followed by the rule name, regex or literal.
//   Comments are preceded by a # and continue to the end of the line.
macro_rules! ebnf {
    () => {
        let mut rules = HashMap::new();
    };
    // add a rule
    ($name:ident := $def:tt $($rest:tt)*) => {
        ebnf!(@add_rule $name $def);
        ebnf!($($rest)*);
    };
    (@add_rule $name:ident $def:tt) => {
        rules.insert(stringify!($name), ebnf!(@parse_rule $def));
    };
}
struct R7RSParser {}
impl R7RSParser {
    ebnf![unstringify!(S7F_EBNF)];
}
