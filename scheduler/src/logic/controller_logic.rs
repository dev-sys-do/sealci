//use crate::proto::controller as proto;
use crate::proto::scheduler as proto;

/// A struct representing an action in the queue.
/// The action has an ID, a score, and additional fields from the ActionRequest proto.
#[derive(Debug)]
pub(crate) struct Action {
    action_id: u32,
    context: proto::ExecutionContext,
    commands: Vec<String>,
}

impl Action {
    /// Constructor
    pub fn new(action_id: u32, context: proto::ExecutionContext, commands: Vec<String>) -> Self {
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
    pub(crate) fn get_context(&self) -> &proto::ExecutionContext {
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
    pub(crate) fn set_context(&mut self, context: proto::ExecutionContext) {
        self.context = context;
    }

    /// Commands setter
    pub(crate) fn set_commands(&mut self, commands: Vec<String>) {
        self.commands = commands;
    }

}

/// ActionsQueue is a collection of Actions stored in a vector.
/// The vector is sorted whenever necessary to maintain order.
pub struct ActionsQueue {
    actions: Vec<Action>,
}

impl ActionsQueue {
    /// Constructor
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }

    /// Insert an Action into the Action Queue and sort the Queue by score.
    pub fn push(&mut self, item: Action) {
        self.actions.push(item);
    }

    /// Remove and return the Action with the lowest score (that is, the first Action), or return None if the Queue is empty.
    pub fn pop(&mut self) -> Option<Action> {
        if self.actions.is_empty() {
            None
        } else {
            Some(self.actions.remove(0))
        }
    }

    /// Return the number of Actions in the Queue
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    /// Check if the Action Queue is empty
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
    
}
