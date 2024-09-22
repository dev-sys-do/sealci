import { usePipelines } from "@/queries/pipelines.queries";
import { PipelineCard } from "../components/pipeline";
import { useEffect } from "react";

export default function PipelinesPage() {
  const { data: pipelines, refetch } = usePipelines(false);

  useEffect(() => {
    const interval = setInterval(() => {
      console.log("fetching pipelines");
      refetch();
    }, 5000);

    return () => clearInterval(interval);
  }, [refetch]);

  if (!pipelines) {
    return <div>fetching</div>;
  }
  return (
    <div>
      <h2 className="text-4xl text-primary my-6 font-serif">All pipelines</h2>
      <div className="grid-cols-2 grid gap-4">
        {pipelines
          .sort((a, b) => {
            return a.id > b.id ? -1 : 1;
          })
          .map((pipeline) => (
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
