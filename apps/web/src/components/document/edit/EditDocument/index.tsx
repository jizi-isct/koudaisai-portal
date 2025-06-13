"use client";

import {
  DocumentFormatMarkdownUpdate,
  DocumentFormatMiscUpdate,
  DocumentFormatPdfUpdate,
  DocumentRead,
  DocumentUpdate,
  SaveStatus
} from "@/lib";
import {SaveStatus as CSaveStatus, TextInput} from "@/components/generic";
import {useState} from "react";
import {useWriteDocumentContext} from "@/contexts/WriteDocumentContext";
import {DocumentCategorySelector} from "../../common/DocumentCategorySelector";
import {DocumentRequiredOneOfScopesCheckbox} from "../../common/DocumentRequiredOneOfScopesCheckbox";
import {EditDocumentFormatMarkdown} from "../EditDocumentFormatMarkdown";
import {EditDocumentFormatMisc} from "../EditDocumentFormatMisc";
import {EditDocumentFormatPdf} from "../EditDocumentFormatPdf";

type EditDocumentProps = {
  document: DocumentRead
}

export function EditDocument({document}: EditDocumentProps) {
  const {
    categories,
    updateDocument: updateDocument_,
  } = useWriteDocumentContext()
  const [status, setStatus] = useState<SaveStatus>("saved");
  const [error, setError] = useState<string | null>(null)
  const [title, setTitle] = useState(document.title)
  const [category, setCategory] = useState<string | null>(document.category)
  const [requiredOneOfScopes, setRequiredOneOfScopes] = useState<string[]>(document.required_one_of_scopes)
  const [formatPdf, setFormatPdf] = useState<DocumentFormatPdfUpdate | undefined>(document.format_pdf)
  const [formatMisc, setFormatMisc] = useState<DocumentFormatMiscUpdate | undefined>(document.format_misc)
  const [formatMarkdown, setFormatMarkdown] = useState<DocumentFormatMarkdownUpdate | undefined>(document.format_markdown)
  const updateDocument = async (id: string, update: DocumentUpdate) => {
    setStatus("saving")
    try {
      await updateDocument_(id, update)
    } catch (e) {
      setStatus("unsaved")
      setError(e instanceof Error ? e.message : String(e))
      return
    }
    setStatus("saved")
    setError(null)
  }

  const handleTitleChange = async (newTitle: string) => {
    setTitle(newTitle)
    await updateDocument(
      document.id,
      {
        title: newTitle,
      }
    )
  }

  const handleCategoryChange = async (categoryId: string | null) => {
    setCategory(categoryId)
    await updateDocument(
      document.id,
      {
        category: categoryId,
      }
    )
  }

  const handleScopeChange = async (requiredOneOfScopes: string[]) => {
    setRequiredOneOfScopes(requiredOneOfScopes)
    await updateDocument(
      document.id,
      {
        required_one_of_scopes: requiredOneOfScopes
      }
    )
  }

  const handleFormatPdfChange = async (format: DocumentFormatPdfUpdate) => {
    setFormatPdf(format)
    setFormatMisc(undefined)
    setFormatMarkdown(undefined)
    await updateDocument(
      document.id,
      {
        format_pdf: format,
        format_misc: undefined,
        format_markdown: undefined
      }
    )
  }

  const handleFormatMiscChange = async (format: DocumentFormatMiscUpdate) => {
    setFormatMisc(format)
    setFormatPdf(undefined)
    setFormatMarkdown(undefined)
    await updateDocument(
      document.id,
      {
        format_pdf: undefined,
        format_misc: format,
        format_markdown: undefined
      }
    )
  }

  const handleFormatMarkdownChange = async (format: DocumentFormatMarkdownUpdate) => {
    setFormatMarkdown(format)
    setFormatPdf(undefined)
    setFormatMisc(undefined)
    await updateDocument(
      document.id,
      {
        format_pdf: undefined,
        format_misc: undefined,
        format_markdown: format
      }
    )
  }

  return (
    <div>
      <CSaveStatus status={status}/>
      {error && <span style={{color: "red"}}>error</span>}
      <div>
        <label>
          タイトル
          <TextInput
            value={title}
            setValue={str => handleTitleChange(str)}
            paragraph={false}
          />
        </label>
      </div>
      <div>
        <DocumentCategorySelector categories={categories} categoryId={category}
                                  setCategoryId={handleCategoryChange}/>
      </div>
      <div>
        <DocumentRequiredOneOfScopesCheckbox requiredOneOfScopes={requiredOneOfScopes}
                                             setRequiredOneOfScopes={handleScopeChange}/>
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
      <aside>
        id: {document.id} <br/>
        created_at: {document.created_at} <br/>
        updated_at: {document.updated_at} <br/>
        title: {document.title} <br/>
        category: {document.category} <br/>
        required_one_of_scopes: {document.required_one_of_scopes} <br/>
        format_pdf.file_key: {document.format_pdf?.file_key} <br/>
        format_misc.file_key: {document.format_misc?.file_key} <br/>
      </aside>
    </div>
  )
}