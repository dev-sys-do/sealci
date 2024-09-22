import { useParams } from "react-router-dom";

export default function PipelinePage() {
  const { id } = useParams();
  return (
    <div>
      <span className="flex flex-row m-0 p-0 items-end text-4xl font-thin text-primaryDark gap-3">
        <h2 className="text-4xl text-primary font-serif my-0 p-0">Pipeline</h2>
        <p>#{id}</p>
      </span>
      <h3 className="text-2xl text-primary my-6 font-serif">Actions</h3>
    </div>
  );
}
