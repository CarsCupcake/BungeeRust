use std::collections::{HashMap};
use std::fs::{File, read_to_string};
use std::io::Write;
use crate::file::deserializers::deserializers;

pub trait Serializable {
    fn serialize(&self) -> String;
    fn get_string(&self) -> &String {
        panic!("Illegal Call")
    }
}



pub trait Deserializer {
    fn deserialize(&self, s: &String) ->Box <dyn Serializable>;
    fn is_type(&self, s: &String) -> bool;
}

pub fn of(f: String, default: HashMap<String, Box<dyn Serializable>>) -> ConfigSection {
    let mut section = ConfigSection{element: default};
    let result = read_to_string(f);
    if result.is_err() {
        return section;
    }
    for line in result.unwrap().lines() {
        if line.is_empty() {
            continue;
        }
        if !line.contains(':') {
            panic!("Illegal line!")
        }
        let mut split = line.split(':');
        let name = split.next().unwrap().to_string();
        let mut data_val = String::new();
        let mut i = 0;
        let size = split.clone().count();
        for s in split {
            data_val.push_str(s);
            i += 1;
            if i != size {
                data_val.push(':');
            }
        }
        data_val = format_spaces(data_val);
        for d in deserializers() {
            let deser = d;
            if deser.is_type(&data_val) {
                let val = deser.deserialize(&data_val);
                section.set(name,val);
                break;
            }
        }

    }
    section
}

fn format_spaces(mut s: String) -> String {
    while s.chars().next().unwrap() == ' ' {
        s.remove(0);
    }
    while s.chars().last().unwrap() == ' ' {
        s.remove(s.len() - 1);
    }
    s
}

pub struct ConfigSection {
    pub element: HashMap<String, Box<dyn Serializable>>,
}

impl ConfigSection {

    pub fn get(&self, location: String) -> Option<&Box<dyn Serializable>> {
        self.element.get(&*location)
    }

    pub fn get_or_default<'a>(&'a self, location: String, default: &'a dyn Serializable) -> &dyn Serializable {
        if let Some(s) = self.get(location) {
            return s.as_ref();
        }
        default
    }

    pub fn is_empty(&self) -> bool {
        self.element.is_empty()
    }

    pub fn set(&mut self, location: String, val: Box<dyn Serializable>) {
        self.element.insert(location, val);
    }


    pub fn save(&self, file: &str) {
        let mut st = String::new();
        for element in self.element.iter() {
            st.push_str(element.0);
            st.push_str(": ");
            st.push_str(&*element.1.as_ref().serialize());
            st.push('\n');
        }
        let f = File::create(file);
        if f.is_err() {
            eprintln!("Error while saving {}", file);
            return;
        }
        let mut raw = f.unwrap();
        raw.write_all(st.as_ref()).expect("There was an error while writing to the file");
    }
}

