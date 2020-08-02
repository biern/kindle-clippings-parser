use nom::{
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    character::complete::space1,
    combinator::map_res,
    multi::many1,
    sequence::{preceded, separated_pair, terminated, tuple},
};
use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Clipping {
    title: String,
    author: String,
    location: Location,
    text: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Location {
    from: u32,
    to: u32,
}

fn main() {
    let input = fs::read_to_string("My Clippings.txt").unwrap();

    let clippings = many1(parse_clipping)(&input);

    println!("{:?}", clippings);
}

fn parse_clipping(input: &str) -> nom::IResult<&str, Clipping> {
    let (input, (title, author)) = parse_title(input)?;
    let (input, location) = parse_location(input)?;
    let (input, text) = preceded(tag("\r\n"), parse_text)(input)?;

    return Ok((
        input,
        Clipping {
            title: title.into(),
            author: author.into(),
            location,
            text: text.into(),
        },
    ));
}

fn parse_title(input: &str) -> nom::IResult<&str, (&str, &str)> {
    let (input, line) = terminated(take_while(|c| c != '\r'), tag("\r\n"))(input)?;

    let split: Vec<_> = line.rsplitn(2, " (").take(2).collect();

    let title = split
        .get(1)
        .ok_or_else(|| nom::Err::Error((line, nom::error::ErrorKind::Tag)))?;

    let author = split
        .get(0)
        .map(|l| &l[0..l.len() - 1])
        .ok_or_else(|| nom::Err::Error((line, nom::error::ErrorKind::Tag)))?;

    Ok((input, (title, author)))
}

fn parse_location(input: &str) -> nom::IResult<&str, Location> {
    let (input, _) = tuple((tag("- Your Highlight at location"), space1))(input)?;
    let (input, (loc_from, loc_to)) = separated_pair(
        map_res(digit1, |d| u32::from_str_radix(d, 10)),
        tag("-"),
        map_res(digit1, |d| u32::from_str_radix(d, 10)),
    )(input)?;

    let (input, _) = terminated(take_while(|c| c != '\r'), tag("\r\n"))(input)?;

    Ok((
        input,
        Location {
            from: loc_from,
            to: loc_to,
        },
    ))
}

fn parse_text(input: &str) -> nom::IResult<&str, &str> {
    parse_until(tag("\r\n==========\r\n"))(input)
}

pub fn parse_until<'a, E: nom::error::ParseError<&'a str>, F>(
    terminator: F,
) -> impl Fn(&'a str) -> nom::IResult<&'a str, &'a str, E>
where
    F: Fn(&'a str) -> nom::IResult<&'a str, &'a str, E>,
{
    move |input: &str| {
        for c in 0..input.len() {
            let terminated = terminator(&input[c..]);

            if let Ok((remaining, _)) = terminated {
                return Ok((remaining, &input[..c]));
            }
        }

        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    }
}

mod test {
    use insta::assert_debug_snapshot;

    use super::*;

    const SINGLE_CLIPPING: &str = "Flow (Mihaly Csikszentmihalyi)\r
- Your Highlight at location 1213-1214 | Added on Sunday, 12 July 2015 17:36:17\r
\r
The reason it is possible to achieve such complete involvement in a flow experience is that goals are usually clear, and feedback immediate.\r
==========\r
";

    #[test]
    fn title() {
        assert_eq!(
            parse_title("Flow (Mihaly Csikszentmihalyi)\r\n"),
            Ok(("", ("Flow", "Mihaly Csikszentmihalyi")))
        );
    }

    #[test]
    fn title_with_parens() {
        assert_eq!(
            parse_title("Foo (Bar) Baz (Author)\r\n"),
            Ok(("", ("Foo (Bar) Baz", "Author")))
        );
    }

    #[test]
    fn location() {
        let res = parse_location(
            "- Your Highlight at location 1213-1214 | Added on Sunday, 12 July 2015 17:36:17\r\n",
        );

        assert_eq!(
            res,
            Ok((
                "",
                Location {
                    from: 1213,
                    to: 1214
                }
            ))
        );
    }

    #[test]
    fn text() {
        let res = parse_text("Foo bar baz.\r\n==========\r\n");

        assert_eq!(res, Ok(("", "Foo bar baz.".into(),)));
    }

    #[test]
    fn parse_single_clipping() {
        let res = parse_clipping(SINGLE_CLIPPING);

        assert_debug_snapshot!(res);
    }
}
