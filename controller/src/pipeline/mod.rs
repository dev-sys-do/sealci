use crate::action::action_repository::Action;

pub mod pipeline_controller;
pub mod pipeline_repository;
pub mod pipeline_service;

pub struct Pipeline {
    pub id: i64,
    pub repository_url: String,
    pub actions: Vec<Action>,
}
