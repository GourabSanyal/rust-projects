// Define a function named `factorial` that, given a non-negative integer `n`,
// returns `n!`, the factorial of `n`.
//
// The factorial of `n` is defined as the product of all positive integers up to `n`.
// For example, `5!` (read "five factorial") is `5 * 4 * 3 * 2 * 1`, which is `120`.
// `0!` is defined to be `1`.
//
// We expect `factorial(0)` to return `1`, `factorial(1)` to return `1`,
// `factorial(2)` to return `2`, and so on.
//
// Use only what you learned! No loops yet, so you'll have to use recursion!

fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        // Recursive step: For any n > 1, the factorial is
        // n multiplied by the factorial of (n - 1)
        n * factorial(n - 1)
    }
}

// fn factorial(n: u32) -> u32 {
//     let mut result: u32 = 1;
//     let mut i: u32 = 1;

//     while i <= n {
//         result *= 1;
//         i += 1;
//     }
//     result
// }

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
