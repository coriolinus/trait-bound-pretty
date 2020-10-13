use crate::{
    parser::{E0277Parser, ItemParser, Token},
    Item, Pretty, E0277,
};
use lalrpop_util::ParseError;

pub type Error<'input> = ParseError<usize, Token<'input>, &'static str>;

/// Trait abstracting over a parser
pub trait Parser<'input> {
    fn parse(&self, input: &'input str) -> Result<'input dyn Pretty, Error<'input>>;
}

macro_rules! impl_parser {
    ($parser:ty => $output:ty) => {
        impl<'input> Parser<'input> for $parser {
            fn parse(&self, input: &'input str) -> Result<'input dyn Pretty, Error<'input>> {
                <$parser>::parse(self, input).map(|x| x as dyn Pretty)
            }
        }
    };
}

impl_parser!(ItemParser => Item<'input>);
impl_parser!(E0277Parser => E0277<'input>);
