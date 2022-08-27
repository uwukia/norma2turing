// use instant::{Instant, Duration};
use super::CompilationResult;

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
    // let start_time = Instant::now();
    let filename = get_filename(input);

    match NormaRoutine::parse(input) {
        Ok((norma, removed_instructions)) => {
            // let parse_time = start_time.elapsed();

            let turing_machine = TuringMachine::conjure_from(norma);
            // let compile_time = start_time.elapsed();

            let successful = true;
            let (titlecode, errorcode, showcode) = if let Some(removed) = removed_instructions {
                let removed_string = removed.join(" ");
                ("WARN".to_string(), "W00".to_string(), removed_string)
            } else {
                ("OK".to_string(), "OK0".to_string(), "".to_string())
            };

            let filedata = if let Some(turing) = turing_machine {
                turing.to_string()
            } else {
                DEFAULT_FILEDATA.to_string()
            };

            let timetaken = display_time(); // display_time(parse_time, Some(compile_time));

            CompilationResult { successful, titlecode, errorcode, showcode, timetaken, filedata, filename }
        },
        Err(parse_error) => {
            // let parse_time = start_time.elapsed();

            let successful = false;
            let titlecode = "ERR".to_string();
            let (errorcode, showcode) = parse_error.index_and_display();
            let filedata = DEFAULT_FILEDATA.to_string();
            let timetaken = display_time(); // display_time(parse_time, None);

            CompilationResult { successful, titlecode, errorcode, showcode, timetaken, filedata, filename }
        },
    } 
}

fn display_time() -> String {
    String::new()
}