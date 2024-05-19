use crate::ast::AstNode;
pub fn generate(ast: &dyn AstNode) {
    println!("Generating code...");
    let node = ast.as_any().downcast_ref().unwrap();
    match node {
        BinOperator => {
            println!("Generating code for BinOperator");
        },
        _ => {
            println!("Code generation not implemented for this node");
        }
    }
}