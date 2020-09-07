pub fn nth(n: u32) -> u32 {
    let mut prime = 2;
    let mut prime_count = 0;

    for x in 3.. {
        // n == 0 handles prime number 2
        if n == 0 || prime_count == n {
            break;
        }
        // 2 is the 0th prime number, no need to count it
        if x > 2 && is_prime(x) {
            prime = x;
            prime_count += 1;
        }
    }
    prime
}

fn is_prime(x: u32) -> bool {
    let is_not_even = || x % 2 != 0;
    let not_divisible_by_three = || sum_digits(x) % 3 != 0;

    let root = (x as f32).sqrt().round() as u32;
    let not_divisible = || !(2..(root + 1)).any(|y| x % (y) == 0);

    let a = is_not_even();
    let b = not_divisible_by_three();
    let c = not_divisible();
    x == 2 || x == 3 || (a && b && c)
}

fn sum_digits(x: u32) -> u32 {
    x.to_string()
        .chars()
        .map(|digit| digit.to_digit(10))
        .filter(|digit| digit.is_some())
        .map(|digit| digit.unwrap())
        .sum()
}
