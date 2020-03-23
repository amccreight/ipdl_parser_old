extern crate lalrpop;

fn main() {
    println!("cargo:rerun-if-changed=src/ipdl.lalrpop");

    lalrpop::process_root().unwrap();
}
