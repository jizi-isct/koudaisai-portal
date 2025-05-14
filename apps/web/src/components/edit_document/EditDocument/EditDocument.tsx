"use client";

import {Document, DocumentCategory, fetchClientAdmin} from "@/lib";
import {TextInput} from "@/components/generic";
import {useState} from "react";

type Props = {
  categories: Array<DocumentCategory>,
  document: Document,
  setDocument: (document: Document) => void
}

export function EditDocument({categories, document, setDocument}: Props) {
  const [status, setStatus] = useState<string | null>(null);

  const handleTitleChange = (newTitle: string) => {
    setDocument({
      ...document,
      title: newTitle
    })
  }

  const handleCategoryChange = (id: string) => {
    if (id === "__null__") {
      console.error("INVALID CATEGORY ID: " + id + "")
    }
    setDocument({
      ...document,
      category: id
    })
  }

  const handleScopeChange = (scope: string, allowed: boolean) => {
    if (allowed) {
      if (!document.required_one_of_scopes?.includes(scope)) {
        setDocument({
          ...document,
          required_one_of_scopes: (document.required_one_of_scopes ? document.required_one_of_scopes : []).concat(scope)
        })
      }
    } else {
      setDocument({
        ...document,
        required_one_of_scopes: (document.required_one_of_scopes ? document.required_one_of_scopes : [])
          .filter((value) => value !== scope)
      })
    }
  }

  const handleFileUpload = async (file: File, format: "pdf" | "misc") => {
    setStatus("ファイルをアップロード中...")
    const {data, error} = await fetchClientAdmin.POST("/files/upload", {
      body: {
        file_name: file.name,
      }
    });

    if (data) {
      await fetch(data.presigned_url!, {
        method: "PUT",
        body: file
      })

      if (format === "pdf") {
        setDocument({
          ...document,
          format_pdf: {
            file_key: data.key!,
            file_name: file.name,
          }
        })
      } else if (format === "misc") {
        setDocument({
          ...document,
          format_misc: {
            file_key: data.key!,
            file_name: file.name,
          }
        })
      }
      setStatus("FILE UPLOAD SUCCESS: " + data.key + "")
    } else {
      setStatus("FILE UPLOAD ERROR: " + error)
    }
  }

  const handleMarkdownChange = (content: string) => {
    setDocument({
      ...document,
      format_markdown: {
        content: content
      }
    })
  }

  return (
    <div>
      <div>
        <h3>タイトル</h3>
        <TextInput
          value={document.title}
          setValue={str => handleTitleChange(str)}
          paragraph={false}
        />
      </div>
      <div>
        <h3>カテゴリ</h3>
        <select value={document.category ?? "__null__"} onChange={e => handleCategoryChange(e.target.value)}>
          {
            categories?.map((data, i) => <option key={`option-${i}`} value={data.id}>{data.title}</option>)
          }
          {
            document.category ?? <option key={`option-null`} value={"__null__"}>カテゴリなし</option>
          }
        </select>
      </div>
      <div>
        <h3>閲覧権限管理(チェックした対象が閲覧可能になります)</h3>
        <div>
          <input type="checkbox" checked={document.required_one_of_scopes!.includes("none")} onChange={e => {
            handleScopeChange("none", e.target.checked)
          }}/>
          <span>非ログイン</span>
        </div>
        <div>
          <input type="checkbox" checked={document.required_one_of_scopes!.includes("booth")} onChange={e => {
            handleScopeChange("booth", e.target.checked)
          }}/>
          <span>模擬店企画</span>
        </div>
        <div>
          <input type="checkbox" checked={document.required_one_of_scopes!.includes("general")} onChange={e => {
            handleScopeChange("general", e.target.checked)
          }}/>
          <span>一般企画</span>
        </div>
        <div>
          <input type="checkbox" checked={document.required_one_of_scopes!.includes("stage")} onChange={e => {
            handleScopeChange("stage", e.target.checked)
          }}/>
          <span>ステージ企画</span>
        </div>
        <div>
          <input type="checkbox" checked={document.required_one_of_scopes!.includes("labo")} onChange={e => {
            handleScopeChange("labo", e.target.checked)
          }}/>
          <span>研究室企画</span>
        </div>
      </div>
      {
        document.format_pdf &&
              <>
                <div>
                  <h3>pdfファイルをアップロードする</h3>
                  {
                    document.format_pdf.file_key === ""
                      ? "ファイルをアップロードしてください。"
                      : "すでにファイルがアップロードされています。"
                  }
                  <input
                          type="file"
                          accept={"application/pdf"}
                          onChange={e => handleFileUpload(e.target.files![0], "pdf")}
                  />
                </div>
              </>
      }
      {
        document.format_misc &&
              <>
                <div>
                  <h3>ファイルをアップロードする</h3>
                  {
                    document.format_misc.file_key === ""
                      ? "ファイルをアップロードしてください。"
                      : "すでにファイルがアップロードされています。"
                  }
                  <input
                          type="file"
                          onChange={e => handleFileUpload(e.target.files![0], "misc")}
                  />
                </div>
              </>
      }
      {
        document.format_markdown &&
              <>
                <div>
                  <h3>内容</h3>
                  <TextInput
                          value={document.format_markdown!.content}
                          setValue={str => handleMarkdownChange(str)}
                          paragraph={true}
                  />
                </div>
              </>
      }
      {status && <div>{status}</div>}
    </div>
  )
}