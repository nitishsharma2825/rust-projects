use rustyline::DefaultEditor;

mod parser;
mod planner;
mod optimizer;
mod executor;
mod types;
mod storage;
mod database;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
            },
            Err(e) => {
                println!("{}", e);
                break
            }
        }
    }
}
