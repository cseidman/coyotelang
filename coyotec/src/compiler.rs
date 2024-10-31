#![allow(unused_assignments, unused_variables)]
use crate::debug::display_ast;
use crate::generator::generate;
use crate::lexer::{lex, SourceType};
use crate::parse::parser::parse;
use anyhow::{bail, Result};
use cyasm::assembler::assemble;

/// The compiler module is the entry point for the compiler. It takes a string of code
/// and returns a vector of bytes that represent the compiled code.
pub fn compile(code: &str, source_type: SourceType) -> Result<Vec<u8>> {
    // Empty vector to hold the compiled bytecode
    let mut bytecode = Vec::new();
    let tokens = lex(code, source_type)?;

    let node = parse(&tokens);
    //node.unwrap().display_tree();
    // Lex the code

    // Parse the tokens
    if let Some(node) = parse(&tokens) {
        // Display the AST
        //display_ast(&node, 0);

        // Generate the assembly code
        //println!("Generating asm...");
        let instructions = generate(&node);
        let mut asm = String::new();
        for line in instructions {
            //println!("{}", line);
            asm.push_str(&line);
        }

        // Assemble the assembly code into bytecode
        bytecode = assemble(&asm);
    } else {
        bail!("Error parsing");
    }

    Ok(bytecode)
}

#[cfg(test)]
mod test {
    use crate::compiler::compile;
    use crate::lexer::SourceType;

    #[test]
    fn test_compile() {
        let code = "1 * 2 + 3 * 4";
        println!("Testing compile: {}", code);
        compile(code, SourceType::Interactive);

        let code = "1 + 2 * 3 + 4";
        println!("Testing compile: {}", code);
        compile(code, SourceType::Interactive);
    }
}
