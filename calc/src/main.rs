extern crate nom;
use nom::{character::complete::char, sequence::tuple, IResult};

fn parse_abc_to_ac(input: &str) -> IResult<&str, (char, char)> {
    tuple((char('a'), char('b'), char('c')))(input)
        .map(|(rest, result)| (rest, (result.0, result.2)))
}

fn main() {
    println!("abc: {:?}", parse_abc_to_ac("abc"));
}
