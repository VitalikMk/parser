use anyhow::Result;
use parser::{LogParser};
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> Result<()> {
    let reader = BufReader::new(File::open("./store.log")?).lines();

    let key_value = LogParser::parse(reader)?;
    println!("{:?}", key_value);

    Ok(())
}

mod parser {
    use std::collections::HashMap;
    use std::io::{BufReader, Lines};
    use std::iter::Peekable;
    use std::str::Chars;

    pub struct LogParser<R: Read> {
        lines: Lines<BufReader<R>>,
        key_value: HashMap<String, String>,
    }

    impl <R: Read> LogParser<R> {
        pub fn parse(lines: Lines<BufReader<R>>) -> Result<HashMap<String, String>> {
            let mut pareser = LogParser {
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

        fn parse_control(&mut self, line: &mut Peekable<Chars>) -> Result<()> {}

        fn parse_del_key(&mut self, line: &mut Peekable<Chars>) -> Result<()> {}

        fn parse_key_value_for_key(&mut self, line: &mut Peekable<Chars>) {}

    }
}
