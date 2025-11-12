mod redoxri;
use redoxri::*;
// use std::path::Path;

fn main() -> () {
    let _redoxri = Redoxri::new(&[
        // "--cfg", "isolate",
        "--cfg", "debug",
        "--cfg", "unstable",
        "--cfg", "unmute_on_fail",
        //"--cfg", "mute_self",
    ]);

    let _main = Mcule::new("redoxri", "./libredoxri.rlib")
        .add_step(&[
            "rustc", "./redoxri.rs", "--crate-type", "lib",
        ])
        .with(&["redoxri.rs".into()])
        .mute()
        .compile();

    let redoxsrc = Mcule::new("redoxsrc", "./redoxri.rs");

    redoxsrc.copy_to("./examples/01_Basics_in_Rust/redoxri.rs");

    println!("!");
}
