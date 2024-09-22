import { Link } from "react-router-dom";
import { MdiGithub } from "../icons/github";
import { Pipeline } from "../types";
import { getPipelineStatus } from "../utils/pipelines";

export function PipelineCard({
  pipeline,
  commit_hash,
}: {
  pipeline: Pipeline;
  commit_hash: string;
}) {
  const status = getPipelineStatus(pipeline);
  const { id, name } = pipeline;
  const color =
    status === "ACTION_STATUS_ERROR"
      ? "bg-error"
      : status === "ACTION_STATUS_COMPLETED"
        ? "bg-success"
        : "bg-warning";
  return (
    <Link
      to={`/${id}`}
      className="border-[0.3px] px-4 py-2 rounded-md border-border bg-secondaryDark grid-rows-1"
    >
      <span className="flex flex-row justify-between">
        <span className="flex flex-row justify-start gap-3 items-center">
          <h3 className="w-fit text-primary text-2xl truncate">{name}</h3>
          <p className="text-primaryDark text-2xl font-light">#{id}</p>
        </span>
        <span className="border-border border-[0.3px] rounded-full flex flex-row gap-2 items-center px-4">
          <div className={`w-2 h-2 rounded-full ${color}`} />
          <p className="text-primary text-sm font-light">{commit_hash}</p>
        </span>
      </span>
      <span className="bg-secondary rounded-full py-2 px-4 flex-row flex items-center gap-2 w-fit mt-3">
        <MdiGithub color="white" width={25} height={25} />
        <a
          href={pipeline.repository_url}
          className="text-primaryDark hover:text-accent"
        >
          {pipeline.repository_url}
        </a>
      </span>
    </Link>
  );
}
