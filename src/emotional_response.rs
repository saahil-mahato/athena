//! # Emotional Response Module
//!
//! This module manages the emotional responses of NPCs, allowing them to experience and react
//! to various emotional states. It provides methods to set, get, and modify emotional states,
//! as well as to choose actions based on these emotions.

use std::collections::HashMap;

/// Represents the emotional states an NPC can experience.
#[derive(Debug, Clone, PartialEq)]
pub enum Emotion {
    Joy,
    Trust,
    Fear,
    Surprise,
    Sadness,
    Disgust,
    Anger,
    Anticipation,
    Neutral,
}

/// Represents the emotional response system of an NPC.
pub struct EmotionalResponse {
    /// The current emotional state of the NPC.
    current_emotion: Emotion,
    /// A memory store for past emotional states and their triggers.
    memory: HashMap<String, Emotion>,
}

impl EmotionalResponse {
    /// Creates a new EmotionalResponse instance with a default emotional state.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::emotional_response::EmotionalResponse;
    /// let emotional_response = EmotionalResponse::new();
    /// ```
    pub fn new() -> Self {
        EmotionalResponse {
            current_emotion: Emotion::Neutral, // Default emotional state
            memory: HashMap::new(),
        }
    }

    /// Sets the current emotional state of the NPC.
    ///
    /// # Arguments
    ///
    /// * `emotion` - The new emotional state to set for the NPC.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::emotional_response::{EmotionalResponse, Emotion};
    /// let mut emotional_response = EmotionalResponse::new();
    /// emotional_response.set_emotion(Emotion::Joy);
    /// ```
    pub fn set_emotion(&mut self, emotion: Emotion) {
        self.current_emotion = emotion;
    }

    /// Gets the current emotional state of the NPC.
    ///
    /// # Returns
    ///
    /// The current emotional state as an `Emotion`.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::emotional_response::{EmotionalResponse, Emotion};
    /// let mut emotional_response = EmotionalResponse::new();
    /// emotional_response.set_emotion(Emotion::Anger);
    /// let current_emotion = emotional_response.get_emotion();
    /// println!("Current Emotion: {:?}", current_emotion);
    /// ```
    pub fn get_emotion(&self) -> &Emotion {
        &self.current_emotion
    }

    /// Records an emotional memory with a trigger.
    ///
    /// # Arguments
    ///
    /// * `trigger` - A string representing the event or situation that triggered the emotion.
    /// * `emotion` - The emotional state associated with the trigger.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::emotional_response::{EmotionalResponse, Emotion};
    /// let mut emotional_response = EmotionalResponse::new();
    /// emotional_response.record_memory("lost_game", Emotion::Sadness);
    /// ```
    pub fn record_memory(&mut self, trigger: &str, emotion: Emotion) {
        self.memory.insert(trigger.to_string(), emotion);
    }

    /// Retrieves an emotional memory based on the trigger.
    ///
    /// # Arguments
    ///
    /// * `trigger` - A string representing the event or situation that triggered the emotion.
    ///
    /// # Returns
    ///
    /// An `Option<Emotion>` containing the emotional state associated with the trigger, or `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::emotional_response::{EmotionalResponse, Emotion};
    /// let mut emotional_response = EmotionalResponse::new();
    /// emotional_response.record_memory("lost_game", Emotion::Sadness);
    /// if let Some(emotion) = emotional_response.get_memory("lost_game") {
    ///     println!("Emotion for 'lost_game': {:?}", emotion);
    /// }
    /// ```
    pub fn get_memory(&self, trigger: &str) -> Option<&Emotion> {
        self.memory.get(trigger)
    }

    /// Chooses an action based on the current emotional state of the NPC.
    ///
    /// # Returns
    ///
    /// A string describing the action chosen based on the current emotion.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::emotional_response::{EmotionalResponse, Emotion};
    /// let mut emotional_response = EmotionalResponse::new();
    /// emotional_response.set_emotion(Emotion::Fear);
    /// let action = emotional_response.choose_action();
    /// println!("Chosen Action: {}", action);
    /// ```
    pub fn choose_action(&self) -> String {
        match self.current_emotion {
            Emotion::Joy => "Dance".to_string(),
            Emotion::Trust => "Collaborate".to_string(),
            Emotion::Fear => "Hide".to_string(),
            Emotion::Surprise => "Investigate".to_string(),
            Emotion::Sadness => "Cry".to_string(),
            Emotion::Disgust => "Reject".to_string(),
            Emotion::Anger => "Shout".to_string(),
            Emotion::Anticipation => "Prepare".to_string(),
            Emotion::Neutral => "Observe".to_string(),
        }
    }
}