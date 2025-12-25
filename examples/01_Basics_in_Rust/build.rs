mod redoxri;
use redoxri::*;

fn main() -> () {
    let _redoxri = Redoxri::new(&[""]);

    let out = Mcule::new("out", "out")
        .with(&["out".into()])
        .add_step(&["mkdir", "$out"])
        .compile();

    let main_src = Mcule::new("main_src", "src/main.rs");

    let _main = Mcule::new("fibonacci", "out/fibonacci")
        .with(&[main_src.clone()])
        .add_step(&["rustc", &main_src.outpath, "-o", "$out"])
        .compile()
        .run();
}
