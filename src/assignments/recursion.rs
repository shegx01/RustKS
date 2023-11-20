pub fn sum_of_digits(num: u64) -> u64 {
    return if num == 0 {
        0
    } else {
        (num % 10) + sum_of_digits(num / 10)
    };

    // Elixir Version
    // require Integer
    // def sum_of_digit(n) when is_integer(n) and n >= 0, do: sum_digit(n)
    // defp sum_digit(0), do: 0
    // defp sum_digit(n), do: Integer.mod(n, 10) + sum_digit(div(n, 10))
}

pub fn sum_of_digits_optimized(n: u64) -> u64 {
    fn sum_digits(n: u64, acc: u64) -> u64 {
        return if n == 0 {
            acc
        } else {
            sum_digits(n / 10, acc + (n % 10))
        };
    }

    sum_digits(n, 0)
}

pub fn pow_rec(base: u64, exp: u32) -> u64 {
   return if exp == 0 { 1 }
    else { base * pow(base, exp -1) as u64 }
}

pub fn pow(base: u64, exp: u32) -> u64 {
    // For some reasons,
    // This is slower than the direct counterpart
    fn calc_pow(base: u64, exp: u32, result: u64) -> u64 {
        return if exp == 0 {
            result
        } else {
            calc_pow(base, exp - 1, result * base)
        }
    }

    calc_pow(base, exp, 1)
}
#[test]
fn sum_of_digits_rec_test() {
    assert_eq!(sum_of_digits(21), 3);
    assert_eq!(sum_of_digits(2221), 7);
    assert_eq!(sum_of_digits(7120), 10);
}
#[test]
fn sum_of_digits_optimized_test() {
    assert_eq!(sum_of_digits_optimized(21), 3);
    assert_eq!(sum_of_digits_optimized(2221), 7);
    assert_eq!(sum_of_digits_optimized(7120), 10);
}

#[test]
pub fn pow_test_rec() {
    assert_eq!(pow_rec(2, 4), 16);
    assert_eq!(pow_rec(3, 3), 27)
}


#[test]
pub fn pow_test() {
    assert_eq!(pow(2, 4), 16);
    assert_eq!(pow(3, 3), 27)
}
