use std::fmt::Write;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser);
#[cfg(test)]
mod parser_tests;

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
    fn pretty_internal<Writer: Write>(
        &self,
        indent_level: usize,
        writer: &mut Writer,
    ) -> Result<(), std::fmt::Error> {
        indent(indent_level, writer)?;
        match self {
            Self::Lifetime(lifetime) => writer.write_str(lifetime),
            Self::Item(item) => item.pretty_internal(indent_level, writer),
            Self::Reference {
                lifetime,
                mut_,
                item,
            } => {
                writer.write_char('&')?;
                writer.write_str(lifetime)?;
                if *mut_ {
                    writer.write_str(" mut")?;
                }
                writer.write_char(' ')?;
                item.pretty_internal(indent_level, writer)
            }
            Self::Tuple(bounds) => {
                writer.write_str("(\n")?;
                for (idx, bound) in bounds.iter().enumerate() {
                    bound.pretty_internal(indent_level + 1, writer)?;
                    if idx != bounds.len() - 1 {
                        writer.write_char(',')?;
                    }
                    writer.write_char('\n')?;
                }
                indent(indent_level, writer)?;
                writer.write_char(')')
            }
        }
    }
}

/// A struct, trait, enum, or typedef.
///
/// Note that the name vector is backwards: `item[0]` is the item name; `item[1]` is the parent module, etc.
#[derive(Debug)]
pub struct Item<'a> {
    name: Vec<&'a str>,
    generic_bounds: Vec<Bound<'a>>,
}

impl<'a> Item<'a> {
    /// Pretty-print this Item into a new string.
    pub fn pretty(&self) -> String {
        let mut writer = String::new();
        self.pretty_internal(0, &mut writer)
            .expect("writing to String should never fail");
        writer
    }

    /// Pretty-print this Item to the supplied writer.
    pub fn pretty_to<Writer: Write>(&self, writer: &mut Writer) -> Result<(), std::fmt::Error> {
        self.pretty_internal(0, writer)
    }

    fn pretty_internal<Writer: Write>(
        &self,
        indent_level: usize,
        writer: &mut Writer,
    ) -> Result<(), std::fmt::Error> {
        for (idx, component) in self.name.iter().enumerate().rev() {
            writer.write_str(component)?;
            if idx != 0 {
                writer.write_str("::")?;
            }
        }
        if !self.generic_bounds.is_empty() {
            writer.write_str("<\n")?;
            for (idx, bound) in self.generic_bounds.iter().enumerate() {
                bound.pretty_internal(indent_level + 1, writer)?;
                if idx != self.generic_bounds.len() - 1 {
                    writer.write_char(',')?;
                }
                writer.write_char('\n')?;
            }
            indent(indent_level, writer)?;
            writer.write_char('>')?;
        }
        Ok(())
    }
}

const INDENT: &str = "  ";
fn indent<Writer: Write>(indent_level: usize, writer: &mut Writer) -> Result<(), std::fmt::Error> {
    for _ in 0..indent_level {
        writer.write_str(INDENT)?;
    }
    Ok(())
}
