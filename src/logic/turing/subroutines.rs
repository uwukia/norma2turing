use super::{TransitionRow, TransitionNode, State, Symbol, Direction};

pub const INC_X_LEN: u16 = INC_X.len() as u16;
pub const INC_Y_LEN: u16 = INC_Y.len() as u16;
pub const DEC_X_LEN: u16 = DEC_X.len() as u16;
pub const DEC_Y_LEN: u16 = DEC_Y.len() as u16;
pub const ZERO_X_LEN: u16 = ZERO_X.len() as u16;
pub const ZERO_Y_LEN: u16 = ZERO_Y.len() as u16;

const INC_X: [TransitionRow; 3] = map3([b"   0XD1XD2XE", b"      1YD2YE", b"f@D2XE2YE   "]);
const INC_Y: [TransitionRow; 2] = map2([b"   0XD0YD1YE", b"f@D1XE1YE   "]);
const DEC_X: [TransitionRow; 5] = map5([b"   1XD4YE4-E", b"   1XD1YD2-E", b"   4-E3-E   ", b"   4YE3YE   ", b"f@D4XE      "]);
const DEC_Y: [TransitionRow; 3] = map3([b"   0XD0YD1-E", b"f@D2XE2-E   ", b"f@D2XE2YE   "]);
const ZERO_X: [TransitionRow; 3] = map3([b"   2XE1YE1-E", b"f@D         ", b"f@D         "]);
const ZERO_Y: [TransitionRow; 3] = map3([b"   0XD2YE1-E", b"f@D1XE      ", b"f@D2XE      "]);

pub fn inc_x(offset: u16, end: State) -> Vec<TransitionRow> {
    INC_X.iter().map(|row| row.map(offset, end)).collect()
}

pub fn inc_y(offset: u16, end: State) -> Vec<TransitionRow> {
    INC_Y.iter().map(|row| row.map(offset, end)).collect()
}

pub fn dec_x(offset: u16, end: State) -> Vec<TransitionRow> {
    DEC_X.iter().map(|row| row.map(offset, end)).collect()
}

pub fn dec_y(offset: u16, end: State) -> Vec<TransitionRow> {
    DEC_Y.iter().map(|row| row.map(offset, end)).collect()
}

pub fn zero_x(offset: u16, end_t: State, end_f: State) -> Vec<TransitionRow> {
    vec![ZERO_X[0].map(offset, State::End), ZERO_X[1].map(offset, end_t), ZERO_X[2].map(offset, end_f)]
}

pub fn zero_y(offset: u16, end_t: State, end_f: State) -> Vec<TransitionRow> {
    vec![ZERO_Y[0].map(offset, State::End), ZERO_Y[1].map(offset, end_t), ZERO_Y[2].map(offset, end_f)]
}

const fn map2(array: [&[u8; 12]; 2]) -> [TransitionRow; 2] {
    [parse_row(*array[0]), parse_row(*array[1])]
}

const fn map3(array: [&[u8; 12]; 3]) -> [TransitionRow; 3] {
    [parse_row(*array[0]), parse_row(*array[1]), parse_row(*array[2])]
}

const fn map5(array: [&[u8; 12]; 5]) -> [TransitionRow; 5] {
    [parse_row(*array[0]), parse_row(*array[1]), parse_row(*array[2]), parse_row(*array[3]), parse_row(*array[4])]
}

const fn parse_row(input: [u8; 12]) -> TransitionRow {
    let start = parse_cell([input[0], input[1], input[2]]);
    let symbx = parse_cell([input[3], input[4], input[5]]);
    let symby = parse_cell([input[6], input[7], input[8]]);
    let blank = parse_cell([input[9], input[10], input[11]]);
    TransitionRow { start, symbx, symby, blank }
}

const fn parse_cell(input: [u8; 3]) -> Option<TransitionNode> {
    if input[1] != 32 {
        let state = match input[0] {
            48 => State::Num(0),
            49 => State::Num(1),
            50 => State::Num(2),
            51 => State::Num(3),
            52 => State::Num(4),
            102 => State::End,
            _ => unreachable!(),
        };

        let write = match input[1] {
            64 => Symbol::Start,
            88 => Symbol::SymbX,
            89 => Symbol::SymbY,
            45 => Symbol::Blank,
            _ => unreachable!(),
        };

        let goto = match input[2] {
            69 => Direction::Left,
            68 => Direction::Right,
            _ => unreachable!(),
        };

        Some(TransitionNode { state, write, goto })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn works() {
        super::map3([b"   0XD1XD2XE", b"      1YD2YE", b"f@D2XE2YE   "]);
        super::map2([b"   0XD0YD1YE", b"f@D1XE1YE   "]);
        super::map5([b"   1XD4YE4-E", b"   1XD1YD2-E", b"   4-E3-E   ", b"   4YE3YE   ", b"f@D4XE      "]);
        super::map3([b"   0XD0YD1-E", b"f@D2XE2-E   ", b"f@D2XE2YE   "]);
        super::map3([b"   2XE1YE1-E", b"f@D         ", b"f@D         "]);
        super::map3([b"   0XD2YE1-E", b"f@D1XE      ", b"f@D2XE      "]);
    }
}
