mod redoxri;
use redoxri::*;

fn main() -> () {
    let main = Mcule::new("redoxr", "./redoxr.rs")
        .add_step(&[
            "echo", "test",
        ])
        .with(&["test".into()])
        .just_compile();

}
