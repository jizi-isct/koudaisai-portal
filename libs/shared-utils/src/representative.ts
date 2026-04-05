/**
 * ユーザーIDから代表者のインデックスを取得
 */
import {GroupRead, UserRead} from "@koudaisai/shared-types";

export function getRepresentativeIndex(user: UserRead, group: GroupRead): string | undefined {
  if (group.type_press) {
    return undefined
  }
  if (group.type_plan?.type_booth) {
    if (group.type_plan.type_booth.representative1 === user.id) {
      return "一";
    } else if (group.type_plan.type_booth.representative2 === user.id) {
      return "二";
    } else if (group.type_plan.type_booth.representative3 === user.id) {
      return "三";
    }
  }
  if (group.type_plan?.type_general) {
    if (group.type_plan.type_general.representative1 === user.id) {
      return "一";
    } else if (group.type_plan.type_general.representative2 === user.id) {
      return "二";
    } else if (group.type_plan.type_general.representative3 === user.id) {
      return "三";
    }
  }
  if (group.type_plan?.type_stage) {
    if (group.type_plan.type_stage.representative1 === user.id) {
      return "一";
    } else if (group.type_plan.type_stage.representative2 === user.id) {
      return "二";
    } else if (group.type_plan.type_stage.representative3 === user.id) {
      return "三";
    }
  }
  if (group.type_plan?.type_labo) {
    return undefined
  }
  return undefined
}

