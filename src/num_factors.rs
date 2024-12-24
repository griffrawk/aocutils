use crate::prime_factors::prime_factors;
use itertools::Itertools;
use std::collections::HashMap;

pub fn num_factors_alt(of: usize) -> usize {
    // Return number of factors of natural number
    // See CompProg book p.198
    // If the prime factorization of n is given by:
    //      n = P1^v1 * P1^v2 ... Pk^vk
    // where P is a prime, and v is its power, then the number of factors, d, is given by:
    //      d(n) = (v1+1)(v2+1)...(vk+1)

    prime_factors(of)
        .iter()
        .fold(HashMap::<usize, usize>::new(), |mut m, &prime| {
            *m.entry(prime).or_default() += 1;
            m
        })
        .iter()
        .fold(1, |mut d, (_, &power)| {
            d *= power + 1;
            d
        })
}

pub fn num_factors(of: usize) -> usize {
    prime_factors(of)
        .into_iter()
        .group_by(|&a| a)
        .into_iter()
        .fold(1, |mut d, (_, prime)| {
            d *= prime.count() + 1;
            d
        })
}
