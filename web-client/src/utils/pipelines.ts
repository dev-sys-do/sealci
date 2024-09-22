import { Pipeline, PipelineStatus } from "../types";

export function getPipelineStatus(pipeline: Pipeline): PipelineStatus {
  const statuses = pipeline.actions.map((action) => action.status);
  if (statuses.includes("ACTION_STATUS_ERROR")) {
    return "ACTION_STATUS_ERROR";
  } else if (statuses.includes("ACTION_STATUS_RUNNING")) {
    return "ACTION_STATUS_RUNNING";
  } else if (statuses.includes("ACTION_STATUS_SCHEDULED")) {
    return "ACTION_STATUS_SCHEDULED";
  } else if (statuses.includes("ACTION_STATUS_PENDING")) {
    return "ACTION_STATUS_PENDING";
  } else {
    return "ACTION_STATUS_COMPLETED";
  }
}
