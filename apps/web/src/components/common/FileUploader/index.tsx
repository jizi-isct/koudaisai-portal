import {useCallback, useState} from "react";
import type {apiQueryClientType} from "@/lib";
import { Flex, Upload, Button, message, Spin } from "antd";
import { UploadOutlined, LoadingOutlined } from '@ant-design/icons';
import styles from "./FileUploader.module.css";

type FileUploaderProps = {
  callback: (fileKey: string, fileName: string) => (void | Promise<void>),
  fileType?: string,
  client: apiQueryClientType,
}

export function FileUploader({callback, fileType, client}: FileUploaderProps) {
  const [isUploading, setIsUploading] = useState(false)
  const [messageApi, contextHolder] = message.useMessage();
  const {mutateAsync: mutateUploadFile} = client.useMutation("post", "/files/upload")
  const handleFileUpload = useCallback(async (file: File | undefined) => {
    setIsUploading(true)
    if (!file) {
      messageApi.error(`ファイルを指定してください`);
      return
    }
    try {
      const response = await mutateUploadFile({
        body: {
          file_name: file.name,
        },
      });

      await fetch(response.presigned_url, {
        method: "PUT",
        body: file,
        headers: {
          "Content-Type": file.type,
        },
      })

      await callback(response.key, file.name)
    } catch (e) {
      messageApi.error(`エラー：${e}`);
    } finally {
      setIsUploading(false)
    }
    setIsUploading(false)
    messageApi.success(`ファイルがアップロードされました`);
  }, [messageApi, callback, mutateUploadFile])

  return (
    <>
      {contextHolder}
      <Upload beforeUpload={(file) => {
        handleFileUpload(file); // ファイル選択後に自分の関数を呼ぶ
        return false; // アップロードは自動で行わない
      }} accept={fileType} maxCount={1}>
        <Button icon={<UploadOutlined />}>アップロードする</Button>
      </Upload>
      {isUploading && <Flex>
        <Spin indicator={<LoadingOutlined spin />} size="small" />
        <span>アップロード中</span>
      </Flex>}
    </>
  )
}