'use client'; // クライアントサイドコンポーネントとして実行するために追加

import React from "react";
import {$apiMembers, ApprovalRequestRead} from "@/lib";
import {ButtonCompact} from "@/components/generic/ButtonCompact";
import {Heading1, LoadingScreen} from "@/components/generic";
import {Loader} from "@/components/generic/Loader";
import {ViewPendingEditExhibitionInfoRequest} from "@/components/plan/ViewPendingEditExhibitionInfoRequest";
import {$plansInfoApiNoLogin} from "@/lib/plansInfoApi";


type Props = {
  refetch: () => Promise<void>;
  planId: string;
  approvalRequest: ApprovalRequestRead
};

export function EditPendingForm({
                                  refetch,
                                  planId,
                                  approvalRequest
                                }: Props) {
  const {mutateAsync: closeApprovalRequest} = $apiMembers.useMutation("post", "/users/{user_id}/approval_requests/{request_id}/close")
  const [isClosing, setIsClosing] = React.useState<boolean>(false);

  const {data: plan} = $plansInfoApiNoLogin.useQuery("get", "/plans/{planId}", {
    params: {
      path: {
        planId: planId
      }
    }
  })

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

  if (!plan) {
    return <LoadingScreen/>
  }

  return (<div>
    <Heading1 emoji={"📝"}>企画情報訂正申請は現在審査中です</Heading1>

    <ViewPendingEditExhibitionInfoRequest approvalRequest={approvalRequest} plan={plan}/>
    {
      isClosing ? <Loader/> :
        <ButtonCompact text={"企画情報訂正申請を取り下げる"} onClick={handleClose}/>
    }
  </div>)
}