pub mod part1;
pub mod part2;

use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    part2::main()
}
