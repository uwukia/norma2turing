const DEFAULT_STARTER: &'static str = "X,Y\n\n@\n-";

use super::{NormaRoutine, Instruction, DoKind, BoolKind};
use std::{iter, fmt};

pub fn instruction_len(instruction: &Instruction) -> u16 {
    match instruction {
        Instruction::Operation(do_kind, _) => match do_kind {
            DoKind::IncX => subroutines::INC_X_LEN,  
            DoKind::IncY => subroutines::INC_Y_LEN,  
            DoKind::DecX => subroutines::DEC_X_LEN,  
            DoKind::DecY => subroutines::DEC_Y_LEN,  
        },
        Instruction::Boolean(bool_kind, _, _) => match bool_kind {
            BoolKind::ZeroX => subroutines::ZERO_X_LEN,
            BoolKind::ZeroY => subroutines::ZERO_Y_LEN,
        },
    }
}

mod subroutines;

pub struct TuringMachine {
    table: TransitionFunction,
    documentation: String,
}

impl TuringMachine {
    /// Returns None if `norma.is_empty()`
    pub fn conjure_from(norma: NormaRoutine) -> Option<Self> {
        if norma.is_empty() { return None }

        let (total, instruction_pos) = norma.instruction_positions();
        let num_len = (total - 1).to_string().len();

        let mut table = TransitionFunction::new(total);
        let mut documentation = String::new();

        for (label, instruction) in norma.into_iter() {
            match instruction {
                Instruction::Operation(op, goto) => {
                    let state = instruction_pos.get(goto).map_or(State::End, |val| State::Num(*val));
                    let (start, end) = table.add_operation(op, state);
                    let str_start = State::Num(start).display(num_len);
                    let str_end   = State::Num(end).display(num_len);

                    let doc_line = format!("{str_start}-{str_end} => {label}: {instruction}\n");
                    documentation.push_str(&doc_line);
                },
                Instruction::Boolean(op, then_goto, else_goto) => {
                    let then_state = instruction_pos.get(then_goto).map_or(State::End, |val| State::Num(*val));
                    let else_state = instruction_pos.get(else_goto).map_or(State::End, |val| State::Num(*val));
                    let (start, end) = table.add_boolean(op, then_state, else_state);
                    let str_start = State::Num(start).display(num_len);
                    let str_end   = State::Num(end).display(num_len);

                    let doc_line = format!("{str_start}-{str_end} => {label}: {instruction}\n");
                    documentation.push_str(&doc_line);
                },
            }
        }

        Some(Self { table, documentation })
    }

    pub fn list_states(&self) -> String {
        let last_before_final = self.table.last_before_final();
        let num_len = last_before_final.to_string().len();

        (0..=last_before_final).map(|val| State::Num(val))
            .chain(iter::once(State::End))
            .map(|state| state.display(num_len))
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn starter_and_final(&self) -> String {
        let num_len = self.table.last_before_final().to_string().len();
        let starter = State::Num(0).display(num_len);
        let final_state = State::End.display(num_len);
        format!("{starter}\n{final_state}")
    }
}

impl fmt::Display for TuringMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let states = self.list_states();
        let bounds = self.starter_and_final();
        write!(f, "{DEFAULT_STARTER}\n{states}\n{bounds}\n{}\n{}", self.table, self.documentation)
    }
}

struct TransitionFunction {
    rows: Vec<TransitionRow>
}

impl TransitionFunction {
    pub fn new(len: usize) -> Self {
        let mut vec = Vec::with_capacity(len);
        vec.push(TransitionRow::starter());
        Self { rows: vec }
    }

    pub fn add_operation(&mut self, op: DoKind, end: State) -> (u16, u16) {
        let offset = self.rows.len() as u16;
        let mut rows = match op {
            DoKind::IncX => subroutines::inc_x(offset, end),
            DoKind::IncY => subroutines::inc_y(offset, end),
            DoKind::DecX => subroutines::dec_x(offset, end),
            DoKind::DecY => subroutines::dec_y(offset, end),
        };

        let start = offset;
        let end = offset + (rows.len() as u16 - 1);

        self.rows.append(&mut rows);
        (start, end)
    }

    pub fn add_boolean(&mut self, op: BoolKind, end_t: State, end_f: State) -> (u16, u16) {
        let offset = self.rows.len() as u16;
        let mut rows = match op {
            BoolKind::ZeroX => subroutines::zero_x(offset, end_t, end_f),
            BoolKind::ZeroY => subroutines::zero_y(offset, end_t, end_f),
        };

        let start = offset;
        let end = offset + (rows.len() as u16 - 1);

        self.rows.append(&mut rows);
        (start, end)
    }

    pub fn last_before_final(&self) -> u16 {
        self.rows.len() as u16 - 1 
    }
}

impl fmt::Display for TransitionFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num_len = self.last_before_final().to_string().len();
        
        let string = self.rows.iter().enumerate()
            .map(|(idx, row)| row.display(State::Num(idx as u16), num_len))
            .collect::<Vec<String>>()
            .join("");
    
        let last_string = TransitionRow::empty().display(State::End, num_len);

        write!(f, "{string}{last_string}")
    }
}

#[derive(Debug)]
pub struct TransitionRow {
    start: Cell,
    symbx: Cell,
    symby: Cell,
    blank: Cell,
}

impl TransitionRow {
    pub fn starter() -> Self {
        let start = Some(TransitionNode { state: State::Num(1), write: Symbol::Start, goto: Direction::Right });
        Self { start, symbx: None, symby: None, blank: None }
    }

    pub fn empty() -> Self {
        Self { start: None, symbx: None, symby: None, blank: None }
    }

    pub fn map(&self, offset: u16, map_end: State) -> Self {
        let map_node = |node: &TransitionNode| 
            node.with_state(if let State::Num(val) = node.state { State::Num(val+offset) } else { map_end }); 

        let start = self.start.as_ref().map(map_node);
        let symbx = self.symbx.as_ref().map(map_node);
        let symby = self.symby.as_ref().map(map_node);
        let blank = self.blank.as_ref().map(map_node);

        Self { start, symbx, symby, blank }
    }

    pub fn display(&self, state: State, num_len: usize) -> String {
        let wrapper = |symb: Symbol, node: &TransitionNode| 
            format!("{},{symb},,{},,,", state.display(num_len), node.display(num_len));
        let start = self.start.as_ref().map(|node| wrapper(Symbol::Start, node)).unwrap_or_default();
        let symbx = self.symbx.as_ref().map(|node| wrapper(Symbol::SymbX, node)).unwrap_or_default();
        let symby = self.symby.as_ref().map(|node| wrapper(Symbol::SymbY, node)).unwrap_or_default();
        let blank = self.blank.as_ref().map(|node| wrapper(Symbol::Blank, node)).unwrap_or_default();
        format!("{start}{symbx}{symby}{blank}")
    }
}

type Cell = Option<TransitionNode>;

#[derive(Debug)]
struct TransitionNode {
    state: State,
    write: Symbol,
    goto: Direction,
}

impl TransitionNode {
    pub fn display(&self, num_len: usize) -> String {
        format!("{},{},{}", self.state.display(num_len), self.write, self.goto)
    }

    pub fn with_state(&self, new_state: State) -> Self {
        Self { state: new_state, write: self.write, goto: self.goto }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Num(u16),
    End,
}

impl State {
    pub fn display(&self, num_len: usize) -> String {
        match self {
            Self::Num(value) => format!("q{:01$}", value, num_len),
            Self::End => "qF".to_string()
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Symbol {
    Start,
    SymbX,
    SymbY,
    Blank,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Start => write!(f, "@"),
            Symbol::SymbX => write!(f, "X"),
            Symbol::SymbY => write!(f, "Y"),
            Symbol::Blank => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Left  => write!(f, "E"),
            Direction::Right => write!(f, "D"),
        }
    }
}
