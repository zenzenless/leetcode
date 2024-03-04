/*
 * @lc app=leetcode.cn id=232 lang=rust
 * @lcpr version=30117
 *
 * [232] 用栈实现队列
 */


// @lcpr-template-start

// @lcpr-template-end
// @lc code=start
struct MyQueue {
    queue:[i32;100],
    tail:usize,
    head:usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        Self { 
            queue: [0;100],
            head:0,
            tail:0
        }
    }
    
    fn push(&mut self, x: i32) {
        self.queue[self.tail]=x;
        self.tail+=1;
    }
    
    fn pop(&mut self) -> i32 {
        let t=self.queue[self.head];
        self.head+=1;
        t
    }
    
    fn peek(&self) -> i32 {
        self.queue[self.head]
    }
    
    fn empty(&self) -> bool {
        self.head==self.tail
    }
}


// @lc code=end



/*
// @lcpr case=start
// ["MyQueue", "push", "push", "peek", "pop", "empty"][[], [1], [2], [], [], []]\n
// @lcpr case=end

 */

