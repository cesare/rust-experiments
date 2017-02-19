fn closed_range() {
    let ns: Vec<i32> = (1..10).collect();
    println!("closed_range: {:?}", ns);
}

fn half_open_range() {
    let range = 1..;
    let ns: Vec<i32> = range.take(5).collect();
    println!("half_open_range: {:?}", ns);
}

fn odd_numbers() {
    // for now, range.step_by(2) is not stable. So we need some workaorund.
    let range = 1..;
    let ns: Vec<i32> = range.map(|n| n * 2 - 1).take(10).collect();
    println!("odd_numbers: {:?}", ns);
}

fn main() {
    closed_range();
    half_open_range();
    odd_numbers();
}
