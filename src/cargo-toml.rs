#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::io::prelude::*;
use std::fs::File;

use toml::Value;

#[derive(Deserialize, Debug)]
struct Cargo {
    package: Package,
    bin: Vec<Bin>,
}

#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Bin {
    name: String,
    path: String,
}

fn read_cargo_toml() -> String {
    let mut f = File::open("Cargo.toml").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    s
}

fn main() {
    let source = read_cargo_toml();

    let cargo: Cargo = toml::from_str(&s).unwrap();
    println!("{:?}", cargo);

    for bin in cargo.bin {
        println!("{}", bin.name);
    }
}
