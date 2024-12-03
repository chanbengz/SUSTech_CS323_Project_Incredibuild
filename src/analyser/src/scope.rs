use crate::symbol::Symbol;

#[derive(Debug)]
struct ScopeStack<T> {
    head: Option<Box<Symbol<T>>>, // Pointer to the top of the stack
}

impl<T> ScopeStack<T> {
    // Create a new empty stack
    fn new() -> Self {
        Stack { head: None }
    }

    // Push an element onto the stack
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            data: value,
            next: self.head.take(), // Current head becomes the next node
        });
        self.head = Some(new_node); // New node becomes the new head
    }

    // Pop an element from the stack
    fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next; // Move head to the next node
            Some(node.data) // Return the popped value
        } else {
            None // Stack is empty
        }
    }

    // Peek at the top element without removing it
    fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.data)
    }

    // Check if the stack is empty
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}
