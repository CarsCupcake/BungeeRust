use crate::file::configuration::{Deserializer, Serializable};

pub fn deserializers() -> [Box<dyn Deserializer>; 3] { [Box::new(StringDes {}), Box::new(IntDes {}), Box::new(BoolDes {})] }

struct StringDes {}
impl Deserializer for StringDes {
    fn deserialize(&self, s: &String) -> Box<dyn Serializable> {
        let mut new_s = String::new();
        let mut i = 0;
        for c in s.chars() {
            if i == 0 || i == s.len() - 1 {
            } else {
                new_s.push(c);
            }
            i += 1;
        }
        Box::new((new_s).clone())
    }

    fn is_type(&self, s: &String) -> bool {
        s.chars().next().unwrap() == '"' && s.chars().last().unwrap() == '"'
    }
}


struct IntDes {}

impl Deserializer for IntDes {
    fn deserialize(&self, s: &String) -> Box<dyn Serializable> {
        let res: Result<i32, _> = (*s).parse();
        return if res.is_err() {
            eprintln!("Error: could not parse \"{}\" as an i32", s);
            Box::new(0)
        } else {
            Box::new(res.unwrap())
        }
    }

    fn is_type(&self, s: &String) -> bool {
        for c in (*s).chars() {
            if !c.is_numeric() {
                return false
            }
        }
        true
    }
}
struct FloatDes {}

impl Deserializer for FloatDes {
    fn deserialize(&self, s: &String) -> Box<dyn Serializable>  {
        let res: Result<f64, _> = (*s).parse();
        if res.is_err() {
            eprintln!("Error: could not parse \"{}\" as an f64", s);
            return  Box::new(0_f64)
        }
        Box::new(res.unwrap())
    }

    fn is_type(&self, s: &String) -> bool {
        let mut b = false;
        for c in (*s).chars() {
            if !c.is_numeric() {
                if c == '.' {
                    if b {
                        return  false
                    } else {
                        b = true
                    }
                }else {
                    return false
                }
            }
        }
        true
    }
}
struct BoolDes {}

impl Deserializer for BoolDes {
    fn deserialize(&self, s: &String)  -> Box<dyn Serializable>  {
        return  if (*s).eq(&"true".to_string()) {
           Box::new(true)
        }else if (*s).eq(&"false".to_string()) {
          Box::new(false)
        } else {
            panic!("Illegal!")
        }
    }

    fn is_type(&self, s: &String) -> bool {
        (*s).eq(&"true".to_string()) || (*s).eq(&"false".to_string())
    }
}