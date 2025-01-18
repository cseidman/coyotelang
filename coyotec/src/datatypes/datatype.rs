#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NativeType {}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DataType {
    Integer,
    Float,
    Boolean,
    Text,
    Array,
    Function,
    List,
    Struct(usize),
    None,
}

impl DataType {
    pub fn get_prefix(&self) -> &str {
        match self {
            DataType::Integer => "i",
            DataType::Float => "f",
            DataType::Boolean => "b",
            DataType::Text => "s",
            DataType::Array => "a",
            DataType::Function => "f",
            DataType::List => "l",
            DataType::Struct(_) => "s",
            DataType::None => "n",
        }
    }

    pub fn get_vm_type(&self) -> u8 {
        match self {
            DataType::Integer => 6,
            DataType::Float => 1,
            DataType::Boolean => 2,
            DataType::Text => 3,
            DataType::Array => 3,
            DataType::Function => 3,
            DataType::List => 3,
            DataType::Struct(_) => 3,
            DataType::None => 0,
        }
    }
}
