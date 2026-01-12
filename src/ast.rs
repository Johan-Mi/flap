pub struct Tree {
    ops: Vec<crate::Op>,
    sizes: Vec<usize>,
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
        let tree = self.tree;
        let mut next = self.index + 1;
        let end = self.index + tree.sizes[self.index];
        core::iter::from_fn(move || (next != end).then(|| (next, next += tree.sizes[next]).0))
            .map(|index| Self { index, tree })
    }
}

pub struct Builder {
    tree: Tree,
    stack: Vec<usize>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            tree: Tree {
                ops: Vec::new(),
                sizes: Vec::new(),
            },
            stack: Vec::new(),
        }
    }
}

impl Builder {
    pub fn start_node(&mut self, op: crate::Op) {
        self.stack.push(self.tree.ops.len());
        self.tree.ops.push(op);
        self.tree.sizes.push(0);
    }

    pub fn finish_node(&mut self) {
        let entry = self.stack.pop().unwrap();
        self.tree.sizes[entry] = self.tree.ops.len() - entry;
    }

    pub fn parent(&self) -> crate::Op {
        self.tree.ops[*self.stack.last().unwrap()]
    }

    pub fn build(self) -> Tree {
        assert!(self.stack.is_empty());
        self.tree
    }
}
