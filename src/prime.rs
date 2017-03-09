use std::ops::Range;
mod sequence;
use sequence::ArithmeticProgression;

#[derive(Debug)]
pub struct Prime {
    known_primes: Vec<u64>,
    max_investigated_number: u64,
}

struct Candidate {
    n: u64,
    is_prime: bool,
}

impl Candidate {
    pub fn new(n: u64) -> Candidate {
        Candidate {
            n: n,
            is_prime: true,
        }
    }
}

impl Prime {
    pub fn new() -> Prime {
        Prime {
            known_primes: vec![2, 3, 5, 7, 11, 13, 17, 19],
            max_investigated_number: 20,
        }
    }

    fn max_known_prime(&self) -> u64 {
        *self.known_primes.last().unwrap()
    }

    fn next_candidates_range(&self) -> Range<u64> {
        let from = self.max_investigated_number + 1;
        let fixed = from + 1000;
        let limit = self.max_known_prime().pow(2);

        if limit < fixed {
            from..limit
        } else {
            from..fixed
        }
    }

    fn next_candidates(&self) -> Vec<Candidate> {
        let range = self.next_candidates_range();
        self.candidates(range.start, range.end)
    }

    fn candidates(&self, from: u64, to: u64) -> Vec<Candidate> {
        (from..to).map(|n| Candidate::new(n)).collect()
    }

    fn eliminate(candidates: &mut Vec<Candidate>, n: u64) {
        let first = candidates.first().unwrap().n;
        let last = candidates.last().unwrap().n;

        let offset = Prime::offset_to_nearest_multiple(n, first);
        let seq = ArithmeticProgression::new(offset, n);

        for i in seq.take_while(|&n| n <= last) {
            if let Some(candidate) = candidates.get_mut(i as usize) {
                candidate.is_prime = false
            }
        }
    }

    fn offset_to_nearest_multiple(n: u64, o: u64) -> u64 {
        let r = o % n;
        if r == 0 { 0 } else { n - r }
    }

    fn filter(&self, candidates: &mut Vec<Candidate>) -> Vec<u64> {
        for p in &self.known_primes {
            Prime::eliminate(candidates, *p);
        }

        let xs: Vec<u64> =
            candidates.iter().filter_map(|c| if c.is_prime { Some(c.n) } else { None }).collect();
        xs
    }

    fn proceed(&mut self) {
        let mut cs = self.next_candidates();
        let last = cs.last().unwrap().n;
        let ps = self.filter(&mut cs);

        self.known_primes.extend_from_slice(&ps);
        self.max_investigated_number = last;
    }
}

fn main() {
    let mut prime = Prime::new();
    prime.proceed();
    prime.proceed();
    println!("{:?}", prime.known_primes);
}
