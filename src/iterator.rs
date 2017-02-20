use std::iter;

fn zip() {
    let ts = iter::repeat(true);
    let zs = (1..10).zip(ts);
    println!("{:?}", zs.collect::<Vec<(i32, bool)>>());
}

fn main() {
    zip();
}
