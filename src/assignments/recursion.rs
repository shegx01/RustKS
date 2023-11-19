pub fn sum_of_digits(num: u64) -> u64 {
    // for an input that is 21,
    // 1 + 2 + 0
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

