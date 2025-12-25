mod redoxri;
use redoxri::*;

fn main() -> () {
    let redoxri = Redoxri::new(&[""]);

    let main = Mcule::new("fibonacci", "fibonacci")
        .with(&["fibonacci.rs".into()])
        .add_step(&["rustc", "fibonacci.rs", "-o", "$out"])
        .compile()
        .run();
}
