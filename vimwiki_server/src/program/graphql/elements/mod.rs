use vimwiki::{elements, LC};

mod blocks;
pub use blocks::*;
mod comments;
pub use comments::*;
mod utils;
pub use utils::*;

/// Represents a single document page
#[derive(async_graphql::SimpleObject, Debug)]
pub struct Page {
    /// The elements contained within the page
    elements: Vec<BlockElement>,

    /// The comments contained within the page
    comments: Vec<Comment>,

    /// The area where the page resides
    region: Region,
}

impl From<LC<elements::Page>> for Page {
    fn from(mut lc: LC<elements::Page>) -> Self {
        let elements = lc
            .element
            .elements
            .drain(..)
            .map(BlockElement::from)
            .collect();
        let comments =
            lc.element.comments.drain(..).map(Comment::from).collect();
        let region = Region::from(lc.region);

        Self {
            elements,
            comments,
            region,
        }
    }
}