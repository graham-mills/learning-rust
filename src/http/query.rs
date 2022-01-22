use std::collections::HashMap;
use std::convert::From;

/// Stores the query parameters appended to an URL,
/// e.g. `/search.html?term=rust&sort=1&page=2`.
/// These parameters are represented as a key-value map,
/// where each key can either have a single value, or
/// multiple values, as modelled by the `Value` enum.
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

/// Represents the value of a query parameter's key
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multi(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) ->Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    /// Returns a new `QueryString` from a raw string
    fn from(s: &'buf str) -> Self {
        // Example query string:
        // a=1&b=2&c&c=&e===&d=7&d=abc
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut value = "";

            if let Some(index) = sub_str.find('=') {
                key = &sub_str[..index];
                value = &sub_str[index + 1..];
            }

            data.entry(key)
            .and_modify(|existing: &mut Value| match existing {
                Value::Single(first_value) => {
                    *existing = Value::Multi(vec![first_value, value]);
                },
                Value::Multi(vec) => {
                    vec.push(value);
                }
            })
            .or_insert(Value::Single(value));
        }

        QueryString{data}
    }
}

