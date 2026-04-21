import { useMemo } from "react";
import {getApiFetchClient} from "@koudaisai/shared-api"

export function useApiFetchClientWithNoAuth(baseUrl: string) {
  return useMemo(() => getApiFetchClient(baseUrl), [baseUrl])
}