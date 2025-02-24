fn f(mut n: i32) -> i32 {
    let mut c = 0;

    while n >= 0 {
        n -= 2;
        c += n - 2;
    }

    c
}

fn main() {
    println!("{}", f(11))
}
