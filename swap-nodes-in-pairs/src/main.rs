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

///////////////////////

fn create_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for num in nums {
        let new_node = Box::new(ListNode::new(num));

        tail = &mut tail.insert(new_node).next;
    }

    head
}

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    None
}

fn main() {
    let head = create_linked_list(vec![1, 2, 3, 4]);

    println!("{:?}", swap_pairs(head));
}
