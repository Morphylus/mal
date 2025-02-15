//use std::io::{self, Write};
//
//fn main() -> io::Result<()> {
//    let mut input = String::new();
//    loop {
//        print!("user> ");
//        io::stdout().flush()?;
//
//        io::stdin().read_line(&mut input)?;
//
//        if !input.is_empty() {
//            println!("{}", rep(&input));
//        }
//        input.clear();
//    }
//}
//
//fn rep(instr: &str) -> &str {
//    print(eval(read(instr)))
//}
//
////fn read(instr: &str) -> MalType {
////    instr
////}
//
//fn eval(instr: &str) -> &str {
//    instr
//}
//
//fn print(instr: &str) -> &str {
//    instr
//}
