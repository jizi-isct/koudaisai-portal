"use client";

import {Button} from "@/components/generic";
import {useState} from "react";
import {Document, DocumentCategory, fetchClientAdmin} from "@/lib";
import {EditDocument} from "../EditDocument";

type Props = {
  onCreate: (document: Document) => void,
  categories: Array<DocumentCategory>,
  initialCategory: DocumentCategory
}

export function CreateDocument({categories, onCreate, initialCategory}: Props) {
  const [document, setDocument] = useState<Document>({
    title: "",
    category: initialCategory.id,
    format_pdf: {
      file_key: ""
    },
    required_one_of_scopes: []
  })
  const [error, setError] = useState<string | null>(null)

  const handleFormatChange = (format: "pdf" | "markdown") => {
    if (format === "pdf") {
      setDocument({
        ...document,
        format_pdf: {
          file_key: ""
        },
        format_markdown: undefined
      })
    } else if (format === "markdown") {
      setDocument({
        ...document,
        format_markdown: {
          content: ""
        },
        format_pdf: undefined
      })
    }
  }

  const handleCreateAction = async () => {
    const {data, error} = await fetchClientAdmin.POST("/documents", {
      body: document
    })

    if (data) {
      onCreate(data)
    } else {
      setError("通信エラー: " + error)
    }
  }

  return (
    <div>
      <div>
        <h3>フォーマット</h3>
        <div>
          <input type="radio" name="format" checked={document.format_pdf !== undefined}
                 onChange={() => handleFormatChange("pdf")}/>
          <span>PDF</span>
        </div>
        <div>
          <input type="radio" name="format" checked={document.format_markdown !== undefined}
                 onChange={() => handleFormatChange("markdown")}/>
          <span>Markdown</span>
        </div>
      </div>
      <EditDocument categories={categories} document={document} setDocument={setDocument}/>
      {error && <div>{error}</div>}
      <Button text={"作成"} onClick={() => handleCreateAction()}></Button>
    </div>
  )
}