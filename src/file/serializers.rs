use crate::file::configuration::Serializable;

impl Serializable for String {
    fn serialize(&self) -> String {
        String::from("\"".to_owned()+ &self.clone() + "\"")
    }
    fn get_string(&self) -> &String {
        self
    }
}

impl Serializable for i32 {
    fn serialize(&self) -> String {
        self.to_string()
    }
}

impl Serializable for f64 {
    fn serialize(&self) -> String {
        self.to_string()
    }
}

impl Serializable for bool {
    fn serialize(&self) -> String {
        self.to_string()
    }
}