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
    fn sum(r: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = r {
            let b = n.borrow();
            b.val + Self::sum(&b.left) + Self::sum(&b.right)
        } else {
            0
        }
    }
    fn f(r: &TreeNode, s: i32) -> (i64, i32) {
        use std::cmp::max;

        let t1 = if let Some(n) = &r.left {
            Self::f(&n.borrow(), s)
        } else { (0, 0) };
        let t2 = if let Some(n) = &r.right {
            Self::f(&n.borrow(), s)
        } else { (0, 0) };
        let sum = r.val + t1.1 + t2.1;
        (max(max(t1.0, t2.0), sum as i64 * (s - sum) as i64), sum)
    }
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let s = Self::sum(&root);
        let r = root.unwrap();
        let (res, _) = Self::f(&*r.borrow(), s);
        (res % 1_000_000_007) as i32
    }
}