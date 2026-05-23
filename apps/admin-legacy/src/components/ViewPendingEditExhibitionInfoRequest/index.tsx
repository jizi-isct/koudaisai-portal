import {ApprovalRequestRead} from "@koudaisai/shared-types";
import {BasePlanRead} from "@koudaisai/shared-types";
import {getFilesRedirectUrl} from "@koudaisai/shared-utils";
import React from "react";

type Props = {
  approvalRequest: ApprovalRequestRead,
  plan: BasePlanRead
}

export function ViewPendingEditExhibitionInfoRequest({approvalRequest, plan}: Props) {
  return (
    <div>
      <h2>企画内容紹介文</h2>
      {
        approvalRequest.type_edit_exhibition_info.description ?
          <div>
            <h3>変更前</h3>
            {plan.description}
            <h3>変更後</h3>
            {approvalRequest.type_edit_exhibition_info.description}
          </div> :
          <div>変更なし</div>
      }

      <h2>企画アイコン</h2>
      {
        approvalRequest.type_edit_exhibition_info.icon_key ?
          <div>
            <h3>変更前</h3>
            <img
              src={`https://api2025.jizi.jp/cdn-cgi/image/width=128,height=128,format=webp,quality=auto/v1/plans/${plan.id}/icon`}
              alt={"現在の企画のアイコン"}
              width={128}
              height={128}
            />
            <h3>変更後</h3>
            <img
              src={getFilesRedirectUrl(approvalRequest.type_edit_exhibition_info.icon_key)}
              alt={"新しい企画アイコン"}
              width={128}
              height={128}
            />
          </div> :
          <div>変更なし</div>
      }
    </div>
  )
}
