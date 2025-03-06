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
    pub fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut v = vec![0; 10];
        let n = root.unwrap();
        let r = n.borrow();
        Self::f(&r, &mut v)
    }
    
    fn f(r: &TreeNode, mut p: &mut Vec<i32>) -> i32 {
        p[r.val as usize] += 1;
        let mut c = 0;
        if r.left == None && r.right == None {
            if p.iter().filter(|i| *i % 2 == 1).count() <= 1 {
                c = 1;
            }
        } else {
            if let Some(n) = &r.left {
                c += Self::f(&n.borrow(), &mut p);
            }
            if let Some(n) = &r.right {
                c += Self::f(&n.borrow(), &mut p);
            }
        }
        p[r.val as usize] -= 1;
        c
    }
}