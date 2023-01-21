use crate::r7rs::vars::Variable;
use std::collections::BTreeMap;

pub struct Environment {
    vars: BTreeMap<String, Variable>,
    parent: Option<Box<Environment>>,
}
