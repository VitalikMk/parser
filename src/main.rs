use anyhow::Result;
use parser::LogParser;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> Result<()> {
    let reader = BufReader::new(File::open("store.txt")?).lines();

    let key_value = LogParser::parse(reader)?;
    println!("{:?}", key_value);

    Ok(())
}

mod parser {
    use std::collections::HashMap;
    use std::io::{BufReader, Lines, Read};
    use std::iter::Peekable;
    use std::str::Chars;
    use anyhow::{anyhow, Result};


    pub struct LogParser<R: Read> {
        lines: Lines<BufReader<R>>,
        key_value: HashMap<String, String>,
    }

    impl <R: Read> LogParser<R> {
        pub fn parse(lines: Lines<BufReader<R>>) -> Result<HashMap<String, String>> {
            let mut parser = LogParser {
                lines,
                key_value: HashMap::new(),
            };

            while let Some(Ok(line)) = parser.lines.next() {
                parser.parse_line(&mut line.chars().peekable())?;
            }

            Ok(parser.key_value)
        }

        fn parse_line(&mut self, line: &mut Peekable<Chars>) -> Result<()> {
            match line.peek() {
                Some(':') => {
                    line.next();
                    self.parse_control(line)?;
                }
                Some(_) => {
                    self.parse_key_value_for_key(line)?;
                }
                None => {
                    // Empty line, do nothing
                }
            }
            Ok(())
        }

        fn parse_control(&mut self, line: &mut Peekable<Chars>) -> Result<()> {
            let mut control = String::new();
            loop {
                match line.peek() {
                    Some(' ') => {
                        line.next();
                        break;
                    }
                    None => {
                        break;
                    }
                    _ => control.push(line.next().unwrap()),
                }
            }

            match control.as_ref() {
                "del" => {
                    self.parse_del_key(line)?;
                }
                _ => return Err(anyhow!("unrecognized control {}", control)),
            };
            Ok(())
        }

        fn parse_del_key(&mut self, line: &mut Peekable<Chars>) -> Result<()> {
            let mut key = String::new();
            loop {
                match line.peek() {
                    Some('a'..='z') => {
                        key.push(line.next().unwrap());
                    }
                    Some(ch) => return Err(anyhow!("unexpected character '{}' in delete key", ch)),
                    None => {
                        break;
                    }
                }
            }

            self.key_value.remove(key.as_str());

            Ok(())
        }

        fn parse_key_value_for_key(&mut self, line: &mut Peekable<Chars>) -> Result<()> {
            let mut key = String::new();
            loop {
                match line.peek() {
                    Some('a'..='z') => {
                        key.push(line.next().unwrap());
                    }
                    Some('=') => {
                        line.next();
                        break;
                    }
                    Some(ch) => {
                        return Err(anyhow!("unexpected character '{}' in key value", ch))
                    }
                    None => {
                        return Err(anyhow!("key should be followed by '=' and a value"));
                    }
                }
            }

            self.parse_key_value_for_value(key, line)?;

            Ok(())
        }

        fn parse_key_value_for_value(
            &mut self,
            key: String,
            line: &mut Peekable<Chars>,
        ) -> Result<()> {
            let mut value = String::new();
            loop {
                match line.peek() {
                    Some('a'..='z') => {
                        value.push(line.next().unwrap());
                    }
                    Some(ch) => {
                        return Err(anyhow!("unexpected character '{}' in value", ch));
                    }
                    None => {
                        break;
                    }
                }
            }

            self.key_value.insert(key, value);

            Ok(())
        }

    }
}
