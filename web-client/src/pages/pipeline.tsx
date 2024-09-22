import { useParams } from "react-router-dom";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { a11yDark, CodeBlock } from "react-code-blocks";

export default function PipelinePage() {
  const { id } = useParams();
  return (
    <div>
      <span className="flex flex-row m-0 p-0 items-end text-4xl font-thin text-primaryDark gap-3">
        <h2 className="text-4xl text-primary font-serif my-0 p-0">Pipeline</h2>
        <p>#{id}</p>
      </span>
      <h3 className="text-2xl text-primary my-6 font-serif">Actions</h3>
      <Accordion type="single" collapsible>
        <AccordionItem value="item-1" className="text-primary ">
          <AccordionTrigger>
            <span className="flex flex-row items-end gap-2">
              1. <p className="font-mono text-2xl">Is it accessible?</p>
            </span>
          </AccordionTrigger>
          <AccordionContent>
            <CodeBlock
              customStyle={{
                fontFamily: "Darker Grotesque, monospace",
                fontSize: "20px",
                fontWeight: "500",
              }}
              text={`yarn run accessibility-check`}
              language={"bash"}
              showLineNumbers={true}
              theme={a11yDark}
            />
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  );
}
