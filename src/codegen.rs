use JsonValue;

pub struct Generator {
    pub minify: bool,
    code: String,
    dent: u16,
}

impl Generator {
    pub fn new(minify: bool) -> Self {
        Generator {
            minify: minify,
            code: String::new(),
            dent: 0,
        }
    }

    pub fn new_line(&mut self) {
        if !self.minify {
            self.code.push('\n');
            for _ in 0..self.dent {
                self.code.push_str("    ");
            }
        }
    }

    pub fn write_json(&mut self, json: &JsonValue) {
        match *json {
            JsonValue::String(ref string) => {
                self.write_char('"');

                for ch in string.chars() {
                    match ch {
                        '\\' | '/' | '"' => {
                            self.write_char('\\');
                            self.write_char(ch);
                        },
                        '\n'       => self.write("\\n"),
                        '\r'       => self.write("\\r"),
                        '\t'       => self.write("\\t"),
                        '\u{000C}' => self.write("\\f"),
                        '\u{0008}' => self.write("\\b"),
                        _          => self.write_char(ch)
                    }
                }

                self.write_char('"');
            },
            JsonValue::Number(ref number) => self.write(&number.to_string()),
            JsonValue::Boolean(ref value) => self.write(if *value { "true" } else { "false" }),
            JsonValue::Null               => self.write("null"),
            JsonValue::Array(ref array)   => {
                self.write_char('[');
                let mut first = true;
                for item in array {
                    if first {
                        first = false;
                    } else {
                        self.write_min(", ", ",");
                    }
                    self.write_json(item);
                }
                self.write_char(']');
            },
            JsonValue::Object(ref object) => {
                let mut first = true;
                self.write_char('{');
                self.indent();
                for (key, value) in object.iter() {
                    if first {
                        first = false;
                    } else {
                        self.write_min(", ", ",");
                    }
                    self.write(&format!("{:?}", key));
                    self.write_min(": ", ":");
                    self.write_json(value);
                }
                self.dedent();
                self.new_line();
                self.write_char('}');
            }
        }
    }

    pub fn write(&mut self, slice: &str) {
        self.code.push_str(slice);
    }

    pub fn write_min(&mut self, slice: &str, minslice: &str) {
        if self.minify {
            self.write(minslice);
        } else {
            self.write(slice);
        }
    }

    pub fn write_char(&mut self, ch: char) {
        self.code.push(ch);
    }

    pub fn indent(&mut self) {
        self.dent += 1;
    }

    pub fn dedent(&mut self) {
        self.dent -= 1;
    }

    pub fn consume(self) -> String {
        self.code
    }
}
