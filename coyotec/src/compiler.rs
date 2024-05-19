use crate::lexer::{lex, SourceType};
use crate::parser::parse;

pub fn compile(code:&str, source_type:SourceType) {
    if let Ok(tokens) = lex(code, source_type) {

        if let Some(node) = parse(&tokens) {
            node.visit();
        } else {
            println!("Error parsing");
        }

    }
}

#[cfg(test)]
mod test {
    use crate::lexer::{lex, SourceType};
    use crate::compiler::compile;

    #[test]
    fn test_compile() {
        let code = "1 * 2 + 3 * 4";
        compile(code, SourceType::Interactive);

        let code = "1 + 2 * 3 + 4";
        compile(code, SourceType::Interactive);
    }
}

