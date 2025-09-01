'use client'; // クライアントサイドコンポーネントとして実行するために追加

import React from "react";
import {$apiMembers} from "@/lib";
import { Flex, Typography, Button, Input } from "antd";
import {FileUploader} from "@/components/common/FileUploader";
import {Heading1} from "@/components/generic";

const { TextArea } = Input;

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
      <Flex vertical gap={20}>
        <Flex vertical gap={4}>
          <Typography.Title level={5}>企画概要</Typography.Title>
          <TextArea defaultValue={description} style={{ resize: 'none'}} rows={2} onChange={(e) => setDescription(e.target.value)} />
        </Flex>
        <Flex vertical gap={8}>
          <Typography.Title level={5}>アイコン画像</Typography.Title>
          <FileUploader fileType={"image/*"} callback={(key, _) => setIconKey(key)} client={$apiMembers}/>
        </Flex>
        <Flex vertical gap={8}>
          <Typography.Title level={5}>訂正理由</Typography.Title>
          <TextArea placeholder="理由を入力してください" style={{ resize: 'none'}} onChange={(e) => setIssueReason(e.target.value)} />
        </Flex>
        <Button type="primary" style={{ alignSelf: "flex-start" }} onClick={handleSubmit}>企画情報の訂正を申請する</Button>
      </Flex>
    </div>
  );
};