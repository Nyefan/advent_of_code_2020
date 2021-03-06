#[macro_use]
extern crate lazy_static;

use std::error::Error;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

pub mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    day_1::part_1::main()?;
    day_1::part_2::main()?;
    day_2::part_1::main()?;
    day_2::part_2::main()?;
    day_3::part_1::main()?;
    day_3::part_2::main()?;
    day_4::part_1::main()?;
    day_4::part_2::main()?;
    day_5::part_1::main()?;
    day_5::part_2::main()?;
    Ok(())
}
