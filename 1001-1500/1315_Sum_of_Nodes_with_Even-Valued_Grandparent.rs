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
    fn f(r: &TreeNode, g1: bool, g2: bool) -> i32 {
        let mut s = if g1 { r.val } else { 0 };
        if let Some(c) = &r.left {
            s += Self::f(&c.borrow(), g2, r.val % 2 == 0);
        }
        if let Some(c) = &r.right {
            s += Self::f(&c.borrow(), g2, r.val % 2 == 0);
        }
        s
    }
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let r1 = root.unwrap();
        let r2 = r1.borrow();
        Self::f(&r2, false, false)
    }
}