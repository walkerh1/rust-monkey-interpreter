use crate::evaluator::environment::Environment;
use crate::evaluator::eval;
use crate::evaluator::object::Object;
use crate::parser::Parser;
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

pub struct Repl;

const PROMPT: &str = ">> ";

impl Repl {
    pub fn start() -> io::Result<()> {
        let reader = io::stdin();
        let mut writer = io::stdout();

        let env = Rc::new(RefCell::new(Environment::new()));

        loop {
            writer.write_all(PROMPT.as_bytes())?;
            writer.flush()?;

            let mut buffer = String::new();
            let bytes_read = reader.read_line(&mut buffer)?;

            if bytes_read == 0 {
                writeln!(writer)?;
                break;
            }

            let parsing_result = Parser::parse_program(buffer.as_str());
            let program = match parsing_result {
                Ok(program) => program,
                Err(errors) => {
                    errors.iter().for_each(|e| println!("{e:?}"));
                    continue;
                }
            };

            let eval_result = eval(program, Rc::clone(&env));
            match eval_result {
                Ok(object) => match &*object {
                    Object::Null | Object::Function(_) => continue,
                    obj => println!("{obj}"),
                },
                Err(error) => println!("{error:?}"),
            }
        }

        Ok(())
    }
}
