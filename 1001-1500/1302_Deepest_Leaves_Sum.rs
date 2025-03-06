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
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let r = root.unwrap();
        let mut v = vec![r];
        loop {
            let mut v2 = Vec::new();
            for n in &v {
                if let Some(l) = n.borrow().left.clone() {
                    v2.push(l);
                }
                if let Some(r) = n.borrow().right.clone() {
                    v2.push(r);
                }
            }
            if v2.is_empty() {
                return v.iter().fold(0, |a, n| a + n.borrow().val);
            } else {
                v = v2;
            }
        }
    }
}