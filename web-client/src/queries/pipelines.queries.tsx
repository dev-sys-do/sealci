import { Pipeline } from "@/types";
import { useQuery } from "@tanstack/react-query";
import ky from "ky";

const fetchPipelines = async ({
  verbose,
}: {
  verbose?: boolean;
} = {}): Promise<Pipeline[]> => {
  const endpoint = verbose
    ? "/pipeline?verbose=true"
    : "/pipeline?verbose=false";
  const json = await ky
    .get(import.meta.env.VITE_CONTROLLER_ENDPOINT + endpoint)
    .json<Pipeline[]>();

  return json;
};

const fetchPipeline = async ({
  verbose,
  id,
}: {
  verbose: boolean;
  id: string;
}): Promise<Pipeline> => {
  const endpoint = verbose
    ? `/pipeline/${id}?verbose=true`
    : `/pipeline/${id}?verbose=false`;
  const json = await ky
    .get(import.meta.env.VITE_CONTROLLER_ENDPOINT + endpoint)
    .json<Pipeline>();

  return json;
};

export const usePipelines = (verbose: boolean) => {
  const { data, error, isPending, refetch } = useQuery({
    queryKey: ["pipelines"],
    queryFn: () => fetchPipelines({ verbose }),
  });
  return { data, error, isPending, refetch };
};

export const usePipeline = (verbose: boolean, id: string) => {
  const { data, error, isPending, refetch } = useQuery({
    queryKey: ["pipeline", id],
    queryFn: () => fetchPipeline({ verbose, id }),
  });
  return { data, error, isPending, refetch };
};
