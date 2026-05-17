import {getPlansInfoFetchClient} from "@koudaisai/shared-api";
import {PLANS_INFO_API_URL} from "astro:env/client";

export const plansInfoApi = getPlansInfoFetchClient(PLANS_INFO_API_URL)