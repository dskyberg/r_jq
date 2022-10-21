use colored::Colorize;

use r_jq::serde_json::Value;

pub struct PrettyPrint {
    use_tabs: bool,
    compact: bool,
    tab_size: usize,
    indent: usize,
}

impl Default for PrettyPrint {
    fn default() -> Self {
        Self::new()
    }
}

impl PrettyPrint {
    pub fn new() -> Self {
        Self {
            use_tabs: false,
            compact: false,
            tab_size: 3,
            indent: 0,
        }
    }

    pub fn indent(&mut self) {
        self.indent += 1;
    }
    pub fn outdent(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }

    pub fn with_use_tabs(mut self, use_tabs: bool) -> Self {
        self.use_tabs = use_tabs;
        self
    }

    pub fn with_compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }
    pub fn spaces(&self) -> usize {
        if self.compact {
            0
        } else if self.use_tabs {
            self.indent
        } else {
            self.indent * self.tab_size
        }
    }

    pub fn fill(&self, indent: bool) -> String {
        if !indent {
            return "".to_string();
        }

        let mut result = String::new();
        for _s in 0..self.spaces() {
            if self.use_tabs {
                result.push('\t');
            } else {
                result.push(' ');
            }
        }

        result
    }

    pub fn newline(&self) -> String {
        if self.compact {
            "".to_string()
        } else {
            "\n".to_string()
        }
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
            Value::String(s) => print!("{}\"{}\"", self.fill(indent), s.as_str().green()),
            Value::Array(array) => {
                print!("[{}", self.newline());
                self.indent();
                for idx in 0..array.len() {
                    let v = &array[idx];
                    self.print(v, true)?;
                    if idx < array.len() - 1 {
                        print!(",{}", self.newline());
                    } else {
                        print!("{}", self.newline());
                    }
                }
                self.outdent();
                print!("{}]", self.fill(true));
            }
            Value::Object(m) => {
                print!("{{{}", self.newline());
                self.indent();
                let keys = m.keys().collect::<Vec<&String>>();
                for idx in 0..keys.len() {
                    let key = keys[idx];
                    let value = m.get(key).unwrap();
                    print!("{}\"{}\": ", self.fill(true), key.blue());
                    self.print(value, false)?;
                    if idx < keys.len() - 1 {
                        print!(",{}", self.newline());
                    } else {
                        print!("{}", self.newline());
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
