fn main() {
    println!("Hello, world!");
    // dbg!("build_list(vec![2, 4, 3])", build_list(vec![2, 4, 3]));
    // dbg!(
    //     "add_two_numbers(build_list(vec![2, 4, 3]), build_list(vec![5, 6, 4]))",
    //     add_two_numbers(build_list(vec![2, 4, 3]), build_list(vec![5, 6, 4]))
    // );
    assert_eq!(
        add_two_numbers(build_list(vec![2, 4, 3]), build_list(vec![5, 6, 4])),
        build_list(vec![7, 0, 8])
    );
    assert_eq!(
        add_two_numbers(build_list(vec![0]), build_list(vec![0])),
        build_list(vec![0])
    );
    assert_eq!(
        add_two_numbers(
            build_list(vec![9, 9, 9, 9, 9, 9, 9]),
            build_list(vec![9, 9, 9, 9])
        ),
        build_list(vec![8, 9, 9, 9, 0, 0, 0, 1])
    );
}

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
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut d1 = l1;
    let mut d2 = l2;

    let mut carry = 0;

    let mut root = None;
    let mut ptr = &mut root;

    while d1.is_some() || d2.is_some() || carry != 0 {
        let val = d1.clone().unwrap_or(Box::new(ListNode::new(0))).val
            + d2.clone().unwrap_or(Box::new(ListNode::new(0))).val
            + carry;
        let node = ListNode::new(val % 10);
        *ptr = Some(Box::new(node));
        ptr = &mut ptr.as_mut().unwrap().next;

        carry = val / 10;

        d1 = d1.and_then(|n| n.next);
        d2 = d2.and_then(|n| n.next);
    }
    root
}

/// This one requires reversing the input list
fn _build_list(digits: Vec<i32>) -> Option<Box<ListNode>> {
    let mut root = None;
    for digit in digits.iter().rev() {
        let mut node = ListNode::new(*digit);
        node.next = root;
        root = Some(Box::new(node));
    }

    root
}

fn build_list(digits: Vec<i32>) -> Option<Box<ListNode>> {
    let mut root = None;
    let mut ptr = &mut root;

    for digit in digits.iter() {
        let node = ListNode::new(*digit);
        *ptr = Some(Box::new(node));
        ptr = &mut ptr.as_mut().unwrap().next;
    }

    root
}
