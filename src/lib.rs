use std::io::Write;

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
    /// Pretty-print this Item into a new string.
    pub fn pretty(&self) -> String {
        let mut writer = Vec::new();
        self.pretty_internal(0, &mut writer)
            .expect("writing to Vec<u8> should never fail");
        String::from_utf8(writer).expect("we only ever write valid utf8")
    }

    /// Pretty-print this Item to the supplied writer.
    pub fn pretty_to<Writer: Write>(&self, writer: &mut Writer) -> Result<(), std::io::Error> {
        self.pretty_internal(0, writer)
    }

    fn pretty_internal<Writer: Write>(
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

const INDENT: &str = "  ";
fn indent<Writer: Write>(indent_level: usize, writer: &mut Writer) -> Result<(), std::io::Error> {
    for _ in 0..indent_level {
        write!(writer, "{}", INDENT)?;
    }
    Ok(())
}
