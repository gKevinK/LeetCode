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
    fn count(r: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match r {
            Some(r2) => Self::count(r2.borrow().left.clone()) + Self::count(r2.borrow().right.clone()) + 1,
            None => 0,
        }
    }
    fn find(r: Option<Rc<RefCell<TreeNode>>>, x: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match r {
            Some(r2) => {
                if r2.borrow().val == x {
                    return Some(r2);
                }
                let a = Self::find(r2.borrow().left.clone(), x);
                if a != None {
                    return a;
                }
                let b = Self::find(r2.borrow().right.clone(), x);
                if b != None {
                    return b;
                }
                None
            },
            None => None,
        }
    }
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        let red = Self::find(root.clone(), x).unwrap();
        let a = Self::count(red.borrow().left.clone());
        let b = Self::count(red.borrow().right.clone());
        (a * 2 > n) || (b * 2 > n) || (a * 2 + b * 2 + 2 < n)
    }
}