mod redoxri;
use redoxri::*;

fn main() -> () {
    let _redoxri = Redoxri::new(&[""]);

    let main_src = Mcule::new("main_src", "src/main.rs");

    let _main = Mcule::new("fibonacci", "fibonacci")
        .with(&[main_src.clone()])
        .add_step(&["rustc", &main_src.outpath, "-o", "$out"])
        .compile()
        .run();
}
