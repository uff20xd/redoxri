fn main() {
    let n = 0;
    println!("{n}! is {}", fibonacci(n))
}

fn fibonacci(n: u128) -> u128 {
    let mut a = 1;
    let mut b = 1;
    let mut c = 2;

    for i in 0..n {
        a = b;
        b = c;
        c = a + b;
    }

    a
}

