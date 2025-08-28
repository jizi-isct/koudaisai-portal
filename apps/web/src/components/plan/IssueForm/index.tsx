'use client'; // クライアントサイドコンポーネントとして実行するために追加

import React from "react";
import {$apiMembers} from "@/lib";
import { Flex, Input, Typography, Button, Upload } from "antd";
import { UploadOutlined } from '@ant-design/icons';
import type { UploadProps } from 'antd';
import {TextInput} from "@/components/generic/TextInput/TextInput";
import {FileUploader} from "@/components/common/FileUploader";
import {ButtonCompact} from "@/components/generic/ButtonCompact";
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

  const uploadProps: UploadProps = {
    name: 'file',
    action: 'https://660d2bd96ddfa2943b33731c.mockapi.io/api/upload',
    headers: {
      authorization: 'authorization-text',
    },
    onChange(info) {
      if (info.file.status !== 'uploading') {
        console.log(info.file, info.fileList);
      }
    },
  };


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
      <Flex vertical gap={16}>
        <>
          <Typography.Title level={5}>企画概要</Typography.Title>
          <TextInput defaultValue={description} rows={2} onChange={(e) => setDescription(e.target.value)} />
        </>
        <>
          <Typography.Title level={5}>アイコン画像</Typography.Title>
          <FileUploader fileType={"image/*"} callback={(key, _) => setIconKey(key)} client={$apiMembers}/>
        </>
        <>
          <Typography.Title level={5}>訂正理由</Typography.Title>
          <TextInput placeholder="理由を入力してください" onChange={(e) => setIssueReason(e.target.value)} />
        </>
      </Flex>
      <ButtonCompact text={"企画情報の訂正を申請する"} onClick={handleSubmit}/>
    </div>
  );
};