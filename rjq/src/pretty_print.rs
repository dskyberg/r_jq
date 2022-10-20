use colored::Colorize;

use r_jq::serde_json::Value;

pub struct PrettyPrint {
    tab_size: usize,
    indent: usize,
    _raw: bool,
}

impl Default for PrettyPrint {
    fn default() -> Self {
        Self::new()
    }
}

impl PrettyPrint {
    pub fn new() -> Self {
        Self {
            tab_size: 3,
            indent: 0,
            _raw: false,
        }
    }

    pub fn indent(&mut self) -> &mut Self {
        self.indent += 1;
        self
    }
    pub fn outdent(&mut self) -> &mut Self {
        if self.indent > 0 {
            self.indent -= 1;
        }
        self
    }
    pub fn _set_raw(&mut self, raw: bool) -> &mut Self {
        self._raw = raw;
        self
    }

    pub fn spaces(&self) -> usize {
        self.indent * self.tab_size
    }

    pub fn fill(&self, indent: bool) -> String {
        if !indent {
            return "".to_string();
        }

        let mut result = String::new();
        for _s in 0..self.spaces() {
            result.push(' ');
        }

        result
    }

    pub fn print(&mut self, value: &Value, indent: bool) -> Result<(), Box<dyn std::error::Error>> {
        match value {
            Value::Null => {
                print!("{}{}", self.fill(indent), "null".truecolor(105, 105, 105));
            }
            Value::Bool(b) => {
                print!("{}{}", self.fill(indent), &b)
            }
            Value::Number(n) => print!("{}{}", self.fill(indent), &n),
            Value::String(s) => print!("{}{}", self.fill(indent), s.as_str().green()),
            Value::Array(array) => {
                println!("[");
                self.indent();
                for idx in 0..array.len() {
                    let v = &array[idx];
                    self.print(v, true)?;
                    if idx < array.len() - 1 {
                        println!(",");
                    } else {
                        println!();
                    }
                }
                self.outdent();
                print!("{}]", self.fill(true));
            }
            Value::Object(m) => {
                println!("{{");
                self.indent();
                let keys = m.keys().collect::<Vec<&String>>();
                for idx in 0..keys.len() {
                    let key = keys[idx];
                    let value = m.get(key).unwrap();
                    print!("{}\"{}\": ", self.fill(true), key.blue());
                    self.print(value, false)?;
                    if idx < keys.len() - 1 {
                        println!(",");
                    } else {
                        println!();
                    }
                }
                self.outdent();
                print!("{}}}", self.fill(true));
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use r_jq::serde_json::json;

    #[test]
    fn test_it() {
        let mut pretty = PrettyPrint::new();
        let value = json!({"bool": true, "number": 12, "string": "abc", "null":null, "array":[1,2,3], "obj":{"bool": true, "number": 12, "string": "abc", "null":null, "array":[1,2,3]}});
        pretty.print(&value, true).expect("failed");
    }
}
