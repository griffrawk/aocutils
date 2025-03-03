#[derive(Debug)]
pub struct Fibseq {
    first_term: usize,
    second_term: usize,
}

impl Default for Fibseq {
    fn default() -> Self {
        Self::new()
    }
}

impl Fibseq {
    pub fn new() -> Fibseq {
        Fibseq {
            first_term: 1,
            second_term: 1,
        }
    }
}

impl Iterator for Fibseq {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.first_term;
        self.first_term = self.second_term;
        // Don't continue if next add would arithmetically overflow second_term
        match self.second_term.checked_add(ret) {
            Some(_) => {
                self.second_term += ret;
                Some(ret)
            }
            None => None,
        }
    }
}
