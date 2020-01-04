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
    fn f(s: &mut Vec<Rc<RefCell<TreeNode>>>) {
        let mut r = Rc::clone(s.last().as_ref().unwrap());
        while r.borrow().left.is_some() {
            let r2 = Rc::clone(r.borrow().left.as_ref().unwrap());
            r = r2;
            s.push(Rc::clone(&r));
        }
    }

    pub fn get_all_elements(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut s1 = Vec::new();
        let mut s2 = Vec::new();
        let mut res = Vec::new();
        
        if let Some(r) = root1.as_ref() {
            s1.push(Rc::clone(r));
            Self::f(&mut s1);
        }
        if let Some(r) = root2.as_ref() {
            s2.push(Rc::clone(r));
            Self::f(&mut s2);
        }
        while !s1.is_empty() || !s2.is_empty() {
            if s2.is_empty() || (s1.last().is_some() && s1.last().unwrap().borrow().val <= s2.last().unwrap().borrow().val) {
                let n = s1.pop().unwrap();
                let b = n.borrow();
                res.push(b.val);
                if let Some(r) = b.right.as_ref() {
                    s1.push(Rc::clone(r));
                    Self::f(&mut s1);
                }
            } else {
                let n = s2.pop().unwrap();
                let b = n.borrow();
                res.push(b.val);
                if let Some(r) = b.right.as_ref() {
                    s2.push(Rc::clone(r));
                    Self::f(&mut s2);
                }
            }
        }
        res
    }
}