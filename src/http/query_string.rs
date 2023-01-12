use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc
#[derive(Debug)]
pub struct QueryString <'buff> {
    data : HashMap<&'buff str, Value<'buff>>
}
#[derive(Debug)]
pub enum Value <'buff> {
    Single(&'buff str),
    Multiple(Vec<&'buff str>)
}

impl <'buf> QueryString<'buf>{
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}


impl<'buff> From <&'buff str> for QueryString<'buff>{
    fn from(s: &'buff str) -> Self {
    
    let mut data = HashMap::new();
    
    for substr in s.split('&'){
        let mut key = substr;
        let mut val = "";
        if let Some(i) = substr.find('='){
            key = &substr[..i];
            val = &substr[i + 1..];
        }

        data.entry(key)
            .and_modify(|existing| match existing {
                Value::Single(prev_val) => {
                    // let mut vec = vec![prev_val, val];
                    *existing = Value::Multiple(vec![prev_val, val]);
                },
                Value::Multiple(vec) => vec.push(val)
            })
            .or_insert(Value::Single((val)));

        
    }
    
    QueryString { data }

    }
}


