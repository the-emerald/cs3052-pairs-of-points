use crate::point::Point;
use nom::multi::many1;
use nom::{IResult, error};
use nom::combinator::map_res;
use std::str::FromStr;
use nom::number::complete::double;
use nom::character::complete::{space1, digit1};
use nom::error::ErrorKind;
use nom::Err::Error;

fn point(input: &str) -> IResult<&str, Point> {
    let (input, x) = double(input)?;
    let (input, _) = space1(input)?;
    let (input, y) = double(input)?;

    Ok((input, Point {
        x,
        y
    }))
}

fn number_of_points(input: &str) -> IResult<&str, u64> {
    map_res(
        digit1,
        u64::from_str
    )(input)
}

pub fn parse_stdin(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, points_count) = number_of_points(input)?;
    let (input, points) = many1(point)(input)?;

    if points.len() != points_count as usize {
        return Err(Error(error::Error::new("number of points", ErrorKind::Count)));
    }

    Ok((input, points))
}
