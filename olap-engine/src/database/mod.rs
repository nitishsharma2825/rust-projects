use std::fmt::Error;

use crate::{parser::SQLParser, types::Chunk};

pub struct Database {
    parser: SQLParser,
    planner: Planner,
    optimizer: Optimizer,
    executor: Executor,
}

impl Database {
    pub fn new() -> Result<Database, String> {
        let parser = SQLParser::new();
    }

    pub fn execute() -> Result<Chunk, Error> {
    
    }
}