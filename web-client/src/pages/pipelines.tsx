import { PipelineCard } from "../components/pipeline";

const pipelines = [
  {
    id: 5,
    repository_url: "https://github.com/dev-sys-do/sealci.git",
    name: "Killing the agent host",
    commit_hash: "a0b1c2",
    actions: [
      {
        id: 5,
        pipeline_id: 5,
        name: "killing_hugo",
        container_uri: "nixos:0.1.0",
        commands: ["je quitte trash", "shutdown now"],
        type: "Container",
        status: "ACTION_STATUS_PENDING",
      },
    ],
  },
  {
    id: 21,
    repository_url: "https://github.com/dev-sys-do/sealci",
    name: "First pipeline to pass OMG",
    commit_hash: "e32b1c2",
    actions: [
      {
        id: 21,
        pipeline_id: 21,
        name: "testing_hello_world",
        container_uri: "debian:latest",
        commands: ["cat License"],
        type: "Container",
        status: "ACTION_STATUS_PENDING",
      },
    ],
  },
];

export default function PipelinesPage() {
  return (
    <div>
      <h2 className="text-4xl text-primary my-6 font-serif">All pipelines</h2>
      <div className="grid-cols-2 grid gap-4">
        {pipelines.map((pipeline) => (
          <PipelineCard
            key={pipeline.id}
            pipeline={{
              id: pipeline.id,
              repository_url: pipeline.repository_url,
              name: pipeline.name,
              actions: pipeline.actions,
            }}
            commit_hash={pipeline.commit_hash}
          />
        ))}
      </div>
    </div>
  );
}
