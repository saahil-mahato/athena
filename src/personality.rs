//! # Personality Module
//!
//! This module defines the Big Five personality traits of an NPC and provides methods to manage them.
//! The traits are:
//! - **Openness**: Reflects the degree of intellectual curiosity and creativity.
//! - **Conscientiousness**: Indicates how organized, dependable, and disciplined an individual is.
//! - **Extraversion**: Measures the extent to which a person is outgoing and sociable.
//! - **Agreeableness**: Assesses how cooperative, compassionate, and friendly a person is.
//! - **Neuroticism**: Evaluates emotional stability and the tendency to experience negative emotions.

use serde::{Deserialize, Serialize};

/// Represents the personality of an NPC.
#[derive(Debug, Serialize, Deserialize)]
pub struct Personality {
    pub openness: f64,
    pub conscientiousness: f64,
    pub extraversion: f64,
    pub agreeableness: f64,
    pub neuroticism: f64,

}

impl Personality {
    /// Creates a new Personality instance.
    pub fn new() -> Self {
        Personality {
            openness: 0.5,
            conscientiousness: 0.5,
            extraversion: 0.5,
            agreeableness: 0.5,
            neuroticism: 0.5,
        }
    }

    /// Sets the openness trait of the personality.
    ///
    /// Openness reflects the willingness to engage in new experiences and intellectual curiosity.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value for openness (between 0.0 and 1.0).
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::personality::Personality;
    ///
    /// let mut personality = Personality::new();
    /// personality.set_openness(0.8);
    /// ```
    pub fn set_openness(&mut self, value: f64) {
        self.openness = value.clamp(0.0, 1.0);
    }

    /// Sets the conscientiousness trait of the personality.
    ///
    /// Conscientiousness indicates how organized and dependable an individual is.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value for conscientiousness (between 0.0 and 1.0).
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::personality::Personality;
    ///
    /// let mut personality = Personality::new();
    /// personality.set_conscientiousness(0.7);
    /// ```
    pub fn set_conscientiousness(&mut self, value: f64) {
        self.conscientiousness = value.clamp(0.0, 1.0);
    }

    /// Sets the extraversion trait of the personality.
    ///
    /// Extraversion measures how outgoing and sociable a person is.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value for extraversion (between 0.0 and 1.0).
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::personality::Personality;
    ///
    /// let mut personality = Personality::new();
    /// personality.set_extraversion(0.9);
    /// ```
    pub fn set_extraversion(&mut self, value: f64) {
        self.extraversion = value.clamp(0.0, 1.0);
    }

    /// Sets the agreeableness trait of the personality.
    ///
    /// Agreeableness assesses how cooperative and compassionate a person is.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value for agreeableness (between 0.0 and 1.0).
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::personality::Personality;
    ///
    /// let mut personality = Personality::new();
    /// personality.set_agreeableness(0.6);
    /// ```
    pub fn set_agreeableness(&mut self, value: f64) {
        self.agreeableness = value.clamp(0.0, 1.0);
    }

    /// Sets the neuroticism trait of the personality.
    ///
    /// Neuroticism evaluates emotional stability and the tendency to experience negative emotions.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value for neuroticism (between 0.0 and 1.0).
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::personality::Personality;
    ///
    /// let mut personality = Personality::new();
    /// personality.set_neuroticism(0.4);
    /// ```
    pub fn set_neuroticism(&mut self, value: f64) {
        self.neuroticism = value.clamp(0.0, 1.0);
    }

}