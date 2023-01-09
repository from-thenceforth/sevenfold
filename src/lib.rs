//! sevenfold is an embedded Scheme R7RS interpreter.

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;
mod r7rs_ast;
