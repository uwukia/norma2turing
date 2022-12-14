use super::{Instruction, DoKind, BoolKind};

trait ParseSelf: Sized {
    fn retrieve(&mut self, tag: &str) -> Option<Self>;
    fn retrieve_next(&mut self) -> Option<char>;
    fn retrieve_any<'s, I: Iterator<Item=&'s str>>(&mut self, tags: I) -> Option<Self>;
    fn parse_while<F: FnMut(char) -> bool>(&mut self, pat: F) -> Option<Self>;

    fn ignore(&mut self, tag: &str) {
        self.retrieve(tag);
    }

    fn ignore_any<'s, I: Iterator<Item=&'s str>>(&mut self, tags: I) {
        self.retrieve_any(tags);
    }
}

impl<'s> ParseSelf for &'s str {
    fn retrieve(&mut self, tag: &str) -> Option<Self> {
        let trimmed = self.trim_start();
        
        if trimmed.starts_with(tag) {
            let sep = tag.len();
            *self = &trimmed[sep..];
            Some(&trimmed[..sep])
        } else {
            *self = trimmed;
            None
        }
    } 

    fn retrieve_next(&mut self) -> Option<char> {
        let trimmed = self.trim_start();
        let mut chars = trimmed.chars();

        let ret = chars.next();
        *self = chars.as_str();

        ret
    }

    fn retrieve_any<'i, I: Iterator<Item=&'i str>>(&mut self, mut tags: I) -> Option<Self> {
        let trimmed = self.trim_start();

        let parsed_tag = tags.find(|tag| trimmed.starts_with(tag));

        if let Some(tag) = parsed_tag {
            let sep = tag.len();
            *self = &trimmed[sep..];
            Some(&trimmed[..sep])
        } else {
            *self = trimmed;
            None
        }
    }

    fn parse_while<F: FnMut(char) -> bool>(&mut self, mut pat: F) -> Option<Self> {
        let trimmed = self.trim_start();
        let mut chars = trimmed.char_indices();
        let sep = chars.find(|(_, c)| !pat(*c)).map(|(i, _)| i).unwrap_or(trimmed.len());

        if sep != 0 {
            *self = &trimmed[sep..];
            Some(&trimmed[..sep])
        } else {
            *self = trimmed;
            None
        }
    }
}

pub fn parse_line<'s>(line_number: usize, input: &'s str) -> Result<(&'s str, Instruction<'s>), ParseError<'s>> {
    let parse_error = |line, kind| ParseError::new(line_number, input, line, kind);
    let line = &mut input.clone();

    let label = line.parse_while(|c: char| label_char(c))
        .ok_or_else(|| parse_error(line, ParseErrorKind::MissingLabel))?;
    line.retrieve(":")
        .ok_or_else(|| parse_error(line, ParseErrorKind::MissingColon))?;

    let instruction = parse_instruction(line_number, *line)
        .map_err(|err| err.parent(input))?;

    Ok((label, instruction))
}

fn parse_instruction<'s>(line_number: usize, input: &'s str) -> Result<Instruction<'s>, ParseError<'s>> {
    let parse_error = |line, kind| {
        ParseError::new(line_number, input, line, kind)
    };

    let line = &mut input.clone();
    
    line.ignore_any(["do", "if"].into_iter()); 
    let instruction = line.retrieve_any(["inc", "dec", "zero"].into_iter())
        .ok_or_else(|| parse_error(line, ParseErrorKind::UnknownInstruction))?;
    let register = line.retrieve_any(["X", "x", "Y", "y"].into_iter())
        .ok_or_else(|| parse_error(line, ParseErrorKind::UnknownRegister))?;

    line.ignore("then");
    line.retrieve("goto")
        .ok_or_else(|| parse_error(line, ParseErrorKind::MissingThen))?;
    let then_label = line.parse_while(|c: char| label_char(c))
        .ok_or_else(|| parse_error(line, ParseErrorKind::MissingThenLabel))?;

    let else_label = if instruction == "zero" {
        line.retrieve("else")
            .ok_or_else(|| parse_error(line, ParseErrorKind::MissingElse))?;
        line.retrieve("goto")
            .ok_or_else(|| parse_error(line, ParseErrorKind::MissingThen))?;
        line.parse_while(|c: char| label_char(c))
            .ok_or_else(|| parse_error(line, ParseErrorKind::MissingElseLabel))?
    } else { "" };

    Ok(get_instruction(instruction, register, then_label, else_label))
}

fn get_instruction<'s>(inst: &'s str, reg: &'s str, tlabel: &'s str, elabel: &'s str) -> Instruction<'s> {
    let joint = format!("{}{}", inst.to_lowercase(), reg.to_uppercase());
    match joint.as_str() {
        "incX" => Instruction::Operation(DoKind::IncX, tlabel),
        "incY" => Instruction::Operation(DoKind::IncY, tlabel),
        "decX" => Instruction::Operation(DoKind::DecX, tlabel),
        "decY" => Instruction::Operation(DoKind::DecY, tlabel),
        "zeroX" => Instruction::Boolean(BoolKind::ZeroX, tlabel, elabel),
        "zeroY" => Instruction::Boolean(BoolKind::ZeroY, tlabel, elabel),
        error => panic!("parsed invalid instruction {error}"),
    }
}

fn label_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.'
}

#[derive(Debug)]
pub struct ParseError<'e> {
    line_number: usize,
    input: &'e str,
    rest: &'e str,
    kind: ParseErrorKind,
}

impl<'e> ParseError<'e> {
    pub fn new(line_number: usize, input: &'e str, rest: &'e str, kind: ParseErrorKind) -> Self {
        Self { line_number, input, rest, kind }
    }

    pub fn parent(mut self, outer_input: &'e str) -> Self {
        self.input = outer_input;
        self
    }

    pub fn display_code(&self) -> String {
        let input = self.input;
        let sep = input.len() - self.rest.len();
        let pos = input.char_indices().take_while(|(i, _)| *i < sep).count();
        let spaces = std::iter::repeat(' ').take(pos).collect::<String>();
        format!("{input}\n{spaces}???")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    MissingLabel,
    MissingColon,
    UnknownInstruction,
    UnknownRegister,
    MissingThen,
    MissingThenLabel,
    MissingElse,
    MissingElseLabel,
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind, self.line_number + 1)
    }
}

use std::fmt;
impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseErrorKind::*;
        match self {
            MissingLabel       => write!(f, "E01"),
            MissingColon       => write!(f, "E02"),
            UnknownInstruction => write!(f, "E03"),
            UnknownRegister    => write!(f, "E04"),
            MissingThen        => write!(f, "E05"),
            MissingThenLabel   => write!(f, "E06"),
            MissingElse        => write!(f, "E07"),
            MissingElseLabel   => write!(f, "E08"),
        }
    }
} 

use super::NormaParseError;
pub fn display_verification_error(err: &NormaParseError<'_>) -> String {
    match err {
        NormaParseError::Parse(parse_error) => parse_error.to_string(),
        NormaParseError::LabelDeclaredTwice(label, line1, line2) => format!("EL2 {label} {line1} {line2}"),
        NormaParseError::InstructionCallsItself(label, line) => format!("ECI {label} {line}"),
        NormaParseError::TooManyInstructions => format!("ETM"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn error_kind() {

        use super::ParseErrorKind::*;

        let assert = |input: &str, error: super::ParseErrorKind| {
            assert_eq!(super::parse_line(0, input).unwrap_err().kind, error)
        };

        assert("->", MissingLabel);
        assert("_lbl0 stuff", MissingColon);
        assert("lbl : idk?", UnknownInstruction);
        assert("lbl: zeroZ", UnknownRegister);
        assert("lbl  :  inc  x  got", MissingThen);
        assert("lbl  :  decY  goto ", MissingThenLabel);
        assert("lbl  :  zero  X  goto label el", MissingElse);
        assert("lbl  :  zero  x  goto label else =?", MissingThen);
        assert("lbl  :  zero  x  goto label else goto =?", MissingElseLabel);
    }

    #[test]
    fn passes() {
        let assert_accepts = |input| assert!(super::parse_line(0, input).is_ok());

        assert_accepts("0: do inc X goto 1");
        assert_accepts("lbl : dec Y goto _");
        assert_accepts(" e: zerox then goto end else goto other  ");
    }
}
