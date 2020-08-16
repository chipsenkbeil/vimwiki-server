// Imported for easier access by submodules
use super::components::{self, BlockComponent, InlineComponent, Page};
use nom::{
    branch::alt,
    combinator::{all_consuming, map},
    multi::many0,
    IResult,
};

mod headers;

/// Parses str slice into a wiki page
pub fn parse_str(input: &str) -> IResult<&str, Page> {
    page(input)
}

/// Parses entire vimwiki page
fn page(input: &str) -> IResult<&str, Page> {
    // Continuously parse input for new block components until we have nothing left (or we fail)
    map(all_consuming(many0(block_component)), Page::new)(input)
}

/// Parses a block component
fn block_component(input: &str) -> IResult<&str, BlockComponent> {
    // TODO: Remove duplicate header and add all other block components
    alt((
        map(headers::header, From::from),
        map(headers::header, From::from),
    ))(input)
}

/// Parses an inline component
fn inline_component(input: &str) -> IResult<&str, InlineComponent> {
    // TODO: Add all inline component parsers
    panic!("TODO: Implement");
}
