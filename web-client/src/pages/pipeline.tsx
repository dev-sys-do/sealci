import { useParams } from "react-router-dom";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { a11yDark, CodeBlock } from "react-code-blocks";
import { usePipeline } from "@/queries/pipelines.queries";

export default function PipelinePage() {
  const { id } = useParams();
  const { data: pipeline } = usePipeline(true, `${id}`);

  if (!pipeline) {
    return <div>fetching</div>;
  }
  return (
    <div>
      <span className="flex flex-row m-0 p-0 items-end text-4xl font-thin text-primaryDark gap-3">
        <h2 className="text-4xl text-primary font-serif my-0 p-0">Pipeline</h2>
        <p>#{id}</p>
      </span>
      <h3 className="text-2xl text-primary my-6 font-serif">Actions</h3>
      <Accordion type="single" collapsible>
        {pipeline.actions.map((action, index) => (
          <AccordionItem value={`item-${index + 1}`} className="text-primary ">
            <AccordionTrigger>
              <span className="flex flex-row items-center gap-2">
                <span className="flex flex-row items-end">
                  {index + 1}.{" "}
                  <p className="font-mono text-2xl">{action.name}</p>
                </span>
                {action.status === "ACTION_STATUS_COMPLETED" ? (
                  <span className="bg-success w-3 h-3 rounded-full">✔</span>
                ) : action.status === "ACTION_STATUS_ERROR" ? (
                  <span className="bg-error w-3 h-3 rounded-full">✖</span>
                ) : (
                  <span className="bg-warning w-3 h-3 rounded-full"></span>
                )}
              </span>
            </AccordionTrigger>
            <AccordionContent>
              <CodeBlock
                customStyle={{
                  fontFamily: "Darker Grotesque, monospace",
                  fontSize: "20px",
                  fontWeight: "500",
                }}
                text={action.logs ? action.logs.join("\n") : ""}
                language={"bash"}
                showLineNumbers={true}
                theme={a11yDark}
              />
            </AccordionContent>
          </AccordionItem>
        ))}
      </Accordion>
    </div>
  );
}
