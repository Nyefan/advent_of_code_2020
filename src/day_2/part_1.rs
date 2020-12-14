use super::PasswordSpecification;
use crate::utils::read_lines;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
    let count = read_lines("./inputs/2.input")?
        .flat_map(|line| line?.parse::<PasswordSpecification>())
        .filter(validate)
        .count();
    println!("Valid Password Count: {}", count);
    Ok(())
}

fn validate(ps: &PasswordSpecification) -> bool {
    let count = ps.password.chars().filter(|c| c.eq(&ps.letter)).count() as i32;
    return count >= ps.range.0 && count <= ps.range.1;
}
