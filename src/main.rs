use crate::lib::{Attributes};

mod lib;

fn main() {
    use std::env;
    let mut args = env::args();
    args.next();
    let description = args.next().expect("Need at least two params");
    let pkgs = args.map(|s| format!("\n\t\t\tpkgs.{}",s)).collect::<Vec<String>>().join(" ");
    let pkgs = format!("{pkgs}\n\t");
    let output = Attributes::new(&description, &pkgs).generate();
    println!("{}",output);
}
