// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::<i32>::new();
        let mut s = Vec::<i32>::new();
        let mut h = &head;
        while let Some(n) = h {
            v.push(n.val);
            h = &n.next;
        }
        for i in (0..v.len()).rev() {
            let t = v[i];
            while *s.last().unwrap_or(&2_000_000_000) <= t {
                s.pop();
            }
            v[i] = *s.last().unwrap_or(&0);
            s.push(t);
        }
        v
    }
}