use crate::geometry::Point;
use nom::character::complete::{digit1, newline, space1};
use nom::combinator::map_res;
use nom::error::ErrorKind;
use nom::multi::many1;
use nom::number::complete::double;
use nom::Err::Error;
use nom::{error, IResult};
use std::str::FromStr;

fn point(input: &str) -> IResult<&str, Point> {
    let (input, x) = double(input)?;
    let (input, _) = space1(input)?;
    let (input, y) = double(input)?;
    let (input, _) = newline(input)?;

    Ok((input, Point { x, y }))
}

fn number_of_points(input: &str) -> IResult<&str, u64> {
    map_res(digit1, u64::from_str)(input)
}

pub fn parse_stdin(input: &str) -> IResult<&str, Vec<Point>> {
    let (input, points_count) = number_of_points(input)?;
    let (input, _) = newline(input)?;
    let (input, points) = many1(point)(input)?;

    if points.len() != points_count as usize {
        return Err(Error(error::Error::new(
            "number of points",
            ErrorKind::Count,
        )));
    }

    Ok((input, points))
}
