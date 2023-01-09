use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/r7rs.pest"]
pub struct R7RSParser;
