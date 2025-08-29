mod redoxri;
use redoxri::*;

fn main() -> () {
    let main = Mcule::new("redoxr", "./redoxr.rs")
        .with(&["test".into()]);

}
