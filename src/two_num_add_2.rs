/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
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
        for v in vec.into_iter(){
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

impl Iterator for ListNode {
    type Item = Box<ListNode>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take()
    }
}
pub struct IntoIter (ListNode);

impl Iterator for IntoIter{
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.next.is_none() {
            return None;
        }
        self.0=*self.0.next.take().unwrap();
        Some(self.0.val)
    }
}

impl ListNode {
    fn append(&mut self, val: i32) -> &mut Option<Box<ListNode>> {
        self.next = Some(Box::new(ListNode::new(val)));
        &mut self.next
    }
    fn add(&mut self, val: i32) {
        self.val = self.val + val;
    }
    fn adding_done(&mut self) -> &mut Option<Box<ListNode>> {
        if self.val > 9 {
            self.val = self.val % 10;
            return self.append(1);
        }
        &mut self.next
    }
}

impl Solution for ListNode {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let n1 = match l1 {
            Some(n) => n,
            None => panic!("l1 is None"),
        };
        let n2 = match l2 {
            Some(n) => n,
            None => panic!("l2 is None"),
        };

        let mut a0 = ListNode::new(0);
        a0.add(n1.val);
        a0.add(n2.val);
        println!("a0:{:?}", &a0);
        let mut p = a0.adding_done();

        let mut n1_iter = IntoIter(*n1);
        let mut n2_iter = IntoIter(*n2);

        while true {
            let next_a1 = n1_iter.next();
            let next_a2 = n2_iter.next();
            println!("next_a1:{:?}", &next_a1);
            println!("next_a2:{:?}", &next_a2);
            if next_a1.is_none() && next_a2.is_none() {
                break;
            }
            if !next_a2.is_none() && !next_a1.is_none() {
                if p.is_none() {
                    *p = Some(Box::new(ListNode::new(0)));
                }
                let v1 = next_a1.unwrap();
                let v2 = next_a2.unwrap();
                p.as_mut().unwrap().add(v1 + v2);
                println!("p:{:?}", &p);
                p = p.as_mut().unwrap().adding_done();
                println!("p:{:?}", &p);

                continue;
            }
            if next_a1.is_none() || next_a2.is_none() {
                if p.is_none() {
                    *p = Some(Box::new(ListNode::new(0)));
                }
                let val = match next_a1 {
                    Some(n) => n,
                    None => next_a2.unwrap(),
                };
                p.as_mut().unwrap().add(val);
                p=p.as_mut().unwrap().adding_done();
            }
        }
        println!("a0:{:?}", &a0);
        Some(Box::new(a0))
    }
}
// @lc code=end

pub trait Solution {
    fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>>;
}

#[test]
fn test_list_add() {
    let a=vec![7,4,3];
    let b=vec![8,6,5];

    let l1 = ListNode::from_vec(a);
    let l2 = ListNode::from_vec(b);
    println!("{:?}", l1.as_ref().unwrap().to_vec());
    println!("{:?}", l2.as_ref().unwrap().to_vec());
    let ll12 = ListNode::add_two_numbers(l1,l2);
    println!("{:?}", ll12.unwrap().to_vec());
}
