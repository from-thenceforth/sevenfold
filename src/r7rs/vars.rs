use crate::r7rs::value::Value;

pub struct Variable {
    name: String,
    value: Value,
}
impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl std::fmt::Debug for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(\"{}\" . {})", self.name, self.value)
    }
}
