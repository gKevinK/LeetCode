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
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sum = 0;
        let mut m = std::collections::HashMap::new();
        m.insert(0, 1);
        let mut tc = &head;
        while let Some(ref n) = tc {
            sum += n.val;
            *m.entry(sum).or_insert(0) += 1;
            tc = &n.next;
        }
        let mut h = Some(Box::new(ListNode { val: 0, next: head }));
        sum = 0;
        let mut t = &mut h;
        while let Some(ref mut n) = t {
            sum += n.val;
            *m.entry(sum).or_insert(0) -= 1;
            let s = sum;
            while *m.get(&s).unwrap() >= 1 {
                let n2 = n.next.take().unwrap();
                sum += n2.val;
                *m.entry(sum).or_insert(0) -= 1;
                n.next = n2.next;
            }
            t = &mut n.next;
        }
        h.unwrap().next
    }
}