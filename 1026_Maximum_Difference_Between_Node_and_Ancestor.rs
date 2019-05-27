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
use std::cmp::max;
use std::cmp::min;
impl Solution {
    fn f(r: & Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if let Some(x) = r {
            let n = x.borrow();
            let mut d = 0;
            let mut mi = n.val;
            let mut ma = n.val;
            if n.left != None {
                let (a, al, ar) = Self::f(&n.left);
                d = max(d, a);
                d = max(d, max((n.val - al).abs(), (n.val - ar).abs()));
                mi = min(mi, al);
                ma = max(ma, ar);
            }
            if n.right != None {
                let (a, al, ar) = Self::f(&n.right);
                d = max(d, a);
                d = max(d, max((n.val - al).abs(), (n.val - ar).abs()));
                mi = min(mi, al);
                ma = max(ma, ar);
            }
            (d, mi, ma)
        } else {
            (0, 0, 0)
        }
    }
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (a, _, _) = Self::f(&root);
        a
    }
}