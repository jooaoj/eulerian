
pub fn natural_sum(target: i32) -> i32 {
	let mut sum: i32 = 0;

	for i in 0..target {
		if i % 3 == 0 || i % 5 == 0 {
			sum += i;
		}
	}

	sum
}