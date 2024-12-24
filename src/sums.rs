pub fn sum_numbers(n: usize) -> usize {
    //Sum of first n numbers = n(n+1)/2
    n * (n + 1) / 2
}

pub fn sum_squares(n: usize) -> usize {
    // https://en.wikipedia.org/wiki/Square_pyramidal_number
    //Sum of squares of first n numbers = n(n+1)(2n+1)/6
    n * (n + 1) * ((2 * n) + 1) / 6
}
