use super::CompilationResult;

const WARNING: &'static str = "=Algumas instruções foram removidas por nunca serem chamadas";
const DEFAULT_FILEDATA: &'static str = "X,Y\n\n@\n-\nq0,qf\nq0\nqf\nq0,@,,qf,@,D,,,,,,,,,,,,,,,q0,@,,qf,@,D,,,,,,,,,,,,\nMáquina de Turing Vazia";
const DEFAULT_FILENAME: &'static str = "maqturing.mt"; 

use turing::TuringMachine;
mod turing;

use norma::{NormaRoutine, Instruction, DoKind, BoolKind};
mod norma;

pub fn get_filename(input: &str) -> String {
    if let Some(line) = input.lines().next() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") {
            let filename = trimmed[2..].trim();
            if !filename.is_empty() { return format!("{filename}.mt") }
        }
    }

    DEFAULT_FILENAME.to_string()
}

pub fn compile(input: &str) -> CompilationResult {
    let filename = get_filename(input);

    match NormaRoutine::parse(input) {
        Ok((norma, removed_instructions)) => {
            let turing_machine = TuringMachine::conjure_from(norma);
            let successful = true;
            let info = if let Some(removed) = removed_instructions {
                format!("Aviso#{}:\n{:?}", WARNING, removed)
            } else { "Ok#".to_string() };
            let filedata = if let Some(turing) = turing_machine {
                turing.to_string()
            } else {
                DEFAULT_FILEDATA.to_string()
            };

            CompilationResult { successful, info, filedata, filename }
        },
        Err(parse_error) => {
            let successful = false;
            let info = format!("Erro#{parse_error}");
            let filedata = DEFAULT_FILEDATA.to_string();
            CompilationResult { successful, info, filedata, filename }
        },
    } 
}
