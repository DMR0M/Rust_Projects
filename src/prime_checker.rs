pub fn prime_checker(num: i32) -> bool {
    let mut factors = 0;
    for n in 1..=num {
        if num % n == 0 {
            factors += 1;
        }
    }
    factors <= 2
}
