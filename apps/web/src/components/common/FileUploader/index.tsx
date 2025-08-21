import {useCallback, useState} from "react";
import type {apiQueryClientType} from "@/lib";
import {Loader} from "@/components/generic/Loader";
import styles from "./FileUploader.module.css";

type FileUploaderProps = {
  callback: (fileKey: string, fileName: string) => (void | Promise<void>),
  fileType?: string,
  client: apiQueryClientType,
}

export function FileUploader({callback, fileType, client}: FileUploaderProps) {
  const [isUploading, setIsUploading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const {mutateAsync: mutateUploadFile} = client.useMutation("post", "/files/upload")
  const handleFileUpload = useCallback(async (file: File | undefined) => {
    setIsUploading(true)
    if (!file) {
      setError("ファイルを指定してください")
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
      setError(`エラー：${e}`)
    } finally {
      setIsUploading(false)
    }
    setIsUploading(false)
  }, [callback, mutateUploadFile])

  return (
    <div className={styles.root}>
      <input
        type="file"
        accept={fileType}
        onChange={e => handleFileUpload(e.target.files?.[0])}
      />
      {isUploading && <><Loader/><span>アップロード中</span></>}
      {error && <span style={{color: "red"}}>{error}</span>}
    </div>
  )
}