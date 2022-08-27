use std::{fmt, collections::HashMap};

use super::turing::instruction_len;

use parse::{parse_line, ParseError};
mod parse;

pub struct NormaRoutine<'s> {
    inner: Vec<(&'s str, Instruction<'s>)>
}

impl<'s> NormaRoutine<'s> {
    pub fn parse(input: &'s str) -> Result<(Self, Option<Vec<&'s str>>), NormaParseError<'s>> {
        let mut vec = Vec::new();

        let ignore_line = |line: &str| {
            let trim = line.trim_start();
            trim.starts_with("//") || trim.is_empty()
        };

        for (index, line) in input.lines().enumerate().filter(|(_, l)| !ignore_line(l)) {
            let (label, instruction) = parse_line(index, line)
                .map_err(|err| NormaParseError::from(err))?;

            verify(index, &vec, label)?;

            vec.push((label, instruction));
        }

        let removed_instructions = warn_unused(&mut vec)?;

        if vec.len() > 10_000 {
            return Err(NormaParseError::TooManyInstructions)
        }

        Ok((Self { inner: vec }, removed_instructions))
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns (total_length, instruction_positions)
    pub fn instruction_positions(&self) -> (usize, HashMap<&'s str, u16>) {
        let mut map = HashMap::with_capacity(self.inner.len());

        let mut offset = 1;
        for (label, instruction) in self.inner.iter() {
            map.insert(*label, offset);
            offset += instruction_len(instruction);
        }

        (offset as usize, map)
    }

    pub fn into_iter(self) -> impl Iterator<Item=(&'s str, Instruction<'s>)> {
        self.inner.into_iter()
    }
}

fn verify<'s>(index: usize, previous: &Vec<(&'s str, Instruction<'s>)>, label: &'s str)
    -> Result<(), NormaParseError<'s>>
{
    if let Some(pos) = previous.iter().position(|(lbl, _)| *lbl == label) {
        let prev = (pos + 1) as u16;
        let curr = (index + 1) as u16;
        return Err(NormaParseError::LabelDeclaredTwice(label, prev, curr))
    }

    Ok(())
}

fn warn_unused<'s>(instructions: &mut Vec<(&'s str, Instruction<'s>)>)
-> Result<Option<Vec<&'s str>>, NormaParseError<'s>>
{
    let mut map = HashMap::with_capacity(instructions.len());
    for (index, label) in instructions.iter().map(|(l, _)| *l).enumerate() {
        map.insert(label, index == 0);
    }
    
    let mut verify = |label: &'s str, other: &'s str, pos: u16|
        if other == label {
            Err(NormaParseError::InstructionCallsItself(label, pos))
        } else {
            if let Some(called) = map.get_mut(&other) {
                *called = true;
                Ok(())
            } else {
                // Err(NormaParseError::LabelDoesNotExist(label, pos))
                Ok(())
            }
        };

    for (index, (label, instruction)) in instructions.iter().enumerate() {
        let pos = (index + 1) as u16;
        match instruction {
            Instruction::Operation(_, thenl) => {
                verify(label, thenl, pos)?;
            },
            Instruction::Boolean(_, thenl, elsel) => {
                verify(label, thenl, pos)?;
                verify(label, elsel, pos)?;
            },
        }
    }

    let unused = instructions.iter().map(|(l, _)| *l)
                .filter(|label| !*map.get(label).unwrap())
                .collect::<Vec<&'s str>>();

    if unused.is_empty() {
        Ok(None)
    } else {
        let mut updated_vec = Vec::with_capacity(instructions.len() - unused.len());
        let iter = instructions.iter().copied().filter(|(l, _)| !unused.contains(l));
        updated_vec.extend(iter);
        *instructions = updated_vec;
        Ok(Some(unused))
    }
}

pub enum NormaParseError<'s> {
    Parse(ParseError<'s>),
    LabelDeclaredTwice(&'s str, u16, u16),
    // LabelDoesNotExist(&'s str, u16),
    InstructionCallsItself(&'s str, u16),
    TooManyInstructions,
}

impl<'s> NormaParseError<'s> {
    pub fn index_and_display(&self) -> (String, String) {
        let index = self.to_string();
        let display = if let Self::Parse(parse_error) = self {
            parse_error.display_code()
        } else {
            String::new()
        };

        (index, display)
    }
}

impl<'s> From<ParseError<'s>> for NormaParseError<'s> {
    fn from(parse: ParseError<'s>) -> NormaParseError<'s> {
        NormaParseError::Parse(parse)
    }
}

impl fmt::Display for NormaParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", parse::display_verification_error(self))
    }
}

impl fmt::Debug for NormaParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction<'s> {
    Operation(DoKind, &'s str),
    Boolean(BoolKind, &'s str, &'s str),
}

impl fmt::Display for Instruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operation(op, goto)   => write!(f, "do {op} goto {goto}"),
            Self::Boolean(op, thn, els) => write!(f, "do {op} then goto {thn} else goto {els}"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum DoKind {
    IncX,
    IncY,
    DecX,
    DecY,
}

macro_rules! impl_debug {
    ($obj:tt) => {
        impl fmt::Debug for $obj {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(self, f)
            }
        }
    };
}

impl fmt::Display for DoKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::IncX => "incX",
            Self::IncY => "incY",
            Self::DecX => "decX",
            Self::DecY => "decY",
        };

        write!(f, "{string}")
    }
}

#[derive(Clone, Copy)]
pub enum BoolKind {
    ZeroX,
    ZeroY,
}

impl fmt::Display for BoolKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Self::ZeroX => "zeroX",
            Self::ZeroY => "zeroY",
        };

        write!(f, "{string}")
    }
}

impl_debug!(DoKind);
impl_debug!(BoolKind);

#[cfg(test)]
mod tests {
    
    use super::NormaRoutine;

    #[test]
    fn warn_unused() {
        let (_, unused) = NormaRoutine::parse("a: dec X goto d\nb: inc Y goto e").unwrap();
        assert_eq!(unused, Some(vec!["b"]));
    }
}
