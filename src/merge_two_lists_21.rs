/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有&序链表
 */

// @lc code=start
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut cur = &mut head;
        for v in vec.into_iter() {
            let node = ListNode::new(v);
            cur.replace(Box::new(node));
            cur = &mut cur.as_mut().unwrap().next;
        }
        head
    }
    fn to_vec(&self) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut cur = self;
        loop {
            vec.push(cur.val);
            if cur.next.is_none() {
                break;
            }
            cur = cur.next.as_ref().unwrap();
        }
        vec
    }
}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut cur1 = match list1 {
            Some(l) => l,
            None => return list2,
        };
        let mut cur2 = match list2 {
            Some(l) => l,
            None => return Some(cur1),
        };
        let mut head:Option<Box<ListNode>> = None;
        let mut tail = &mut head;

        loop {
            if cur1.val < cur2.val{
                tail.replace(cur1);
                let tail_next=tail.as_mut().unwrap().next.take();
                if tail_next.is_none() {
                    //接上cur2
                    tail.as_mut().unwrap().next = Some(cur2);
                    break;
                }
                cur1=tail_next.unwrap();
                tail=&mut tail.as_mut().unwrap().next;
            }else{
                tail.replace(cur2);
                let tail_next=tail.as_mut().unwrap().next.take();
                if tail_next.is_none() {
                    //接上cur1
                    tail.as_mut().unwrap().next = Some(cur1);
                    break;
                }
                cur2=tail_next.unwrap();
                tail=&mut tail.as_mut().unwrap().next;
            }
        }
        return head
    }
}
// @lc code=end

struct Solution;

#[test]
fn test_merge_two_lits() {
    let l1 = ListNode::from_vec(vec![1, 2, 4]);
    let l2 = ListNode::from_vec(vec![1, 3, 4]);
    let l3 = Solution::merge_two_lists(l1, l2);
    let answer = vec![1,1, 2, 3, 4, 4];
    let l3_vec = l3.as_ref().unwrap().to_vec();
    println!("{:?}", l3_vec);
    assert_eq!(l3_vec, answer);
}
