mod redoxri;
use redoxri::*;
use std::path::Path;

fn main() -> () {
    let _redoxri = Redoxri::new(&[
        "--cfg", "isolate",
        "--cfg", "check",
    ]);
    let main = Mcule::new("redoxri", "./libredoxri.rlib")
        .add_step(&[
            "rustc", "./redoxri.rs", "--crate-type", "lib",
        ])
        .with(&["redoxri.rs".into()])
        .compile();



    println!("sshlow mehr!");
}
