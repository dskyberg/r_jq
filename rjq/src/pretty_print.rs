use colored::Colorize;

use r_jq::serde_json::{Map, Value};

pub struct PrettyPrint {
    tab_size: usize,
    indent: usize,
    raw: bool,
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
            raw: false,
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
    pub fn set_raw(&mut self, raw: bool) -> &mut Self {
        self.raw = raw;
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
        for s in 0..self.spaces() {
            result.push(' ');
        }

        result
    }

    pub fn print(
        &mut self,
        value: &Value,
        indent: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let result = match value {
            Value::Null => {
                format!("{}null", self.fill(indent))
            }
            Value::Bool(b) => {
                format!("{}{}", self.fill(indent), &b)
            }
            Value::Number(n) => format!("{}{}", self.fill(indent), &n),
            Value::String(s) => format!("{}{}", self.fill(indent), s.as_str().green()),
            Value::Array(array) => {
                let mut values: Vec<String> = Vec::new();
                self.indent();
                for v in array {
                    values.push(self.print(v, true)?);
                }
                self.outdent();
                format!("[\n{}\n{}]", values.join(",\n"), self.fill(true))
            }
            Value::Object(m) => {
                let mut values: Vec<String> = Vec::new();
                //s += &format!("{}{{", self.fill(indent));
                self.indent();
                for (key, value) in m {
                    values.push(format!(
                        "{}\"{}\": {}",
                        self.fill(true),
                        key.blue(),
                        self.print(value, false)?
                    ));
                }
                self.outdent();
                format!("{{\n{}\n{}}}", values.join(",\n"), self.fill(true))
            }
        };
        Ok(result)
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
        let result = pretty.print(&value, true).expect("failed");
        println!("{}", &result);
    }
}
