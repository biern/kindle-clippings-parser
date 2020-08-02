use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while},
    character::complete::digit1,
    character::complete::space1,
    combinator::{map, map_res, opt},
    multi::many1,
    sequence::{preceded, terminated, tuple},
};
use std::env;
use std::fs;
use std::string::String;

#[derive(Debug, PartialEq, Eq)]
struct Clipping {
    title: String,
    author: Option<String>,
    content: ClippingContent,
}

#[derive(Debug, PartialEq, Eq)]
enum ClippingContent {
    Highlight(ClippingHighlight),
    Note(ClippingNote),
    Bookmark(ClippingBookmark),
    ArticleClip(ClippingArticleClip),
}

#[derive(Debug, PartialEq, Eq)]
struct ClippingHighlight {
    location: Location,
    text: String,
}

#[derive(Debug, PartialEq, Eq)]
struct ClippingNote {
    location: Location,
    text: String,
}

#[derive(Debug, PartialEq, Eq)]
struct ClippingArticleClip {
    location: Location,
    text: String,
}

#[derive(Debug, PartialEq, Eq)]
struct ClippingBookmark {
    location: Location,
}

#[derive(Debug, PartialEq, Eq)]
struct Location {
    from: u32,
    to: Option<u32>,
    kind: LocationKind,
}

#[derive(Debug, PartialEq, Eq)]
enum LocationKind {
    Page,
    Location,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(args.get(1).unwrap()).unwrap();

    let parsed = many1(parse_clipping)(&input);

    match parsed {
        Ok((_, clippings)) => println!("{:?}", clippings),
        Err(e) => println!("Errors: {:}", e),
    }
}

fn parse_clipping(input: &str) -> nom::IResult<&str, Clipping> {
    let (input, (title, author)) = parse_title(input)?;
    let (input, content) = alt((
        parse_clipping_highlight,
        parse_clipping_note,
        parse_clipping_bookmark,
        parse_clipping_article_clip,
    ))(input)?;

    return Ok((
        input,
        Clipping {
            title: title.into(),
            author: author.map(String::from),
            content,
        },
    ));
}

fn parse_title(input: &str) -> nom::IResult<&str, (&str, Option<&str>)> {
    let (input, line) = terminated(take_while(|c| c != '\r'), tag("\r\n"))(input)?;

    let split: Vec<_> = line.rsplitn(2, " (").take(2).collect();

    if split.len() >= 2 {
        let title = split
            .get(1)
            .ok_or_else(|| nom::Err::Error((line, nom::error::ErrorKind::Tag)))?;

        let author = split
            .get(0)
            .map(|l| &l[0..l.len() - 1])
            .ok_or_else(|| nom::Err::Error((line, nom::error::ErrorKind::Tag)))?;

        Ok((input, (title, Some(author))))
    } else {
        Ok((input, (line, None)))
    }
}

fn parse_clipping_highlight(input: &str) -> nom::IResult<&str, ClippingContent> {
    let (input, (_, location, _)) =
        tuple((tag("- Your Highlight "), parse_location, space1))(input)?;

    let (input, _) = terminated(take_while(|c| c != '\r'), tag("\r\n\r\n"))(input)?;

    let (input, highlight) = parse_until(tag("\r\n==========\r\n"))(input)?;

    return Ok((
        input,
        ClippingContent::Highlight(ClippingHighlight {
            location,
            text: highlight.into(),
        }),
    ));
}

fn parse_clipping_note(input: &str) -> nom::IResult<&str, ClippingContent> {
    let (input, (_, location, _)) = tuple((tag("- Your Note "), parse_location, space1))(input)?;

    let (input, _) = terminated(take_while(|c| c != '\r'), tag("\r\n\r\n"))(input)?;

    let (input, note) = parse_until(tag("\r\n==========\r\n"))(input)?;

    return Ok((
        input,
        ClippingContent::Note(ClippingNote {
            text: note.into(),
            location,
        }),
    ));
}

fn parse_clipping_article_clip(input: &str) -> nom::IResult<&str, ClippingContent> {
    let (input, (_, location, _)) =
        tuple((tag("- Clip This Article "), parse_location, space1))(input)?;

    let (input, _) = terminated(take_while(|c| c != '\r'), tag("\r\n\r\n"))(input)?;

    let (input, text) = parse_until(tag("\r\n==========\r\n"))(input)?;

    return Ok((
        input,
        ClippingContent::ArticleClip(ClippingArticleClip {
            text: text.into(),
            location,
        }),
    ));
}

fn parse_clipping_bookmark(input: &str) -> nom::IResult<&str, ClippingContent> {
    let (input, (_, location, _)) =
        tuple((tag("- Your Bookmark "), parse_location, space1))(input)?;

    let (input, _) = terminated(take_while(|c| c != '\r'), tag("\r\n"))(input)?;
    let (input, _) = parse_until(tag("\r\n==========\r\n"))(input)?;

    return Ok((
        input,
        ClippingContent::Bookmark(ClippingBookmark { location }),
    ));
}

fn parse_location(input: &str) -> nom::IResult<&str, Location> {
    let (input, (kind, _, from, to)) = tuple((
        map(
            alt((tag_no_case("at location"), tag_no_case("on page"))),
            |s: &str| s.to_ascii_lowercase(),
        ),
        tag(" "),
        map_res(digit1, |d| u32::from_str_radix(d, 10)),
        opt(preceded(
            tag("-"),
            map_res(digit1, |d| u32::from_str_radix(d, 10)),
        )),
    ))(input)?;

    Ok((
        input,
        Location {
            kind: match kind.as_ref() {
                "at location" => LocationKind::Location,
                "on page" => LocationKind::Page,
                _ => panic!(format!("Unexpected tag {}", kind)),
            },
            from,
            to,
        },
    ))
}

pub fn parse_until<'a, E: nom::error::ParseError<&'a str>, F>(
    terminator: F,
) -> impl Fn(&'a str) -> nom::IResult<&'a str, &'a str, E>
where
    F: Fn(&'a str) -> nom::IResult<&'a str, &'a str, E>,
{
    move |input: &str| {
        for (i, _c) in input.char_indices() {
            let terminated = terminator(&input[i..]);

            if let Ok((remaining, _)) = terminated {
                return Ok((remaining, &input[..i]));
            }
        }

        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use super::*;

    const SINGLE_HIGHLIGHT: &str = "Flow (Mihaly Csikszentmihalyi)\r
- Your Highlight at location 1213-1214 | Added on Sunday, 12 July 2015 17:36:17\r
\r
The reason it is possible to achieve such complete involvement in a flow experience is that goals are usually clear, and feedback immediate.\r
==========\r
";

    const SINGLE_NOTE: &str = "Flow (Mihaly Csikszentmihalyi)\r
- Your Note at location 1213 | Added on Sunday, 12 July 2015 17:36:17\r
\r
Yada yada ya.\r
==========\r
";

    const SINGLE_BOOKMARK: &str = "Sapiens: A Brief History of Humankind (Harari, Yuval Noah)\r
- Your Bookmark at location 3883 | Added on Sunday, 22 October 2017 23:09:48\r
\r
\r
==========\r
";

    const SINGLE_ARTICLE_CLIP: &str = "crofflr 2015-08-07 (crofflr.com)\r
- Clip This Article at Location 228 | Added on Sunday, 9 August 2015 12:50:40\r
\r
Yada yada ya\r
==========\r
";

    #[test]
    fn title() {
        assert_eq!(
            parse_title("Flow (Mihaly Csikszentmihalyi)\r\n"),
            Ok(("", ("Flow", Some("Mihaly Csikszentmihalyi"))))
        );
    }

    #[test]
    fn title_with_parens() {
        assert_eq!(
            parse_title("Foo (Bar) Baz (Author)\r\n"),
            Ok(("", ("Foo (Bar) Baz", Some("Author"))))
        );
    }

    #[test]
    fn title_no_author() {
        assert_eq!(parse_title("Foo  \r\n"), Ok(("", ("Foo  ", None))));
    }

    #[test]
    fn location_range() {
        let res = parse_location("at location 1213-1214");

        assert_eq!(
            res,
            Ok((
                "",
                Location {
                    kind: LocationKind::Location,
                    from: 1213,
                    to: Some(1214)
                }
            ))
        );
    }

    #[test]
    fn location_single() {
        let res = parse_location("at location 1213");

        assert_eq!(
            res,
            Ok((
                "",
                Location {
                    kind: LocationKind::Location,
                    from: 1213,
                    to: None
                }
            ))
        );
    }

    #[test]
    fn location_page() {
        let res = parse_location("on page 1213-1214");

        assert_eq!(
            res,
            Ok((
                "",
                Location {
                    kind: LocationKind::Page,
                    from: 1213,
                    to: Some(1214)
                }
            ))
        );
    }

    #[test]
    fn text_utf8() {
        let res: nom::IResult<&str, &str> = parse_until(tag("xxx"))("’Foo bar baz’.xxx");

        assert_eq!(res, Ok(("", "’Foo bar baz’.".into(),)));
    }

    #[test]
    fn parse_single_clipping() {
        let res = parse_clipping(SINGLE_HIGHLIGHT);

        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_single_note() {
        let res = parse_clipping(SINGLE_NOTE);

        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_single_bookmark() {
        let res = parse_clipping(SINGLE_BOOKMARK);

        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_single_article_clip() {
        let res = parse_clipping(SINGLE_ARTICLE_CLIP);

        assert_debug_snapshot!(res);
    }
}
