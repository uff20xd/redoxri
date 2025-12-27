mod redoxri;
use redoxri::*;
// use std::path::Path;
static COMMON_FLAGS: &[&str] = &["-Copt-level=1"];
fn main() -> () {
    let _redoxri = Redoxri::new(&[
        //"--cfg", "isolate",
        //"--cfg", "debug",
        //"--cfg", "unstable",
        //"--cfg", "legacy"
        //"--cfg", "unmute_on_fail",
        //"--cfg", "mute_on_default",
        ""
    ]);

    let main = Mcule::new("redoxri", "libredoxri.rlib")
        .with(&["redoxri.rs".into()]);
    let _main = main.clone()
        .add_step(&[
            "rustc", &main.inputs[0].outpath, "--crate-type", "lib", "--edition=2024"
        ])
        .add_args(COMMON_FLAGS)
        .compile();

    let redoxsrc = Mcule::new("redoxsrc", "redoxri.rs");

    redoxsrc.copy_to("./examples/01_Basics_in_Rust/redoxri.rs");
    //println!("!");
}
    
