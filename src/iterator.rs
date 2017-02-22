use std::iter;

fn cycle() {
    let org = [false, true];
    let ns = (1..11).zip(org.iter().cycle());
    println!("{:?}", ns.collect::<Vec<(i32, &bool)>>());
}

fn zip() {
    let ts = iter::repeat(true);
    let zs = (1..10).zip(ts);
    println!("{:?}", zs.collect::<Vec<(i32, bool)>>());
}

fn main() {
    zip();
    cycle();
}
