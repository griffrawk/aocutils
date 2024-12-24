use bit_set::BitSet;

#[derive(Debug)]
pub struct Sieve {
    bitset: BitSet,
    prime: usize,
    upper: usize,
}

impl Sieve {
    pub fn new(upper: usize) -> Sieve {
        let mut sieve = Sieve {
            bitset: BitSet::with_capacity(upper),
            prime: 1,
            upper,
        };
        sieve.bitset.clear();
        sieve
    }
}

impl Iterator for Sieve {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.prime += 1;
        while self.bitset.contains(self.prime) {
            self.prime += 1;
        }
        for i in (self.prime * 2..self.upper).step_by(self.prime) {
            self.bitset.insert(i);
        }
        // println!("{:?}", self);
        if self.prime < self.upper {
            Some(self.prime)
        } else {
            None
        }
    }
}
