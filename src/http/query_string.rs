use std::collections::HashMap;

pub struct QueryString<'a> {
    data: HashMap<&'a str, Value<'a>>
}

pub enum Value<'a> {
    Single(&'a str),
    Multi(Vec<&'a str>),
}

impl<'a> QueryString<'a> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'a> From<&'a str> for QueryString<'a> {
    fn from(s: &str) -> Self {

        unimplemented!()
    }
}