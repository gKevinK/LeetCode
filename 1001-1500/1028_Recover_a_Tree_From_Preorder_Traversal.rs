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
    fn readi(vs: & Vec<char>, i: &mut usize) -> i32 {
        let mut v: i32 = 0;
        while *i < vs.len() && vs[*i].is_digit(10) {
            v = v * 10 + (vs[*i].to_digit(10).unwrap()) as i32;
            *i += 1;
        }
        v
    }
    pub fn recover_from_preorder(s: String) -> Option<Rc<RefCell<TreeNode>>> {
        let vs: Vec<char> = s.chars().collect();
        let mut i: usize = 0;
        let r: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(Self::readi(& vs, &mut i))));
        let mut s: Vec<Rc<RefCell<TreeNode>>> = vec![r.clone()];
        while i < vs.len() {
            let mut deep = i;
            while i < vs.len() && vs[i] == '-' {
                i += 1;
            }
            deep = i - deep;
            let n = Rc::new(RefCell::new(TreeNode::new(Self::readi(& vs, &mut i))));
            while s.len() > deep {
                s.pop();
            }
            {
                let mut l = s.last().unwrap().borrow_mut();
                if l.left == None {
                    l.left = Some(n.clone());
                } else {
                    l.right = Some(n.clone());
                }
            }
            s.push(n);
        }
        Some(r)
    }
}