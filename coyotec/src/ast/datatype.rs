#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DataType {
    Integer,
    Float,
    Boolean,
    String,
    Array,
    Function,
    Struct(usize),
    None,
}

impl DataType {
    pub fn get_prefix(&self) -> &str {
        match self {
            DataType::Integer => "i",
            DataType::Float => "f",
            DataType::Boolean => "b",
            DataType::String => "s",
            DataType::Array => "a",
            DataType::Function => "f",
            DataType::Struct(_) => "s",
            DataType::None => "n",
        }
    }
}
