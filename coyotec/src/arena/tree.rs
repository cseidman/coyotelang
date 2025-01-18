pub const EXPR: usize = 1;
pub const STATEMENT: usize = 2;

pub struct ExprNode {
    node_id: usize,
    children: Vec<ExprNode>,
    parent_type: usize,
    parent_id: usize,
}

pub struct Tree {
    arena_type: Vec<Vec<usize>>,
}

impl Tree {
    pub fn new() -> Self {
        let arena_type = vec![Vec::<usize>::new(), Vec::<usize>::new()];
        Self { arena_type }
    }

    pub fn insert(&mut self, index: usize, element: usize) {
        self.arena_type[index].push(element);
    }
}
