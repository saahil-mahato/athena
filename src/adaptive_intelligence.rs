//! # Adaptive Intelligence Module
//!
//! This module implements adaptive intelligence for NPCs, allowing them to make decisions based on
//! their current state, context, and experiences. It utilizes a flexible framework that can be
//! customized to fit the needs of different games.

use std::collections::HashMap;

/// Represents a generic state for an NPC. The actual states will be defined externally.
pub type State = String;

/// Represents an action that the NPC can take.
#[derive(Debug, Clone)]
pub struct Action {
    pub name: String,
    pub description: String,
}

/// Represents the adaptive intelligence of an NPC.
pub struct AdaptiveIntelligence {
    /// The current state of the NPC.
    current_state: State,
    /// A memory store for experiences or interactions.
    memory: HashMap<String, String>,
    /// A list of available actions for the NPC.
    actions: Vec<Action>,
}

impl AdaptiveIntelligence {
    /// Creates a new AdaptiveIntelligence instance with the provided actions.
    ///
    /// # Arguments
    ///
    /// * `actions` - A vector of actions that the NPC can take.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::adaptive_intelligence::AdaptiveIntelligence;
    /// use athena::adaptive_intelligence::Action;
    ///
    /// let actions = vec![
    ///     Action { name: "Talk".to_string(), description: "Engage in conversation.".to_string() },
    ///     Action { name: "Run".to_string(), description: "Flee from danger.".to_string() },
    /// ];
    /// let mut ai = AdaptiveIntelligence::new(actions);
    /// ```
    pub fn new(actions: Vec<Action>) -> Self {
        AdaptiveIntelligence {
            current_state: "Idle".to_string(), // Default state
            memory: HashMap::new(),
            actions,
        }
    }

    /// Updates the NPC's state based on the provided context.
    ///
    /// # Arguments
    ///
    /// * `new_state` - A string representing the new state of the NPC.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::adaptive_intelligence::AdaptiveIntelligence;
    /// use athena::adaptive_intelligence::Action;
    ///
    /// let actions = vec![
    ///     Action { name: "Talk".to_string(), description: "Engage in conversation.".to_string() },
    ///     Action { name: "Run".to_string(), description: "Flee from danger.".to_string() },
    /// ];
    /// let mut ai = AdaptiveIntelligence::new(actions);
    /// ai.update_state("Alert");
    /// ```
    pub fn update_state(&mut self, new_state: &str) {
        self.current_state = new_state.to_string();
    }

    /// Chooses an action based on the current state of the NPC and psychological principles.
    ///
    /// The decision-making process can be influenced by the NPC's memory and current emotional state.
    ///
    /// # Returns
    ///
    /// A string describing the chosen action.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::adaptive_intelligence::AdaptiveIntelligence;
    /// use athena::adaptive_intelligence::Action;
    ///
    /// let actions = vec![
    ///     Action { name: "Talk".to_string(), description: "Engage in conversation.".to_string() },
    ///     Action { name: "Run".to_string(), description: "Flee from danger.".to_string() },
    /// ];
    /// let mut ai = AdaptiveIntelligence::new(actions);
    /// ai.update_state("Engaged");
    /// let action = ai.choose_action();
    /// println!("{}", action);
    /// ```
    pub fn choose_action(&self) -> String {
        // Decision-making based on the current state
        match self.current_state.as_str() {
            "Idle" => self.select_action("Rest"),
            "Alert" => self.select_action("Investigate"),
            "Engaged" => self.select_action("Talk"),
            "Fleeing" => self.select_action("Run"),
            _ => "No action available".to_string(),
        }
    }

    /// Selects an action based on the action name.
    ///
    /// # Arguments
    ///
    /// * `action_name` - The name of the action to select.
    ///
    /// # Returns
    ///
    /// A string describing the selected action or a message if the action is not found.
    fn select_action(&self, action_name: &str) -> String {
        for action in &self.actions {
            if action.name == action_name {
                return format!("Action selected: {}", action.description);
            }
        }
        "Action not found".to_string()
    }

    /// Records a memory of an event or interaction.
    ///
    /// # Arguments
    ///
    /// * `key` - A string representing the key for the memory.
    /// * `value` - A string representing the value of the memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::adaptive_intelligence::AdaptiveIntelligence;
    /// use athena::adaptive_intelligence::Action;
    ///
    /// let actions = vec![
    ///     Action { name: "Talk".to_string(), description: "Engage in conversation.".to_string() },
    ///     Action { name: "Run".to_string(), description: "Flee from danger.".to_string() },
    /// ];
    /// let mut ai = AdaptiveIntelligence::new(actions);
    /// ai.record_memory("last_interaction", "spoke to player");
    /// ```
    pub fn record_memory(&mut self, key: &str, value: &str) {
        self.memory.insert(key.to_string(), value.to_string());
    }

    /// Retrieves a memory based on the key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string representing the key for the memory.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the memory value, or `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::adaptive_intelligence::AdaptiveIntelligence;
    /// use athena::adaptive_intelligence::Action;
    ///
    /// let actions = vec![
    ///     Action { name: "Talk".to_string(), description: "Engage in conversation.".to_string() },
    ///     Action { name: "Run".to_string(), description: "Flee from danger.".to_string() },
    /// ];
    /// let mut ai = AdaptiveIntelligence::new(actions);
    /// ai.record_memory("last_interaction", "spoke to player");
    /// if let Some(memory) = ai.get_memory("last_interaction") {
    ///     println!("Memory: {}", memory);
    /// }
    /// ```
    pub fn get_memory(&self, key: &str) -> Option<&String> {
        self.memory.get(key)
    }

    /// Gets the current state of the NPC.
    ///
    /// # Returns
    ///
    /// The current state as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::adaptive_intelligence::AdaptiveIntelligence;
    /// use athena::adaptive_intelligence::Action;
    ///
    /// let actions = vec![
    ///     Action { name: "Talk".to_string(), description: "Engage in conversation.".to_string() },
    ///     Action { name: "Run".to_string(), description: "Flee from danger.".to_string() },
    /// ];
    /// let mut ai = AdaptiveIntelligence::new(actions);
    /// ai.update_state("Alert");
    /// let state = ai.get_current_state();
    /// println!("Current State: {}", state);
    /// ```
    pub fn get_current_state(&self) -> &String {
        &self.current_state
    }
}