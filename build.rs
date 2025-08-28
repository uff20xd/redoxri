mod redoxr;
use redoxr::*;

fn main() -> () {
    let main = Mcule::new("redoxr", "./redoxr.rs")
        .with(&["test".into()]);

}
