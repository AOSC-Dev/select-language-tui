use std::str::Utf8Error;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
    combinator::{map, map_res},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

type Language<'a> = (&'a [u8], &'a [u8], &'a [u8]);

#[inline]
fn line_rest(input: &[u8]) -> IResult<&[u8], ()> {
    map(take_until("\n"), |_| ())(input)
}

#[inline]
fn comment(input: &[u8]) -> IResult<&[u8], ()> {
    map(terminated(tag("#"), line_rest), |_| ())(input)
}

#[inline]
fn whitespace(input: &[u8]) -> IResult<&[u8], ()> {
    alt((map(multispace1, |_| ()), comment))(input)
}

#[inline]
fn hr(input: &[u8]) -> IResult<&[u8], ()> {
    map(many0(whitespace), |_| ())(input)
}

#[inline]
fn languagelist_single_line(input: &[u8]) -> IResult<&[u8], Language> {
    let (input, (_, _, language_english, _, language, _, _, _, _, _, locale, _, _, _)) =
        tuple((
            take_until(";"),
            tag(";"),
            take_until(";"),
            tag(";"),
            take_until(";"),
            tag(";"),
            take_until(";"),
            tag(";"),
            take_until(";"),
            tag(";"),
            take_until(";"),
            tag(";"),
            take_until(";"),
            tag(";"),
        ))(input)?;

    Ok((input, (language, locale, language_english)))
}

pub fn parse_languagelist(input: &[u8]) -> IResult<&[u8], Vec<(&str, &str, &str)>> {
    many0(preceded(
        hr,
        map_res(languagelist_single_line, |v| {
            Ok::<(&str, &str, &str), Utf8Error>({
                (
                    std::str::from_utf8(v.0)?,
                    std::str::from_utf8(v.1)?,
                    std::str::from_utf8(v.2)?,
                )
            })
        }),
    ))(input)
}
