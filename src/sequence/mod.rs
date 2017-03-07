use std::ops::AddAssign;

pub struct ArithmeticProgression<T> {
    current: T,
    diff: T,
}

impl<T> ArithmeticProgression<T> {
    pub fn new(init: T, diff: T) -> ArithmeticProgression<T> {
        ArithmeticProgression {
            current: init,
            diff: diff,
        }
    }
}

impl<T> Iterator for ArithmeticProgression<T>
    where T: AddAssign + Copy
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let current_value = self.current;
        self.current += self.diff;
        Some(current_value)
    }
}
