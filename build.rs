mod redoxri;
use redoxri::*;

fn main() -> () {
    let _redoxri = Redoxri::new(&[]);
    let main = Mcule::new("redoxr", "./redoxr.rs")
        .add_step(&[
            "echo", "test",
        ])
        .with(&["test".into()])
        .just_compile();


    println!("Bow Kreah!");
}
