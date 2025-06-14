"use client";

import {useWriteDocumentContext} from "@/contexts/WriteDocumentContext";
import {Button, TextInput} from "@/components/generic";
import {DocumentCategorySelector} from "../../common/DocumentCategorySelector";
import {DocumentRequiredOneOfScopesCheckbox} from "../../common/DocumentRequiredOneOfScopesCheckbox";
import {EditDocumentFormatPdf} from "@/components/document/edit/EditDocumentFormatPdf";
import {EditDocumentFormatMisc} from "@/components/document/edit/EditDocumentFormatMisc";
import {EditDocumentFormatMarkdown} from "@/components/document/edit/EditDocumentFormatMarkdown";
import {
  DocumentCreate,
  DocumentFormatMarkdownCreate,
  DocumentFormatMarkdownUpdate,
  DocumentFormatMiscCreate,
  DocumentFormatPdfCreate
} from "@/lib";
import {useState} from "react";
import {Loader} from "@/components/generic/Loader";

type CreateNewDocumentProps = {
  title?: string,
  category?: string | null,
  requiredOneOfScopes?: string[],
  callback: () => void
}

export function CreateNewDocument(
  {
    title: _title,
    category: _category,
    requiredOneOfScopes: _requiredOneOfScopes,
    callback
  }: CreateNewDocumentProps) {
  const {
    categories,
    createDocument
  } = useWriteDocumentContext()
  const [title, setTitle] = useState<string>(_title ?? "")
  const [category, setCategory] = useState<string | null>(_category ?? null)
  const [requiredOneOfScopes, setRequiredOneOfScopes] = useState<string[]>(_requiredOneOfScopes ?? []);
  const [formatPdf, setFormatPdf] = useState<DocumentFormatPdfCreate | undefined>({
    file_key: "",
    file_name: ""
  })
  const [formatMarkdown, setFormatMarkdown] = useState<DocumentFormatMarkdownCreate | undefined>(undefined)
  const [formatMisc, setFormatMisc] = useState<DocumentFormatMiscCreate | undefined>(undefined)
  const [isSaving, setIsSaving] = useState<boolean>(false)

  const handleFormatPdfChange = (format: DocumentFormatPdfCreate) => {
    setFormatPdf(format)
    setFormatMarkdown(undefined)
    setFormatMisc(undefined)
  }

  const handleFormatMarkdownChange = (format: DocumentFormatMarkdownUpdate) => {
    setFormatMarkdown(format)
    setFormatPdf(undefined)
    setFormatMisc(undefined)
  }

  const handleFormatMiscChange = (format: DocumentFormatMiscCreate) => {
    setFormatMisc(format)
    setFormatPdf(undefined)
    setFormatMarkdown(undefined)
  }

  const handleCreateDocument = async () => {
    setIsSaving(true)
    const newDocument: DocumentCreate = {
      title,
      category,
      required_one_of_scopes: requiredOneOfScopes,
      format_pdf: formatPdf,
      format_markdown: formatMarkdown,
      format_misc: formatMisc
    }
    await createDocument(newDocument)
    setIsSaving(false)
    callback()
  }

  return (
    <div>
      <div>
        <label>
          タイトル
          <TextInput
            value={title}
            setValue={setTitle}
            paragraph={false}
          />
        </label>
      </div>
      <div>
        <DocumentCategorySelector categories={categories} categoryId={category}
                                  setCategoryId={setCategory}/>
      </div>
      <div>
        <DocumentRequiredOneOfScopesCheckbox requiredOneOfScopes={requiredOneOfScopes}
                                             setRequiredOneOfScopes={setRequiredOneOfScopes}/>
      </div>
      <div>
        <h3>フォーマット</h3>
        <div>
          <label>
            <input type="radio" name="format" value="pdf" checked={!!formatPdf}
                   onChange={() => handleFormatPdfChange({file_key: "", file_name: ""})}/>
            PDF
          </label>
          <label>
            <input type="radio" name="format" value="markdown" checked={!!formatMarkdown}
                   onChange={() => handleFormatMarkdownChange({content: ""})}/>
            Markdown
          </label>
          <label>
            <input type="radio" name="format" value="misc" checked={!!formatMisc}
                   onChange={() => handleFormatMiscChange({file_key: "", file_name: ""})}/>
            その他
          </label>
        </div>
        {formatPdf &&
                <EditDocumentFormatPdf
                        format={formatPdf}
                        updateFormat={handleFormatPdfChange}/>
        }
        {formatMisc &&
                <EditDocumentFormatMisc
                        format={formatMisc}
                        updateFormat={handleFormatMiscChange}/>
        }
        {formatMarkdown &&
                <EditDocumentFormatMarkdown
                        format={formatMarkdown}
                        updateFormat={handleFormatMarkdownChange}/>
        }
      </div>
      {isSaving ? <span>作成中...<Loader/></span> : <Button text={"作成"} onClick={handleCreateDocument}/>}
    </div>
  )
}