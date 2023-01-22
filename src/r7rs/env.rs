use crate::r7rs::vars::Variable;
use std::collections::BTreeMap;

pub struct Environment {
    vars: BTreeMap<String, Variable>,
    parent: Option<Box<Environment>>,
}
impl Environment {
    pub fn new() -> Environment {
        Environment {
            vars: BTreeMap::new(),
            parent: None,
        }
    }
    pub fn keys(&self) -> Vec<String> {
        self.vars.keys().map(|x| x.to_string()).collect()
    }
    pub fn get(&self, key: &str) -> Option<&Variable> {
        match self.vars.get(key) {
            Some(v) => Some(v.clone().to_owned()),
            None => match &self.parent {
                Some(p) => p.get(key),
                None => None,
            },
        }
    }
}
