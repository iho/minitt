use lalrpop_util::lalrpop_mod;
use crate::check::check_main;
mod check;
pub mod eval;
pub mod pretty;

lalrpop_mod!(pub grammar);
pub mod ast;
fn print(s: &str) {
    let res = grammar::ExprParser::new().parse(s).unwrap();
    println!("Term: {:?}", res);
    println!("Type: {:?}", check_main(res));
    println!("");
}

fn main() {
   print("Π (n x: *0), n");
}
