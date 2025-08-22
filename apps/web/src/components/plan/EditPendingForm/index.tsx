'use client'; // クライアントサイドコンポーネントとして実行するために追加

import React from "react";
import {$apiMembers, ApprovalRequestRead, getFilesRedirectUrl} from "@/lib";
import {ButtonCompact} from "@/components/generic/ButtonCompact";
import {Heading1, LoadingScreen} from "@/components/generic";
import Image from "next/image";
import {Loader} from "@/components/generic/Loader";
import {$plansInfoApiNoLogin} from "@/lib/plansInfoApi";


type Props = {
  refetch: () => Promise<void>;
  planId: string;
  initPlanName: string;
  initDescription: string;
  initIsChildFriendly: boolean;
  approvalRequest: ApprovalRequestRead
};

export function EditPendingForm({
                                  refetch,
                                  planId,
                                  initPlanName,
                                  initDescription,
                                  initIsChildFriendly,
                                  approvalRequest
                                }: Props) {
  const {data: basePlan} = $plansInfoApiNoLogin.useQuery("get", "/plans/{planId}", {
    params: {
      path: {
        planId: planId
      }
    }
  })
  const {mutateAsync: closeApprovalRequest} = $apiMembers.useMutation("post", "/users/{user_id}/approval_requests/{request_id}/close")
  const [isClosing, setIsClosing] = React.useState<boolean>(false);

  if (!basePlan) {
    return <LoadingScreen/>
  }

  const handleClose = async () => {
    setIsClosing(true);
    await closeApprovalRequest({
      params: {
        path: {
          user_id: "me",
          request_id: approvalRequest.id
        }
      }
    })
    setIsClosing(false);
    await refetch();
  }

  return (<div>
    <Heading1 emoji={"📝"}>企画情報訂正申請は現在審査中です</Heading1>

    <h3>企画名</h3>
    {
      approvalRequest.type_edit_exhibition_info?.plan_name ?
        <p>{initPlanName} → {approvalRequest.type_edit_exhibition_info.plan_name}</p> :
        <p>{initPlanName} → 変更なし</p>
    }

    <h3>企画概要</h3>
    {
      approvalRequest.type_edit_exhibition_info?.description ?
        <div>
          <p>{initDescription}</p>
          <p>↓</p>
          <p>{approvalRequest.type_edit_exhibition_info.description}</p>
        </div> :
        <div>
          <p>{initDescription}</p>
          <p>↓</p>
          <p>変更なし</p>
        </div>
    }

    <h3>子供向け企画か否か</h3>
    {
      approvalRequest.type_edit_exhibition_info?.is_child_friendly !== undefined ?
        <p>{initIsChildFriendly} → {approvalRequest.type_edit_exhibition_info.is_child_friendly}</p> :
        <p>{initIsChildFriendly} → 変更なし</p>
    }

    <h3>アイコン画像</h3>
    {
      approvalRequest.type_edit_exhibition_info?.icon_key ?
        <p>
          <Image
            src={`https://api2025.jizi.jp/cdn-cgi/image/width=128,height=128,format=webp,quality=auto/v1/plans/${planId}/icon`}
            alt={"現在の企画のアイコン"}
            width={128}
            height={128}
          />
          →
          <Image
            src={getFilesRedirectUrl(approvalRequest.type_edit_exhibition_info.icon_key)}
            alt={"新しい企画アイコン"}
            width={128}
            height={128}
          />
        </p> :
        <p>
          <Image
            src={`https://api2025.jizi.jp/cdn-cgi/image/width=128,height=128,format=webp,quality=auto/v1/plans/${planId}/icon`}
            alt={"現在の企画のアイコン"}
            width={128}
            height={128}
          />
          → 変更なし
        </p>
    }
    {
      isClosing ? <Loader/> :
        <ButtonCompact text={"企画情報訂正申請を取り下げる"} onClick={handleClose}/>
    }
  </div>)
}