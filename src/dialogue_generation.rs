//! Module for Dialogue Generation
//! 
//! This module integrates with the Gemini API to generate dialogues.


use reqwest::Client;

///Represents the dialogue generation system of an NPC.
pub struct DialogueGeneration {
    client: Client,
}

impl DialogueGeneration {
    /// Creates a new DialogueGeneration instance.
    pub fn new() -> Self {
        let client = Client::new();
        DialogueGeneration { client }
    }
}

