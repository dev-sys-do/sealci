use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{collections::HashMap, iter::Map, str::FromStr, sync::Arc};

use crate::{action::action_repository::Action, parser::pipe_parser::Type};

use super::Pipeline;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineDTO {
    pub id: i64,
    pub name: String,
    pub repository_url: String,
}

pub struct PipelineDetailDTO {
    pub pipeline_id: i64,
    pub pipeline_name: String,
    pub repository_url: String,
    pub action_id: i64,
    pub action_name: String,
    pub action_container_uri: String,
    pub action_status: String,
    pub action_type: String,
    pub command: String,
}

struct ActionDetail {
    repository_url: String,
    pipeline_id: i64,
    pipeline_name: String,
    action: Action,
}

pub struct PipelineRepository {
    pool: Arc<PgPool>,
}

impl PipelineRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Pipeline>, sqlx::Error> {
        let rows = sqlx::query_as!(
            PipelineDetailDTO,
            r#"SELECT pipelines.id as pipeline_id,
               pipelines.name as pipeline_name,
               pipelines.repository_url as repository_url,
               a.id as action_id,
               a.name as action_name,
               a.container_uri as action_container_uri,
               a.status as action_status,
               a.type as action_type,
               c.command as command
        FROM pipelines
                 JOIN actions a on pipelines.id = a.pipeline_id
                 JOIN commands c on c.action_id = a.id;"#
        )
        .fetch_all(&*self.pool)
        .await?;

        let mut pipelines: Vec<Pipeline> = Vec::new();
        let mut actions: HashMap<i64, ActionDetail> = HashMap::new();
        let mut pipelines_map: HashMap<i64, Pipeline> = HashMap::new();
        for row in rows {
            let command = row.command;
            let action_id = row.action_id;
            if actions.contains_key(&action_id) {
                let action = actions.get_mut(&action_id).unwrap();
                action.action.commands.push(command);
            } else {
                let action = Action::new(
                    row.action_id,
                    row.pipeline_id,
                    row.action_name,
                    row.action_container_uri,
                    vec![command],
                    Type::from_str(row.action_type.as_str()).unwrap(),
                    row.action_status,
                )
                .unwrap();
                actions.insert(
                    action_id,
                    ActionDetail {
                        repository_url: row.repository_url.clone(),
                        pipeline_id: row.pipeline_id,
                        pipeline_name: row.pipeline_name.clone(),
                        action,
                    },
                );
            }
        }

        for (_, action_detail) in actions.iter() {
            let pipeline_id = action_detail.pipeline_id;
            if pipelines_map.contains_key(&pipeline_id) {
                let pipeline = pipelines_map.get_mut(&pipeline_id).unwrap();
                pipeline.actions.push(action_detail.action.clone());
            } else {
                let pipeline = Pipeline::new(
                    pipeline_id,
                    action_detail.repository_url.clone(),
                    action_detail.pipeline_name.clone(),
                    vec![action_detail.action.clone()],
                );
                pipelines_map.insert(pipeline_id, pipeline);
            }
        }

        for (_, pipeline) in pipelines_map.iter() {
            pipelines.push(pipeline.clone());
        }

        Ok(pipelines)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Pipeline, sqlx::Error> {
        let rows = sqlx::query_as!(
            PipelineDetailDTO,
            r#"SELECT pipelines.id as pipeline_id,
               pipelines.name as pipeline_name,
               pipelines.repository_url as repository_url,
               a.id as action_id,
               a.name as action_name,
               a.container_uri as action_container_uri,
               a.status as action_status,
               a.type as action_type,
               c.command as command
        FROM pipelines
                 JOIN actions a on pipelines.id = a.pipeline_id
                 JOIN commands c on c.action_id = a.id
        WHERE pipelines.id = $1;"#,
            id
        )
        .fetch_all(&*self.pool)
        .await?;

        let mut actions_map: HashMap<i64, ActionDetail> = HashMap::new();

        let mut pipeline_id: i64 = 0;
        let mut pipeline_name: String = String::new();
        let mut repository_url: String = String::new();

        for row in rows {
            pipeline_id = row.pipeline_id;
            pipeline_name = row.pipeline_name.clone();
            repository_url = row.repository_url.clone();

            let command = row.command;
            let action_id = row.action_id;
            if actions_map.contains_key(&action_id) {
                let action = actions_map.get_mut(&action_id).unwrap();
                action.action.commands.push(command);
            } else {
                let action = Action::new(
                    row.action_id,
                    row.pipeline_id,
                    row.action_name,
                    row.action_container_uri,
                    vec![command],
                    Type::from_str(row.action_type.as_str()).unwrap(),
                    row.action_status,
                )
                .unwrap();
                actions_map.insert(
                    action_id,
                    ActionDetail {
                        repository_url: row.repository_url.clone(),
                        pipeline_id: row.pipeline_id,
                        pipeline_name: row.pipeline_name.clone(),
                        action,
                    },
                );
            }
        }

        let mut actions = Vec::new();
        for (_, action) in actions_map.iter() {
            actions.push(action.action.clone());
        }

        if pipeline_id == 0 {
            return Err(sqlx::Error::RowNotFound); // no result found since there can not be a pipeline with id 0
        }

        Ok(Pipeline::new(
            pipeline_id,
            repository_url,
            pipeline_name,
            actions,
        ))
    }

    pub async fn create(
        &self,
        repository_url: &String,
        name: &String,
    ) -> Result<PipelineDTO, sqlx::Error> {
        let row = sqlx::query_as!(
            PipelineDTO,
            r#"INSERT INTO pipelines (repository_url, name) VALUES ($1, $2) RETURNING *"#,
            repository_url,
            name
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(row)
    }
}
