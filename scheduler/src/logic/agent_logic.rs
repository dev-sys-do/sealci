use std::cmp::Ordering;

/// Top secret algorithm used to mathematically compute the freeness score of an Agent. Do not leak!
pub(crate) fn compute_score(cpu_usage: u32, memory_usage: u32) -> u32 {
    (0.5 * cpu_usage as f32 + 0.5 * memory_usage as f32) as u32
}

/// A struct representing an agent in the pool.
/// The agent has an ID and a score.
#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Agent {
    id: u32,
    score: u32,
}

/// Implement `Ord` to order/compare by `score`.
impl Ord for Agent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

/// Implement `PartialOrd` to order/compare by `score`.
impl PartialOrd for Agent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// AgentPool is a collection of Agents stored in a vector.
/// The vector is sorted whenever necessary to maintain order.
pub(crate) struct AgentPool {
    agents: Vec<Agent>,
}

impl AgentPool {
    pub(crate) fn new() -> Self {
        Self {
            agents: Vec::new(),
        }
    }

    /// Insert an Agent into the Agent Pool and sort the Pool by score.
    pub(crate) fn push(&mut self, item: Agent) {
        self.agents.push(item);
        self.sort();  // Keep the vector sorted after each insertion of a new Agent
    }

    /// Remove and return the Agent with the lowest score (that is, the first Agent), or return None if the Pool is empty.
    pub(crate) fn pop(&mut self) -> Option<Agent> {
        if self.agents.is_empty() {
            None
        } else {
            Some(self.agents.remove(0))
        }
    }
    

    /// Peek at the Agent with the lowest score without removing it, or return None if the Pool is empty.
    pub(crate) fn peek(&self) -> Option<&Agent> {
        if self.agents.is_empty() {
            None
        } else {
            self.agents.first()  // The first element has the lowest score
        }
    }

    /// Return the number of Agents in the Pool
    pub(crate) fn len(&self) -> usize {
        self.agents.len()
    }

    /// Check if the Agent Pool is empty
    pub(crate) fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    /// Sort the Agents by score (ascending)
    /// Uses Rust's built-in sorting algorithm to sort the Agents by score. It is a Timsort.
    pub(crate) fn sort(&mut self) {
        self.agents.sort_by_key(|agent| agent.score);
    }

    /// Return a reference to the Agent of the given ID, or None if the Agent is not found.
    pub(crate) fn find_agent(&self, id: u32) -> Option<&Agent> {
        if let Some(index) = self.agents.iter().position(|agent| agent.id == id) {
            Some(&self.agents[index])
        } else {
            None
        }
        
    }

    /// Check if the Agent with the given ID is out of order compared to its neighbors
    pub(crate) fn check_agent_neighbors(&self, id: u32) -> Option<bool> {
        let index = self.agents.iter().position(|agent| agent.id == id)?;
        if index > 0 && self.agents[index].score < self.agents[index - 1].score {
            return Some(true);  // Agent is out of order (lower score than previous)
        }
        if index < self.agents.len() - 1 && self.agents[index].score > self.agents[index + 1].score {
            return Some(true);  // Agent is out of order (higher score than next)
        }
        return Some(false);  // Agent is in correct order
    }

    /// Generate a unique ID by finding the maximum existing ID and incrementing it by 1. This ensures that the new ID is *always* unique among the Agent Pool.
    pub(crate) fn generate_unique_id(&self) -> u32 {
        self.agents.iter().map(|agent| agent.id).max().unwrap_or(0) + 1  // unwrap_or(0) is used to handle the case when the Agent Pool is empty
    }

    // Print the content of the agent pool
    fn print_agents(&self) {
        for agent in &self.agents {
            println!("{:?}", agent);
        }
    }
}

// Example usage
fn main() {
    let mut pq = AgentPool::new();

    // Get the number of elements in the queue
    let queue_len = pq.len();
    println!("Queue length: {}", queue_len);

    // Peek at the element with the highest priority without removing it
    if let Some(agent) = pq.peek() {
        println!("Peeked agent: {:?}", agent);
    } else {
        println!("Queue is empty");
    }
    println!("Is the queue empty? {}", pq.is_empty());
    pq.push(Agent { id: 2, score: 2 });
    pq.push(Agent { id: 1, score: 5 });
    println!("Is the queue empty? {}", pq.is_empty());
    pq.push(Agent { id: 3, score: 8 });
    pq.push(Agent { id: 4, score: 3 });
    pq.print_agents();
    // Check if the agent with ID "c" is out of order compared to its neighbors
    if let Some(is_out_of_order) = pq.check_agent_neighbors(3) {
        println!("Agent '3' out of order: {}", is_out_of_order);
    } else {
        println!("Agent not found");
    }
    
    // Generate a unique ID and push an Agent into the queue
    let unique_id = pq.generate_unique_id();
    let agent = Agent { id: unique_id, score: compute_score(10, 22) };
    pq.push(agent);

    // Sort the queue by score (already happens automatically after every push)
    pq.sort();
    pq.print_agents();

    // Find an agent by ID
    if let Some(agent) = pq.find_agent(3) {
        println!("Found agent: {:?}", agent);
    } else {
        println!("Agent not found");
    }

    // Find an agent by ID
    if let Some(agent) = pq.find_agent(5) {
        println!("Found agent: {:?}", agent);
    } else {
        println!("Agent not found");
    }

    // Check if the agent with ID "c" is out of order compared to its neighbors
    if let Some(is_out_of_order) = pq.check_agent_neighbors(3) {
        println!("Agent '3' out of order: {}", is_out_of_order);
    } else {
        println!("Agent not found");
    }

    // Peek at the element with the lowest score without removing it
    if let Some(agent) = pq.peek() {
        println!("Peeked agent: {:?}", agent);
    } else {
        println!("Queue is empty");
    }

    // Remove and print all elements from the queue
    while let Some(agent) = pq.pop() {
        println!("{:?}", agent);
    }

    // Peek at the element with the lowest score without removing it
    if let Some(agent) = pq.peek() {
        println!("Peeked agent: {:?}", agent);
    } else {
        println!("Queue is empty");
    }

    // Get the number of elements in the queue
    let queue_len = pq.len();
    println!("Queue length: {}", queue_len);
}
