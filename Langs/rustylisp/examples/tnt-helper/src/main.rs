#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{
    iterators::Pairs,
    pratt_parser::{Assoc, Op, PrattParser},
    Parser
};
 use rustyline::{
    error::ReadlineError,
    DefaultEditor,
};
use calculator::Result;

#[derive(Parser)]
#[grammar = "calculator.pest"]
struct Calculator;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use Rule::*;
        use Assoc::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(sub, Left))
            .op(Op::infix(mul, Left) | Op::infix(div, Left))
            .op(Op::infix(pow, Right))
    };
}

fn eval(expression: Pairs<Rule>) -> f64 {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::num => primary.as_str().parse::<f64>().unwrap(),
            Rule::expr => eval(primary.into_inner()),
            _ => unreachable!()
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::add => lhs + rhs,
            Rule::sub => lhs - rhs,
            Rule::mul => lhs * rhs,
            Rule::div => lhs / rhs,
            Rule::pow => lhs.powf(rhs),
            _ => unreachable!()
        })
        .parse(expression)
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let line = line.trim().to_string();
                let parse_result = Calculator::parse(Rule::calculation, &line);
                
                match parse_result {
                    Ok(mut calc) => println!(
                        " = {}",
                        eval(calc.next().unwrap().into_inner())
                    ),
                    Err(_) => println!(" Syntax error")
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("Goodbye ...");
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
    Ok(())
}
