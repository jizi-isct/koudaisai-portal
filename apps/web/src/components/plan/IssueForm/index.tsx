'use client'; // クライアントサイドコンポーネントとして実行するために追加

import React from "react";
import {$apiMembers} from "@/lib";
import {TextInput} from "@/components/generic/TextInput/TextInput";
import {FileUploader} from "@/components/common/FileUploader";
import {ButtonCompact} from "@/components/generic/ButtonCompact";
import {Heading1} from "@/components/generic";


type EditModalProps = {
  refetch: () => Promise<void>;
  initDescription: string;
};

export const EditIssueForm = (
  {
    refetch,
    initDescription,
  }: EditModalProps) => {
  const [description, setDescription] = React.useState<string>(initDescription);
  const [iconKey, setIconKey] = React.useState<string | undefined>();
  const [issueReason, setIssueReason] = React.useState<string>("")
  const {mutateAsync: createApprovalRequest} = $apiMembers.useMutation("post", "/users/{user_id}/approval_requests")

  const handleSubmit = async () => {
    let reqDescription: string | undefined = description;

    if (description === initDescription) {
      reqDescription = undefined;
    }

    // 申請
    await createApprovalRequest(
      {
        params: {
          path: {
            user_id: "me"
          }
        },
        body: {
          type_edit_exhibition_info: {
            description: reqDescription,
            icon_key: iconKey,
          },
          issue_reason: issueReason
        }
      }
    )

    await refetch()
  }

  return (
    <div>
      <Heading1 emoji={"📝"}>企画情報の訂正</Heading1>
      <label>
        企画概要
        <TextInput
          value={description}
          setValue={setDescription}
          paragraph={true}
        />
      </label>
      <label>
        アイコン画像
        <FileUploader fileType={"image/*"} callback={(key, _) => setIconKey(key)} client={$apiMembers}/>
      </label>
      <label>
        訂正理由
        <TextInput
          value={issueReason}
          setValue={setIssueReason}
          paragraph={true}
        />
      </label>
      <ButtonCompact text={"企画情報の訂正を申請する"} onClick={handleSubmit}/>
    </div>
  );
};