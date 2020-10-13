use std::io::Write;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);
#[cfg(test)]
mod parser_tests;
pub mod parser_trait;

pub trait Pretty {
    /// Pretty-print `Self` to the supplied writer.
    ///
    /// This function must only ever write valid utf8 strings.
    fn pretty_to(&self, writer: &mut dyn Write) -> Result<(), std::io::Error>;

    /// Pretty-print `Self` into a new string.
    fn pretty(&self) -> String {
        let mut writer = Vec::new();
        self.pretty_to(&mut writer)
            .expect("writing to Vec<u8> should never fail");
        String::from_utf8(writer).expect("we only ever write valid utf8")
    }
}

#[derive(Debug)]
pub enum Bound<'a> {
    Lifetime(&'a str),
    Item(Item<'a>),
    Reference {
        lifetime: &'a str,
        mut_: bool,
        item: Item<'a>,
    },
    Tuple(Vec<Bound<'a>>),
}

impl<'a> Bound<'a> {
    fn pretty_internal<Writer: Write + ?Sized>(
        &self,
        indent_level: usize,
        writer: &mut Writer,
    ) -> Result<(), std::io::Error> {
        indent(indent_level, writer)?;
        match self {
            Self::Lifetime(lifetime) => write!(writer, "{}", lifetime),
            Self::Item(item) => item.pretty_internal(indent_level, writer),
            Self::Reference {
                lifetime,
                mut_,
                item,
            } => {
                write!(writer, "{}", '&')?;
                write!(writer, "{}", lifetime)?;
                if *mut_ {
                    write!(writer, "{}", " mut")?;
                }
                write!(writer, "{}", ' ')?;
                item.pretty_internal(indent_level, writer)
            }
            Self::Tuple(bounds) => {
                write!(writer, "{}", "(\n")?;
                for (idx, bound) in bounds.iter().enumerate() {
                    bound.pretty_internal(indent_level + 1, writer)?;
                    if idx != bounds.len() - 1 {
                        write!(writer, "{}", ',')?;
                    }
                    write!(writer, "{}", '\n')?;
                }
                indent(indent_level, writer)?;
                write!(writer, "{}", ')')
            }
        }
    }
}

/// A struct, trait, enum, typedef, or generic bound.
#[derive(Debug)]
pub struct Item<'a> {
    /// Note that the name vector is backwards: `item[0]` is the item name; `item[1]` is the parent module, etc.
    name: Vec<&'a str>,
    generic_bounds: Vec<Bound<'a>>,
}

impl<'a> Item<'a> {
    fn pretty_internal<Writer: Write + ?Sized>(
        &self,
        indent_level: usize,
        writer: &mut Writer,
    ) -> Result<(), std::io::Error> {
        for (idx, component) in self.name.iter().enumerate().rev() {
            write!(writer, "{}", component)?;
            if idx != 0 {
                write!(writer, "{}", "::")?;
            }
        }
        if !self.generic_bounds.is_empty() {
            write!(writer, "{}", "<\n")?;
            for (idx, bound) in self.generic_bounds.iter().enumerate() {
                bound.pretty_internal(indent_level + 1, writer)?;
                if idx != self.generic_bounds.len() - 1 {
                    write!(writer, "{}", ',')?;
                }
                write!(writer, "{}", '\n')?;
            }
            indent(indent_level, writer)?;
            write!(writer, "{}", '>')?;
        }
        Ok(())
    }
}

impl<'a> Pretty for Item<'a> {
    fn pretty_to(&self, writer: &mut dyn Write) -> Result<(), std::io::Error> {
        self.pretty_internal(0, writer)
    }
}

const INDENT: &str = "  ";
fn indent<Writer: Write + ?Sized>(indent_level: usize, writer: &mut Writer) -> Result<(), std::io::Error> {
    for _ in 0..indent_level {
        write!(writer, "{}", INDENT)?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct E0277<'a> {
    item: Item<'a>,
    trait_bound: Item<'a>,
}

impl<'a> Pretty for E0277<'a> {
    /// Pretty-print this Item to the supplied writer.
    fn pretty_to(&self, writer: &mut dyn Write) -> Result<(), std::io::Error> {
        write!(writer, "error[E0277]: the item:\n{}", INDENT)?;
        self.item.pretty_internal(1, writer)?;
        write!(writer, "\ndoes not satisfy the trait bound:\n{}", INDENT)?;
        self.trait_bound.pretty_internal(1, writer)?;
        Ok(())
    }
}
