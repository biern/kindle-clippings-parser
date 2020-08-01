use indoc::indoc;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    character::complete::space1,
    combinator::map_res,
    sequence::{pair, terminated, tuple},
};

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
    println!("Hello, world!");
}

fn parse_clipping(input: &str) -> nom::IResult<&str, Clipping> {
    let (input, (title, author)) = parse_title(input)?;
    let (input, location) = parse_location(input)?;
    let (input, text) = parse_text(input)?;

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
    let (input, line) = take_while(|c| c != '\n')(input)?;

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
    let (input, (loc_from, _, loc_to)) = tuple((
        map_res(digit1, |d| u32::from_str_radix(d, 10)),
        tag("-"),
        map_res(digit1, |d| u32::from_str_radix(d, 10)),
    ))(input)?;

    let (input, _) = take_while(|c| c != '\n')(input)?;

    Ok((
        input,
        Location {
            from: loc_from,
            to: loc_to,
        },
    ))
}

fn parse_text(input: &str) -> nom::IResult<&str, &str> {
    terminated(take_while(|c| c != '\n'), tag("\n==========\n"))(input)
}

mod test {
    use super::*;

    const single_clipping_flow: &str = indoc! {"
Flow (Mihaly Csikszentmihalyi)
- Your Highlight at location 1213-1214 | Added on Sunday, 12 July 2015 17:36:17

The reason it is possible to achieve such complete involvement in a flow experience is that goals are usually clear, and feedback immediate.
==========
"};

    #[test]
    fn title() {
        assert_eq!(
            parse_title("Flow (Mihaly Csikszentmihalyi)\n"),
            Ok(("\n", ("Flow", "Mihaly Csikszentmihalyi")))
        );
    }

    #[test]
    fn title_with_parens() {
        assert_eq!(
            parse_title("Foo (Bar) Baz (Author)\n"),
            Ok(("\n", ("Foo (Bar) Baz", "Author")))
        );
    }

    #[test]
    fn location() {
        let res = parse_location(
            "- Your Highlight at location 1213-1214 | Added on Sunday, 12 July 2015 17:36:17\n",
        );

        assert_eq!(
            res,
            Ok((
                "\n",
                Location {
                    from: 1213,
                    to: 1214
                }
            ))
        );
    }

    #[test]
    fn text() {
        let res = parse_text("Foo bar baz.\n==========\n");

        assert_eq!(res, Ok(("", "Foo bar baz.",)));
    }
}
