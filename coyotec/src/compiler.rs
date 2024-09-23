use anyhow::{bail, Result};
use crate::lexer::{lex, SourceType};
use crate::parser::parse;
use crate::generator::generate;
use cyasm::assembler::assemble;

/// The compiler module is the entry point for the compiler. It takes a string of code
/// and returns a vector of bytes that represent the compiled code.
pub fn compile(code:&str, source_type:SourceType) -> Result<Vec<u8>> {
    // Empty vector to hold the compiled bytecode
    let mut bytecode = Vec::new();
    // Lex the code
    if let Ok(tokens) = lex(code, source_type) {
        // Parse the tokens
        if let Some(node) = parse(&tokens) {
            // Generate the assembly code
            let mut asm = String::new();
            generate(&node, &mut asm);

            println!("Generated asm...");
            println!("{}", asm);

            // Assemble the assembly code into bytecode
            bytecode = assemble(&asm);
        } else {
           bail!("Error parsing");
        }
    }
    Ok(bytecode)
}

#[cfg(test)]
mod test {
    use crate::lexer::{lex, SourceType};
    use crate::compiler::compile;

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

