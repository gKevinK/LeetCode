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
    fn f(r: Rc<RefCell<TreeNode>>, carry: i32) -> i32 {
        let mut c = carry;
        let mut n = r.borrow_mut();
        if let Some(right) = n.right.clone() {
            c = Self::f(right, c);
        }
        n.val += c;
        c = n.val;
        if let Some(left) = n.left.clone() {
            c = Self::f(left, c);
        }
        c
    }
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::f(root.clone().unwrap(), 0);
        root
    }
}