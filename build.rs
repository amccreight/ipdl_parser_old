extern crate lalrpop;

fn main() {
    println!("cargo:rerun-if-changed=src/ipdl.lalrpop");

    lalrpop::Configuration::new()
        .generate_in_source_tree()
        .process()
        .unwrap();
}
