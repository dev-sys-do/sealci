use proto;
use std::collections::HashMap;

/// A struct representing an action in the pool.
/// The action has an ID, a score, and additional fields from the ActionRequest proto.
#[derive(Debug)]
struct Action {
    action_id: proto::ActionRequest::action_id,
    context: proto::ActionRequest::context,
    commands: proto::ActionRequest::commands,
}

impl Action {
    /// Constructor
    pub(crate) fn new(action_id: u32, context: proto::ActionRequest::context, commands: Vec<String>) -> Self {
        Self {
            action_id,
            context,
            commands,
        }
    }

    /// Action ID getter
    pub(crate) fn get_action_id(&self) -> &u32 {
        &self.action_id
    }

    /// Context getter
    pub(crate) fn get_context(&self) -> &ExecutionContext {
        &self.context
    }

    /// Commands getter
    pub(crate) fn get_commands(&self) -> &[String] {
        &self.commands
    }


    /// Action ID setter
    pub(crate) fn set_action_id(&mut self, action_id: u32) {
        self.action_id = action_id;
    }

    /// Context setter
    pub(crate) fn set_context(&mut self, context: ExecutionContext) {
        self.context = context;
    }

    /// Commands setter
    pub(crate) fn set_commands(&mut self, commands: Vec<String>) {
        self.commands = commands;
    }
}

/// ActionsQueue is a collection of Actions stored in a vector.
/// The vector is sorted whenever necessary to maintain order.
pub(crate) struct ActionsQueue {
    actions: Vec<Action>,
}

impl ActionsQueue {
    /// Constructor
    pub(crate) fn new() -> Self {
        Self { actions: Vec::new() }
    }

    /// Insert an Action into the Action Pool and sort the Pool by score.
    pub(crate) fn push(&mut self, item: Action) {
        self.actions.push(item);
    }

    /// Remove and return the Action with the lowest score (that is, the first Action), or return None if the Pool is empty.
    pub(crate) fn pop(&mut self) -> Option<Action> {
        if self.actions.is_empty() {
            None
        } else {
            Some(self.actions.remove(0))
        }
    }

    /// Peek at the Action with the lowest score without removing it, or return None if the Pool is empty.
    pub(crate) fn peek(&self) -> Option<&Action> {
        if self.actions.is_empty() {
            None
        } else {
            self.actions.first() // The first element has the lowest score
        }
    }

    /// Return the number of Actions in the Pool
    pub(crate) fn len(&self) -> usize {
        self.actions.len()
    }

    /// Check if the Action Pool is empty
    pub(crate) fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    // Print the content of the action pool
    fn print_actions(&self) {
        for action in &self.actions {
            println!("{:?}", action);
        }
    }
}
























// Example usage
fn main() {
    let mut pq = ActionsQueue::new();

    // Get the number of elements in the queue
    let queue_len = pq.len();
    println!("Queue length: {}", queue_len);

    // Peek at the element with the highest priority without removing it
    if let Some(action) = pq.peek() {
        println!("Peeked action: {:?}", action);
    } else {
        println!("Queue is empty");
    }
    println!("Is the queue empty? {}", pq.is_empty());
    pq.push(Action { action_id: 1, context: ExecutionContext::new(HashMap::new()), commands: vec!["ls".to_string(), "pwd".to_string()] });
    println!("Is the queue empty? {}", pq.is_empty());
    pq.push(Action { action_id: 2, context: ExecutionContext::new(HashMap::new()), commands: vec!["echo 'Hello'".to_string()] });
    pq.push(Action { action_id: 3, context: ExecutionContext::new(HashMap::new()), commands: vec!["echo 'World'".to_string()] });
    pq.print_actions();
    
    let action = Action { action_id: 3, context: ExecutionContext::new(HashMap::new()), commands: vec!["echo 'Hello, World!'".to_string()] };
    pq.push(action);

    // Peek at the element with the lowest score without removing it
    if let Some(action) = pq.peek() {
        println!("Peeked action: {:?}", action);
    } else {
        println!("Queue is empty");
    }

    // Remove and print all elements from the queue
    while let Some(action) = pq.pop() {
        println!("{:?}", action);
    }

    // Peek at the element with the lowest score without removing it
    if let Some(action) = pq.peek() {
        println!("Peeked action: {:?}", action);
    } else {
        println!("Queue is empty");
    }

    // Get the number of elements in the queue
    let queue_len = pq.len();
    println!("Queue length: {}", queue_len);
}

