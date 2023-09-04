use std::any::{Any, TypeId};
use std::collections::LinkedList;
use std::fs::{File, read_to_string};
use crate::file::deserializers::deserializers;

pub struct ConfigElement {
    pub name: String,
    pub data: Option<Box<dyn Serializable>>
}
impl ConfigElement {



    fn to_string(&self) -> String {
        if let Some(data) = &self.data {
            return format!("{}: {}", self.name, (*data).serialize())
        }
        format!("{}: ", self.name)
    }
}
pub trait Serializable {
    fn serialize(&self) -> String;
    fn get_section(&self) -> &ConfigSection {
        panic!("Illegal Call")
    }
    fn get_string(&self) -> &String {
        panic!("Illegal Call")
    }
}
pub trait Deserializer {
    fn deserialize(&self, s: &String, element: &mut ConfigElement);
    fn is_type(&self,s: &String) -> bool;
}

pub fn of(f: String) -> ConfigSection {
    let mut section = ConfigSection::new();
    let result = read_to_string(f);
    if result.is_err() {
        return ConfigSection::new()
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
        let mut element = ConfigElement{name, data: None};
        for d in deserializers() {
            let deser = d;
            if deser.is_type(&data_val) {
                deser.deserialize(&data_val, &mut element);
                section.add_raw(element);
                break
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
    pub element: LinkedList<ConfigElement>
}
impl ConfigSection {

    pub const fn new() -> Self {
        ConfigSection{element: LinkedList::new()}
    }

    pub fn get(&self, location: String) -> Option<&Box<dyn Serializable>> {
        let mut it: Vec<&str> = location.split('.').collect();
        it.reverse();
        self.find_by_split(it)
    }

    fn find_by_split(&self, mut split: Vec<&str>) -> Option<&Box<dyn Serializable>>  {
        let name = split.pop().unwrap();
        for element in self.element.iter().clone() {
            if element.name.eq(&name) {
                return if let Some(data) = &element.data {
                    if split.len() == 0 {
                        return Some(data)
                    }
                    if (*data).type_id() != TypeId::of::<ConfigSection>() {
                        return None
                    }
                    let cs: &ConfigSection = (*data).get_section();
                    return cs.find_by_split(split)
                } else {
                    None
                }
            }
        }
        return None
    }

    pub fn get_or_default<'a>(&'a self, location: String, default: &'a dyn Serializable) -> &dyn Serializable {
        if let Some(s) = self.get(location) {
            return s.as_ref()
        }
        default
    }

    pub fn set(&self, location: String, val: Option<Box<dyn Serializable>>) {
        todo!()
    }

    pub fn save(file: &File) {
        todo!()
    }

    pub fn add_raw(&mut self, element: ConfigElement) {
        self.element.push_back(element)
    }
}

impl Serializable for ConfigSection {
    fn serialize(&self) -> String {
        let mut s = String::from("{\n");
        for el in self.element.iter() {
            s.push_str(&*el.to_string().as_str());
            s.push_str("\n");
        }
        s.push('}');
        s
    }

    fn get_section(&self) -> &ConfigSection {
        self
    }
}