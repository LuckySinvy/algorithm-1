use crate::tree::binary::node::Node;
use std::ptr::NonNull;

pub struct Tree<T> {
    pub root: Option<NonNull<Node<T>>>,
}

impl<T> Tree<T> {
    pub fn height(&self) -> usize {
        fn calc<T>(node: Option<NonNull<Node<T>>>) -> usize {
            node.map_or(0, |node| unsafe {
                let lh = calc((*node.as_ptr()).left);
                let rh = calc((*node.as_ptr()).right);
                1 + std::cmp::max(lh, rh)
            })
        }

        calc(self.root)
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Tree { root: None }
    }
}

impl<T> Drop for Tree<T> {
    fn drop(&mut self) {
        fn visitor<T>(p: Option<NonNull<Node<T>>>) {
            p.map(|p| {
                let p = unsafe { Box::from_raw(p.as_ptr()) };
                visitor(p.left);
                visitor(p.right);
            });
        }
        visitor(self.root);
    }
}
