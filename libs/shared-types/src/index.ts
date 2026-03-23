export type * from './types';
export {
  type paths as apiPaths,
  type webhooks as apiWebhooks,
  type components as apiComponents,
  type $defs as $apiDefs,
  type operations as apiOperations,
} from "./api_v2";
export {
  type paths as authPaths
} from "./auth_v1";
export {
  type paths as plansInfoPaths,
  type components as plansInfoComponents,
} from "./plans_info_api_v1";

export type BasePlanCreate = import("./plans_info_api_v1").components["schemas"]["BasePlanCreate"];
export type BasePlanRead = import("./plans_info_api_v1").components["schemas"]["BasePlanRead"];
export type BasePlanUpdate = import("./plans_info_api_v1").components["schemas"]["BasePlanUpdate"];
export type BoothPlanCreate = import("./plans_info_api_v1").components["schemas"]["BoothPlanCreate"];
export type BoothPlanRead = import("./plans_info_api_v1").components["schemas"]["BoothPlanRead"];
export type BoothPlanUpdate = import("./plans_info_api_v1").components["schemas"]["BoothPlanUpdate"];
export type GeneralPlanCreate = import("./plans_info_api_v1").components["schemas"]["GeneralPlanCreate"];
export type GeneralPlanRead = import("./plans_info_api_v1").components["schemas"]["GeneralPlanRead"];
export type GeneralPlanUpdate = import("./plans_info_api_v1").components["schemas"]["GeneralPlanUpdate"];
export type StagePlanCreate = import("./plans_info_api_v1").components["schemas"]["StagePlanCreate"];
export type StagePlanRead = import("./plans_info_api_v1").components["schemas"]["StagePlanRead"];
export type StagePlanUpdate = import("./plans_info_api_v1").components["schemas"]["StagePlanUpdate"];
export type LaboPlanCreate = import("./plans_info_api_v1").components["schemas"]["LaboPlanCreate"];
export type LaboPlanRead = import("./plans_info_api_v1").components["schemas"]["LaboPlanRead"];
export type LaboPlanUpdate = import("./plans_info_api_v1").components["schemas"]["LaboPlanUpdate"];