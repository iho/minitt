use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar);
pub mod ast;
fn print(s: &str) {
    let res = grammar::ExprParser::new().parse(s).unwrap();
    println!("Term: {:?}", res);
    println!("Type: {:?}", res.type_check());
    println!("");
}

fn main() {
   print("Σ (x:*), x");
}
