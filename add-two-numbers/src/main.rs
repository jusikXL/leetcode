struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn to_number(l: Option<Box<ListNode>>) -> i32 {
        let mut number = 0;
        let mut counter = 1;
        let mut current = &l;

        while let Some(node) = current {
            number += counter * node.val;
            counter *= 10;

            current = &node.next;
        }

        number
    }

    pub fn from_number(mut num: i32) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;

        if num == 0 {
            return Some(Box::new(ListNode::new(0)));
        }

        while num > 0 {
            let digit = num % 10;
            num /= 10;

            let new_node = Box::new(ListNode::new(digit));
            *current = Some(new_node);
            if let Some(ref mut node) = current {
                current = &mut node.next;
            }
        }

        head
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let mut sum;
        let mut carry = 0;

        let mut dummy_head = Box::new(ListNode::new(0));
        let mut current = &mut dummy_head;

        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());

        while l1.is_some() || l2.is_some() {
            sum = carry;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next.as_ref();
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next.as_ref();
            }

            carry = sum / 10;
            sum %= 10;

            let new_node = Box::new(ListNode::new(sum));
            current.next = Some(new_node.clone());
            current = current.next.as_mut().unwrap();
        }

        if carry != 0 {
            let new_node = Box::new(ListNode::new(carry));
            current.next = Some(new_node.clone());
            current = current.next.as_mut().unwrap();
        }

        dummy_head.next
    }
}

fn main() {
    println!("Hello, world!");
}
