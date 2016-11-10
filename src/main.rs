pub mod ipdl;
pub mod ast;

use ast::ParserState;

fn main() {
    println!("Output: {:?}", ipdl::parse_IncludeStmt(&ParserState::new(), "include      Whatever").unwrap());
    println!("Output: {:?}", ipdl::parse_CxxIncludeStmt(&ParserState::new(), "include   \"hello.h\"").unwrap());
}
