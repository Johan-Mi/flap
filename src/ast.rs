pub struct Tree {
    ops: Vec<crate::Op>,
    ends: Vec<usize>,
}

impl Tree {
    pub const fn root(&self) -> Node<'_> {
        let tree = self;
        Node { index: 0, tree }
    }
}

#[derive(Clone, Copy)]
pub struct Node<'tree> {
    index: usize,
    tree: &'tree Tree,
}

impl Node<'_> {
    pub fn op(self) -> crate::Op {
        self.tree.ops[self.index]
    }

    pub fn children(self) -> impl Iterator<Item = Self> {
        let (tree, mut next, end) = (self.tree, self.index + 1, self.tree.ends[self.index]);
        core::iter::from_fn(move || (next != end).then(|| (next, next = tree.ends[next]).0))
            .map(|index| Self { index, tree })
    }
}

pub struct Builder {
    tree: Tree,
    stack: Vec<usize>,
}

impl Default for Builder {
    fn default() -> Self {
        let (ops, ends, stack) = (Vec::new(), Vec::new(), Vec::new());
        let tree = Tree { ops, ends };
        Self { tree, stack }
    }
}

impl Builder {
    pub fn start_node(&mut self, op: crate::Op) {
        self.stack.push(self.tree.ops.len());
        self.tree.ops.push(op);
        self.tree.ends.push(0);
    }

    pub fn finish_node(&mut self) {
        self.tree.ends[self.stack.pop().unwrap()] = self.tree.ops.len();
    }

    pub fn parent(&self) -> crate::Op {
        self.tree.ops[*self.stack.last().unwrap()]
    }

    pub fn build(self) -> Tree {
        assert!(self.stack.is_empty());
        self.tree
    }
}
