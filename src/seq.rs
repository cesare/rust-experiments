mod sequence;
use sequence::ArithmeticProgression;

fn main() {
    let ap = ArithmeticProgression::new(0, 2);
    let ns = ap.take_while(|&n| n < 100).collect::<Vec<u32>>();
    println!("{:?}", ns);
}
