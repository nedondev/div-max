extern crate num;
extern crate hex;
use num::bigint::BigUint;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Big int to compare with max 
   #[arg()]
   value: String,
}

fn main() {

    let args = Args::parse();
    let mut decoded= [0; 32];
    hex::decode_to_slice("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF", &mut decoded).expect("Decoding failed");
    let max_uint: BigUint = BigUint::from_bytes_be(&decoded);

    match args.value.parse::<BigUint>() {
       Ok(n)  => println!("{}", max_uint/n),
       Err(_) => println!("Error")
    }

}
