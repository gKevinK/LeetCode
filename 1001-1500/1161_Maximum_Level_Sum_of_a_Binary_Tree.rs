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
    fn f(r1: Option<Rc<RefCell<TreeNode>>>, mut v: &mut Vec<i32>, d: i32) {
        match r1 {
            Some(r) => {
                if (v.len() as i32) < d {
                    v.push(0);
                }
                v[d as usize - 1] += r.borrow().val;
                Self::f(r.borrow().left.clone(), &mut v, d + 1);
                Self::f(r.borrow().right.clone(), &mut v, d + 1);
            },
            None => {},
        }
    }
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut v: Vec<i32> = Vec::new();
        Self::f(root, &mut v, 1);
        let mut d = 0;
        let mut m = 0;
        for i in 0..v.len() {
            if v[i] > m {
                d = i;
                m = v[i];
            }
        }
        d as i32 + 1
    }
}