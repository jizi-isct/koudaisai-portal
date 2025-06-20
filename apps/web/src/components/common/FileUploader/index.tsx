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
  const handleFileUpload = useCallback(async (file: File | undefined) => {
    setIsUploading(true)
    if (!file) {
      setError("ファイルを指定してください")
      return
    }

    const { data, error } = await client.POST("/files/upload", {
        body: {
          file_name: file.name,
        },
      });

    if (data) {
      await fetch(data.presigned_url, {
        method: "PUT",
        body: file
      })

      await callback(data.key, file.name)
    } else {
      setError(`エラー：${error}`)
    }
    setIsUploading(false)
  }, [callback])

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