const LIMIT: i32 = 4_000_000;

fn fib_sum(sequence: &Vec<i32>, index: usize) -> i32 {
	sequence[index] + sequence[index - 1]
}

pub fn fibonacci_even_sum(mut fib1: i32, mut fib2: i32) -> i32 {
	let mut fib_seq: Vec<i32> = vec![fib1, fib2];
	let mut sum: i32 = 0;
	let mut i: usize = 1;

	while let mut fib = fib_sum(&fib_seq, i) {
		match fib < LIMIT {
			true => {
				fib_seq.push(fib);
				i += 1;
				if fib % 2 == 0 {
					sum += fib;
				}
			},
			false => break,
		};
	}

	println!("{:?}", fib_seq);

	sum
}