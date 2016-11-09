pub mod ipdl;
pub mod ast;

fn main() {
    println!("Output: {:?}", ipdl::parse_IncludeStmt("include      Whatever").unwrap());
}
