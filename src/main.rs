use std::collections::HashMap;
use std::{io, string};

struct VarTable
{
    data: HashMap<String, f64>,
}

impl VarTable
{

    fn setVar(&mut self, name: &str, value: f64)
    {
        self.data.insert(name.to_string(), value);
    }

    fn getVar(&self, name: &str) -> Option<&f64>{
        self.data.get(&name.to_string())
    }

    fn print_var(&self, name: &str){
        let x = self.data.get(&name.to_string());
        match x{
            Some(x) => println!("{x}"),
            None => println!("ERROR: no such variable \"{name}\" "),
        }
    }

    fn do_operation(&mut self, opString_: &str){
        let opString = opString_.split_once('=');

        match opString{
            Some(opString) => {
                let name = opString.0.trim();
                let val = opString.1.trim().parse::<f64>();

                match val{
                    Ok(value) => self.setVar(name, value),
                    Err(_) => println!("ERROR: Could not parse number"),
                }
            }
            None => {
                let name = opString_.trim();
                self.print_var(name);
            }
        }

    }
}



fn main() {

    let mut vt = VarTable{data: HashMap::new()};

    'outer: loop{
        let mut opString = String::new();
        let res = io::stdin().read_line(&mut opString);

        match res{
            Ok(_) => {
                if opString.eq("q\n") { break 'outer;}
                vt.do_operation(&opString);
            },
            Err(_) => {
                println!("Error reading user input");
                break 'outer;
            }
        }
    }

}
