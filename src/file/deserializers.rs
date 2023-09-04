use crate::file::configuration::{ConfigElement, Deserializer};

pub fn deserializers() -> [Box<dyn Deserializer>; 3] { [Box::new(StringDes {}), Box::new(IntDes {}), Box::new(BoolDes {})] }

struct StringDes {}
impl Deserializer for StringDes {
    fn deserialize(&self, s: &String, element: &mut ConfigElement) {
        (*element).data = Some(Box::new((*s).clone()));
    }

    fn is_type(&self, s: &String) -> bool {
        s.chars().next().unwrap() == '"' && s.chars().last().unwrap() == '"'
    }
}


struct IntDes {}

impl Deserializer for IntDes {
    fn deserialize(&self, s: &String,element: &mut ConfigElement) {
        let res: Result<i32, _> = (*s).parse();
        if res.is_err() {
            eprintln!("Error: could not parse \"{}\" as an i32", s);
            (*element).data = None;
        }else {
            (*element).data = Some(Box::new(res.unwrap()));
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
    fn deserialize(&self, s: &String,element: &mut ConfigElement) {
        let res: Result<f64, _> = (*s).parse();
        if res.is_err() {
            eprintln!("Error: could not parse \"{}\" as an f64", s);
            (*element).data = None;
        }else {
            (*element).data = Some(Box::new(res.unwrap() as f64));
        }
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
    fn deserialize(&self, s: &String, element: &mut ConfigElement) {
        if (*s).eq(&"true".to_string()) {
            (*element).data = Some(Box::new(true));
        }else if (*s).eq(&"false".to_string()) {
            (*element).data = Some(Box::new(false));
        } else {
            (*element).data = None;
        }
    }

    fn is_type(&self, s: &String) -> bool {
        (*s).eq(&"true".to_string()) || (*s).eq(&"false".to_string())
    }
}