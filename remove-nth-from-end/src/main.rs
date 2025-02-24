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

fn print_linked_list(head: &Option<Box<ListNode>>) {
    let mut curr = head;

    while let Some(node) = curr {
        println!("{}", node.val);
        curr = &node.next;
    }
}

fn create_linked_list(range: std::ops::Range<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for value in range {
        let new_node = Box::new(ListNode::new(value));

        tail = &mut tail.insert(new_node).next;
    }

    head
}

////////////

fn main() {
    let linked_list = create_linked_list(1..6);

    let linked_list = remove_nth_from_end(linked_list, 2);

    print_linked_list(&linked_list);
}

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut fast = &head;
    let mut slow = &mut head;

    // Move `fast` pointer n steps ahead
    for _ in 0..n {
        if let Some(ref next) = fast {
            fast = &next.next;
        }
    }

    // If `fast` reached the end, remove the first node
    if fast.is_none() {
        return head.unwrap().next;
    }

    // Move both `slow` and `fast` until `fast` reaches the last node
    while let Some(ref next) = fast {
        fast = &next.next;
        slow = &mut slow.as_mut().unwrap().next;
    }

    // Skip the nth node from the end
    slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.take().unwrap().next;

    head
}
