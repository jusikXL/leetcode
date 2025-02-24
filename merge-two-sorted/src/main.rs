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

fn create_linked_list(range: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for value in range {
        let new_node = Box::new(ListNode::new(value));

        tail = &mut tail.insert(new_node).next;
    }

    head
}

////////////////

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged: Vec<i32> = Vec::new();

    merged.extend(&nums1);
    merged.extend(&nums2);
    merged.sort_unstable();

    let n = merged.len();
    if n % 2 == 0 {
        ((merged[n / 2 - 1] + merged[n / 2]) as f64) / 2.0
    } else {
        merged[n / 2] as f64
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(l), None) | (None, Some(l)) => Some(l),
        (Some(l1), Some(l2)) => if l1.val <= l2.val {
            Some(
                Box::new(ListNode {
                    val: l1.val,
                    next: merge_two_lists(l1.next, Some(l2)),
                })
            )
        } else {
            Some(
                Box::new(ListNode {
                    val: l2.val,
                    next: merge_two_lists(Some(l1), l2.next),
                })
            )
        }
    }
}

// pub fn merge_two_lists(
//     list1: Option<Box<ListNode>>,
//     list2: Option<Box<ListNode>>
// ) -> Option<Box<ListNode>> {
//     let mut x = &list1;
//     let mut y = &list2;

//     let mut head = None;
//     let mut tail = &mut head;

//     while let (Some(node1), Some(node2)) = (x, y) {
//         if node1.val < node2.val {
//             let new_node = Box::new(ListNode::new(node1.val));
//             tail = &mut tail.insert(new_node).next;

//             x = &node1.next;
//         } else {
//             let new_node = Box::new(ListNode::new(node2.val));
//             tail = &mut tail.insert(new_node).next;

//             y = &node2.next;
//         }
//     }

//     let mut remaining = if x.is_some() { x } else { y };

//     while let Some(node) = remaining {
//         let new_node = Box::new(ListNode::new(node.val));
//         tail = &mut tail.insert(new_node).next;

//         remaining = &node.next;
//     }

//     head
// }

fn main() {
    let list1 = create_linked_list(vec![1, 2, 4]);
    let list2 = create_linked_list(vec![1, 3, 4]);

    println!("{:?}", merge_two_lists(list1, list2));
}
