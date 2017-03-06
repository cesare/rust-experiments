#[derive(Debug)]
pub struct Prime {
    known_primes: Vec<u64>,
    max_investigated_number: u64,
}

struct FiniteArithmeticProgression {
    current: u64,
    diff: u64,
    upto: u64,
}

impl FiniteArithmeticProgression {
    pub fn new(init: u64, diff: u64, upto: u64) -> FiniteArithmeticProgression {
        FiniteArithmeticProgression {
            current: init,
            diff: diff,
            upto: upto,
        }
    }
}

impl Iterator for FiniteArithmeticProgression {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next_value = self.current + self.diff;
        if next_value < self.upto {
            self.current = next_value;
            Some(next_value)
        } else {
            None
        }
    }
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

    fn next_candidates(&mut self) -> Vec<Candidate> {
        let from = self.max_investigated_number + 1;
        let to = from + 100;
        self.candidates(from, to)
    }

    fn candidates(&mut self, from: u64, to: u64) -> Vec<Candidate> {
        (from..to).map(|n| Candidate::new(n)).collect()
    }

    fn eliminate(candidates: &mut Vec<Candidate>, n: u64) {
        let first = candidates.first().unwrap().n;
        let offset = Prime::offset_to_nearest_multiple(n, first);
        let len = candidates.len() as u64;
        let seq = FiniteArithmeticProgression::new(offset, n, len);
        for i in seq {
            if let Some(candidate) = candidates.get_mut(i as usize) {
                candidate.is_prime = false
            }
        }
    }

    fn offset_to_nearest_multiple(n: u64, o: u64) -> u64 {
        let r = o % n;
        if r == 0 { 0 } else { n - r }
    }

    fn filter(&mut self, candidates: &mut Vec<Candidate>) -> Vec<u64> {
        for p in &self.known_primes {
            Prime::eliminate(candidates, *p);
        }

        let xs: Vec<u64> =
            candidates.iter().filter_map(|c| if c.is_prime { Some(c.n) } else { None }).collect();
        xs
    }
}

fn main() {
    let mut prime = Prime::new();
    let mut candidates = prime.next_candidates();
    let ps = prime.filter(&mut candidates);
    println!("{:?}", ps);
}
