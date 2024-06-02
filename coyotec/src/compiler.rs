use crate::lexer::{lex, SourceType};
use crate::parser::parse;
use crate::generator::generate;
use cyasm::assembler::assemble;

pub fn compile(code:&str, source_type:SourceType) -> Vec<u8> {
    let mut bytecode = Vec::new();
    if let Ok(tokens) = lex(code, source_type) {

        if let Some(node) = parse(&tokens) {
            let mut asm = String::new();
            generate(&node, &mut asm);
            bytecode = assemble(&asm);

        } else {
            panic!("Error parsing");
        }
    }
    bytecode
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

