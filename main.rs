
fn main() {
    println!("\nFizzbuzz made in Rust\n\nby Eesa Zahed\nhttps://github.com/eesazahed\n");

    fizzbuzz();
}

// run fizzbuzz up to 100

fn fizzbuzz() {
    for number in 1..101 {
        if number % 15 == 0 {
            println!("Fizzbuzz");
        } else if number % 5 == 0 {
            println!("Buzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", number);
        }
    }
}