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
    fn f(r: & Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match r {
            Some(t) => {
                let n = t.borrow();
                let (a1, a2) = Self::f(&n.left);
                let (b1, b2) = Self::f(&n.right);
                let mut x1 = a1 + b1;
                let mut x2 = (a2 + b2) * 2;
                if n.left == None && n.right == None {
                    x2 += 1;
                }
                x1 += x2 * n.val;
                (x1, x2)
            },
            None => (0, 0),
        }
    }
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (a, _) = Self::f(&root);
        a
    }
}