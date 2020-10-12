use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);
#[cfg(test)]
mod parser_tests;

#[derive(Debug)]
pub enum Bound<'a> {
    Lifetime(&'a str),
    Item(Item<'a>),
    Reference {
        amp: &'a str,
        lifetime: &'a str,
        mut_: Option<&'a str>,
        item: Item<'a>,
    },
    Tuple(Vec<Bound<'a>>),
}

/// A struct, trait, enum, or typedef.
///
/// Note that the name vector is backwards: `item[0]` is the item name; `item[1]` is the parent module, etc.
#[derive(Debug)]
pub struct Item<'a> {
    name: Vec<&'a str>,
    generic_bounds: Vec<Bound<'a>>,
}
