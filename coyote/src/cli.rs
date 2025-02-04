#![allow(unused_variables, unused_assignments)]

use anyhow::{bail, Result};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use clap::Parser;
use colored::Colorize;
use coyotec::ast::node::NodeType;
use coyotec::ast::Node;
use coyotec::compiler::compile;
use coyotec::generator::IrGenerator;
use coyotec::lexer::{lex, SourceType};
use coyotec::parse::parser;
use coyotec::tokens::Token;
use cvm::vm;
use cvm::vm::Vm;
use cyasm::assembler::assemble;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

/// My Application does awesome things
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Loads a file
    #[clap(short, long, value_parser)]
    file: Option<String>,

    /// Runs in debug mode
    #[clap(short, long, action)]
    debug: bool,

    /// Generates bytecode
    #[clap(short = 'c', long, action)]
    bytecode: bool,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // Check for file loading
    if let Some(file) = &cli.file {
        println!("Loading file: {}", file);
        let bytecode = load_file(file)?;
        vm::execute(bytecode);
    }

    // Check if debug mode is enabled
    if cli.debug {
        println!("Debug mode is enabled.");
        // Add your debug related logic here
    }

    // Check if bytecode generation is requested
    if cli.bytecode {
        println!("Bytecode generation is enabled.");
        // Add your bytecode generation logic here
    }

    // If no flags are provided, launch REPL
    if cli.file.is_none() && !cli.debug && !cli.bytecode {
        println!("Launching REPL...");
        repl()?;
        // Add your REPL launching logic here
    }
    Ok(())
}

fn load_file(file: &str) -> Result<Vec<u8>> {
    let contents = std::fs::read_to_string(file)?;
    compile(&contents, SourceType::File(file.to_string()))
}

fn repl<'a>() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut vm = Vm::new();
    let ast: Node = Node::new(NodeType::Root, Default::default());
    let mut generator = IrGenerator::new(&ast);
    let mut parser = parser::Parser::new(vec![], "".to_string());
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                println!("{} {}", "line:".red(), line.yellow());
                if line == "exit" || line == "quit" || line == "/q" {
                    break;
                }
                tokens = lex(&line, SourceType::Interactive)?;
                parser.add_tokens(tokens, line);

                if let Ok(node) = parser.parse() {
                    coyotec::ast::node::display_tree(&node);

                    // Generate the assembly code
                    generator.generate(&node);
                    let asm = format!("{}", generator);

                    println!("== ASM ==");
                    println!("{}", asm);
                    // Assemble the assembly code into bytecode
                    let bytecode = assemble(&asm);
                    vm.add_code(bytecode);
                    //println!("{:?}", &vm.code);
                    vm.run()
                } else {
                    bail!("Error parsing");
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")?;
    Ok(())
}
