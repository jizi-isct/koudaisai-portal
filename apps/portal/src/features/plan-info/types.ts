import type { apiComponents } from '@koudaisai/shared-types';

/**
 * 企画情報API(events26)の企画。
 * 旧 plans_info API の BasePlanRead 相当だが、フィールドは camelCase の別スキーマ。
 */
export type Project = apiComponents['schemas']['Project'];
export type ProjectType = Project['type'];
/** 開催予定(日時と場所の組)。旧 schedule + location に相当する。 */
export type Occasion = apiComponents['schemas']['Occasion'];
export type FoodStallTag = apiComponents['schemas']['FoodStallTag'];
export type GeneralTag = apiComponents['schemas']['GeneralTag'];
