// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    fn f(r: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
        match r {
            Some(n) => {
                if n.borrow().left == None && n.borrow().right == None {
                    (Some(n), 1)
                } else {
                    let a = Self::f(n.borrow().left.clone());
                    let b = Self::f(n.borrow().right.clone());
                    if a.1 == b.1 {
                        (Some(n), a.1 + 1)
                    } else if a.1 > b.1 {
                        (a.0, a.1 + 1)
                    } else {
                        (b.0, b.1 + 1)
                    }
                }
            },
            None => (None, 0)
        }
    }
    pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let (a, _) = Self::f(root);
        a
    }
}