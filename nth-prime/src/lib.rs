fn is_prime(n: u32, prime_table: &Vec<u32>) -> bool {
    prime_table.iter()
        .filter(|p| (**p) <= ((n as f64).sqrt().floor() as u32))
        .all(|p| n % p != 0)
}

pub fn nth(n: u32) -> u32 {
    let mut prime_table = vec![2u32];

    for value in (3..).step_by(2) {
        if prime_table.len() > n as usize {
            // already got the nth prime number
            break;
        }

        if is_prime(value, &prime_table) {
            prime_table.push(value);
        }
    }
    
    *(prime_table.get(n as usize).unwrap())
}
