mod natural_sum;
mod fibonacci_sum;

fn main() {
    let problem1 = natural_sum::natural_sum(10000);
    println!("Sum of multiples of 3 and 5: {}", problem1);

    let problem2 = fibonacci_sum::fibonacci_even_sum(0, 1);
    println!("Sum of even fibonacci values: {:?}", problem2);
}
