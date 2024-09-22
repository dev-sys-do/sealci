use crate::action::action_repository::Action;
use serde::{Deserialize, Serialize};

pub mod pipeline_controller;
pub mod pipeline_repository;
pub mod pipeline_service;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: i64,
    pub repository_url: String,
    pub name: String,
    pub actions: Vec<Action>,
}

impl Pipeline {
    pub fn new(id: i64, repository_url: String, name: String, actions: Vec<Action>) -> Self {
        return Pipeline {
            id,
            repository_url,
            name,
            actions,
        };
    }
}
