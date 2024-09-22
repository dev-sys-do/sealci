import { usePipelines } from "@/queries/pipelines.queries";
import { PipelineCard } from "../components/pipeline";

export default function PipelinesPage() {
  const { data: pipelines } = usePipelines(false);

  if (!pipelines) {
    return <div>fetching</div>;
  }
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
            commit_hash={"eqwc231"}
          />
        ))}
      </div>
    </div>
  );
}
