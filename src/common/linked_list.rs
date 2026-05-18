#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

///
/// [`&Option<Box<ListNode>>`]链表转[Vec]
///
pub fn linked_list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec = vec![];
    let mut curr = head;
    while let Some(node) = curr {
        vec.push(node.val);
        curr = &node.next;
    }
    vec
}

///
/// [`Vec<i32>`]转[`Option<Box<ListNode>>`]链表
///
pub fn vec_to_linked_list(vec: &Vec<i32>, dummy_head: bool) -> Option<Box<ListNode>> {
    if vec.len() == 0 {
        return None;
    }
    let mut dummy = Box::new(ListNode::new(-1));
    let mut curr = &mut dummy;
    for i in 0..vec.len() {
        let value = vec[i];
        curr.next = Some(Box::new(ListNode::new(value)));
        curr = curr.next.as_mut()?;
    }
    if dummy_head { Some(dummy) } else { dummy.next }
}
