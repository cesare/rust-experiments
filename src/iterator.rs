use std::iter;

fn filter_map() {
    fn convert(x: u32) -> Option<f32> {
        if x % 2 == 0 {
            Some(x as f32 * 1.2)
        } else {
            None
        }
    }

    let xs = 1..10;
    let ys = xs.filter_map(convert).collect::<Vec<f32>>();
    println!("{:?}", ys);
}

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
    filter_map();
}
