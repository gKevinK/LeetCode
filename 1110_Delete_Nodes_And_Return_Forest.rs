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
use std::collections::HashSet;
impl Solution {
    fn f(r: Option<Rc<RefCell<TreeNode>>>, set: &HashSet<i32>, mut res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(v) = &r {
            let mut n = v.borrow_mut();
            if set.contains(&n.val) {
                let left = Self::f(n.left.clone(), &set, &mut res);
                if left != None {
                    res.push(left);
                }
                let right = Self::f(n.right.clone(), &set, &mut res);
                if right != None {
                    res.push(right);
                }
                return None;
            } else {
                n.left = Self::f(n.left.clone(), &set, &mut res);
                n.right = Self::f(n.right.clone(), &set, &mut res);
            }
        }
        return r;
    }

    pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut res = Vec::new();
        let set: HashSet<i32> = to_delete.into_iter().collect();
        let r = Self::f(root, &set, &mut res);
        if r != None {
            res.push(r);
        }
        res
    }
}