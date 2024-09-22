export interface Pipeline {
  id: number;
  repository_url: string;
  name: string;
  actions: Action[];
}

export interface Action {
  id: number;
  pipeline_id: number;
  name: string;
  container_uri: string;
  commands: string[];
  type: string;
  status: string;
}

export type PipelineStatus =
  | "ACTION_STATUS_PENDING"
  | "ACTION_STATUS_SCHEDULED"
  | "ACTION_STATUS_RUNNING"
  | "ACTION_STATUS_COMPLETED"
  | "ACTION_STATUS_ERROR";
