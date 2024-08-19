//! # Knowledge Graph Module
//!
//! This module implements a knowledge graph for NPCs, allowing them to store and manage knowledge
//! about entities, relationships, and properties. The knowledge graph enables NPCs to make informed
//! decisions based on the information available.

use std::collections::HashMap;

/// Represents an entity in the knowledge graph.
#[derive(Debug, Clone)]
pub struct Entity {
    pub id: String,
    pub properties: HashMap<String, String>,
}

impl Entity {
    /// Creates a new entity with the given ID and properties.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier for the entity.
    /// * `properties` - A HashMap of properties associated with the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use athena::knowledge_graph::{Entity};
    /// let mut properties = HashMap::new();
    /// properties.insert("name".to_string(), "Alice".to_string());
    /// let entity = Entity::new("1".to_string(), properties);
    /// ```
    pub fn new(id: String, properties: HashMap<String, String>) -> Self {
        Entity { id, properties }
    }
}

/// Represents a relationship between two entities in the knowledge graph.
#[derive(Debug, Clone)]
pub struct Relationship {
    pub source: String,
    pub target: String,
    pub relation_type: String,
    pub properties: HashMap<String, String>,
}

impl Relationship {
    /// Creates a new relationship between two entities.
    ///
    /// # Arguments
    ///
    /// * `source` - The ID of the source entity.
    /// * `target` - The ID of the target entity.
    /// * `relation_type` - The type of relationship (e.g., "friend", "colleague").
    /// * `properties` - A HashMap of properties associated with the relationship.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use athena::knowledge_graph::{Relationship};
    /// let mut properties = HashMap::new();
    /// properties.insert("since".to_string(), "2021".to_string());
    /// let relationship = Relationship::new("1".to_string(), "2".to_string(), "friend".to_string(), properties);
    /// ```
    pub fn new(source: String, target: String, relation_type: String, properties: HashMap<String, String>) -> Self {
        Relationship {
            source,
            target,
            relation_type,
            properties,
        }
    }
}

/// Represents the knowledge graph for NPCs.
pub struct KnowledgeGraph {
    /// A collection of entities in the knowledge graph.
    entities: HashMap<String, Entity>,
    /// A collection of relationships in the knowledge graph.
    relationships: Vec<Relationship>,
}

impl KnowledgeGraph {
    /// Creates a new empty knowledge graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use athena::knowledge_graph::KnowledgeGraph;
    /// let knowledge_graph = KnowledgeGraph::new();
    /// ```
    pub fn new() -> Self {
        KnowledgeGraph {
            entities: HashMap::new(),
            relationships: Vec::new(),
        }
    }

    /// Adds a new entity to the knowledge graph.
    ///
    /// # Arguments
    ///
    /// * `entity` - The entity to add to the knowledge graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use athena::knowledge_graph::{KnowledgeGraph, Entity};
    /// let mut knowledge_graph = KnowledgeGraph::new();
    /// let mut properties = HashMap::new();
    /// properties.insert("name".to_string(), "Alice".to_string());
    /// let entity = Entity::new("1".to_string(), properties);
    /// knowledge_graph.add_entity(entity);
    /// ```
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id.clone(), entity);
    }

    /// Adds a new relationship to the knowledge graph.
    ///
    /// # Arguments
    ///
    /// * `relationship` - The relationship to add to the knowledge graph.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use athena::knowledge_graph::{KnowledgeGraph, Relationship};
    /// let mut knowledge_graph = KnowledgeGraph::new();
    /// let mut properties = HashMap::new();
    /// properties.insert("since".to_string(), "2021".to_string());
    /// let relationship = Relationship::new("1".to_string(), "2".to_string(), "friend".to_string(), properties);
    /// knowledge_graph.add_relationship(relationship);
    /// ```
    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.push(relationship);
    }

    /// Retrieves an entity by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the entity to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option<&Entity>` containing a reference to the entity if found, or `None` if not found.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use athena::knowledge_graph::{KnowledgeGraph, Entity};
    /// let mut knowledge_graph = KnowledgeGraph::new();
    /// let mut properties = HashMap::new();
    /// properties.insert("name".to_string(), "Alice".to_string());
    /// let entity = Entity::new("1".to_string(), properties);
    /// knowledge_graph.add_entity(entity);
    /// let retrieved_entity = knowledge_graph.get_entity("1");
    /// assert!(retrieved_entity.is_some());
    /// ```
    pub fn get_entity(&self, id: &str) -> Option<&Entity> {
        self.entities.get(id)
    }

    /// Retrieves all relationships for a given entity ID.
    ///
    /// # Arguments
    ///
    /// * `entity_id` - The ID of the entity for which to retrieve relationships.
    ///
    /// # Returns
    ///
    /// A vector of `Relationship` objects associated with the entity.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use athena::knowledge_graph::{KnowledgeGraph, Relationship};
    /// let mut knowledge_graph = KnowledgeGraph::new();
    /// let mut properties = HashMap::new();
    /// properties.insert("since".to_string(), "2021".to_string());
    /// let relationship = Relationship::new("1".to_string(), "2".to_string(), "friend".to_string(), properties);
    /// knowledge_graph.add_relationship(relationship);
    /// let relationships = knowledge_graph.get_relationships("1");
    /// assert_eq!(relationships.len(), 1);
    /// ```
    pub fn get_relationships(&self, entity_id: &str) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.source == entity_id || r.target == entity_id)
            .collect()
    }
}