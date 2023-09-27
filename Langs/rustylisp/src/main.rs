extern crate pest;
#[macro_use]
extern crate pest_derive;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use rustylisp::Result;

use pest::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line.to_owned())?;
                let record = CSVParser::parse(Rule::record, &line)?.next().unwrap();
                let mut field_sum = 0f64;
                match record.as_rule() {
                    Rule::record => {
                        for field in record.into_inner() {
                            field_sum += field.as_str().parse::<f64>().unwrap();
                        }
                    }
                    Rule::EOI => (),
                    _ => unreachable!()
                }
                println!("Sum: {field_sum}");
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {err}");
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}
