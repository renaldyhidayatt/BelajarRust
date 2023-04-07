mod tree;

use draw::Canvas;

use std::{collections::HashSet, rc::Rc};

use tree::{Tree, TreeKind};

pub use self::tree::TreeColor;

#[derive(Default)]
pub struct Forest {
    cache: HashSet<Rc<TreeKind>>,
    trees: Vec<Tree>,
}

impl Forest {
    pub fn plant_tree(&mut self, x: u32, y: u32, color: TreeColor, name: String, data: String) {
        let tree_kind = TreeKind::new(color, name, data);

        self.cache.insert(Rc::new(tree_kind.clone()));

        let tree = Tree::new(x, y, self.cache.get(&tree_kind).unwrap().clone());

        self.trees.push(tree);
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for tree in &self.trees {
            tree.draw(canvas);
        }
    }

    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }
}
