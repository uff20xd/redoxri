mod redoxri;
use redoxri::*;

fn main() -> () {
    let redoxri = Redoxri::new(&[
    ]);

    let main = Mcule::new("fibonacci", "fibonacci.rlib")
        .with(&["fibonacci.rs".into()])
        .add_step(&["rustc", "fibonacci.rs", "-o", "$out", "--crate-type=lib"])
        .compile();


    if redoxri.flags.contains("--clean") { 
    }
}
