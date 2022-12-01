use std::io::stdin;

use day1::entrypoint;
fn main() {
    let input = stdin().lock();
    let result = entrypoint(input);

    println!("{}", result);
}
