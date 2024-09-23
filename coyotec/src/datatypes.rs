#![allow(dead_code, unused_variables)]
#[derive(PartialEq, Debug)]
pub enum DataType {
    Integer,
    Float,
    Bool,
    String,
    Void,
    Unknown,
}

#[derive(PartialEq, Debug)]
pub struct Data<T> {
    pub data_type: DataType,
    pub value: T,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_data() {
        let data = Data {
            data_type: DataType::Integer,
            value: 42,
        };
        assert_eq!(data.data_type, DataType::Integer);
        assert_eq!(data.value, 42);

        let sdata = Data {
            data_type: DataType::String,
            value: "Hello, World!".to_string(),
        };
        assert_eq!(sdata.data_type, DataType::String);
        assert_eq!(sdata.value, "Hello, World!".to_string());




    }
}

