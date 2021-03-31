use std::io;

fn main() {
    println!("Please enter a number");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let input: u16 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        prime_numbers_to_n(input);
        nth_prime_num(input);
        break;
    }
}

fn prime_numbers_to_n(n: u16) -> Vec<u16> {
    let mut nums = Vec::new();
    for x in 2..n {
        if is_prime(x) {
            nums.push(x);
        }
    }

    println!("There are {} prime numbers between 1 and {}", nums.len(), n);
    println!("The prime numbers are as follows:");
    println!("{:?}", nums);
    nums
}

fn is_prime(n: u16) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return n != 1;
}

fn nth_prime_num(n: u16) -> u16 {
    let mut nums = Vec::new();
    let mut counter = 2;
    while nums.len() < n.into() {
        if is_prime(counter) {
            nums.push(counter)
        }
        counter += 1
    }
    println!("The {}th prime number is {}", n, nums[nums.len() - 1]);
    nums[nums.len() - 1]
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn prime_nums_to_n() {
        assert_eq!(
            prime_numbers_to_n(100),
            [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
        assert_eq!(
            prime_numbers_to_n(500),
            [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257,
                263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353,
                359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449,
                457, 461, 463, 467, 479, 487, 491, 499
            ]
        )
    }

    #[test]
    fn is_prime_true() {
        assert_eq!(is_prime(97), true);
        assert_eq!(is_prime(67), true);
        assert_eq!(is_prime(499), true)
    }

    #[test]
    fn is_prime_false() {
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(100), false);
        assert_eq!(is_prime(133), false)
    }

    #[test]
    fn nth_prime_works() {
        assert_eq!(nth_prime_num(5), 11);
        assert_eq!(nth_prime_num(10), 29);
        assert_eq!(nth_prime_num(20), 71);
    }
}
