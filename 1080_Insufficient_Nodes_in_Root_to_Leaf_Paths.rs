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
    fn f(r: Option<Rc<RefCell<TreeNode>>>, limit: i32) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        if let Some(_n) = r.clone() {
            let mut n = _n.borrow_mut();
            if n.left == None && n.right == None {
                return (n.val, if n.val < limit { None } else { r });
            }
            let mut v = Vec::new();
            if n.left != None {
                let (a, b) = Self::f(n.left.clone(), limit - n.val);
                v.push(a);
                n.left = b;
            }
            if n.right != None {
                let (a, b) = Self::f(n.right.clone(), limit - n.val);
                v.push(a);
                n.right = b;
            }
            let s = v.iter().max().unwrap() + n.val;
            (s, if s < limit { None } else { r })
        } else {
            (0, None)
        }
    }
    pub fn sufficient_subset(root: Option<Rc<RefCell<TreeNode>>>, limit: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let (a, b) = Self::f(root, limit);
        b
    }
}