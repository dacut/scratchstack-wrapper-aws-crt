use {
    std::ptr::null_mut,
};

#[repr(C)]
pub struct AwsCLinkedListNode {
    pub next: *mut AwsCLinkedListNode,
    pub prev: *mut AwsCLinkedListNode,
}

#[repr(C)]
pub struct AwsCLinkedList {
    pub head: AwsCLinkedListNode,
    pub tail: AwsCLinkedListNode,
}

impl AwsCLinkedListNode {
    /// Set node's next and prev pointers to NULL.
    pub fn reset(&mut self) {
        self.next = null_mut();
        self.prev = null_mut();
    }
}
