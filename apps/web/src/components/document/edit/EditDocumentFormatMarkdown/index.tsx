"use client";

import {DocumentFormatMarkdownRead, DocumentFormatMarkdownUpdate,} from "@/lib";
import {TextInput} from "@/components/generic";

type EditDocumentFormatMarkdownProps = {
  format: DocumentFormatMarkdownRead,
  updateFormat: (format: DocumentFormatMarkdownUpdate) => void
}

export function EditDocumentFormatMarkdown({format, updateFormat}: EditDocumentFormatMarkdownProps) {
  const handleContentChange = (content: string) => {
    updateFormat({
      content: content
    })
  }

  return (
    <label>
      内容
      <TextInput
        value={format.content}
        setValue={str => handleContentChange(str)}
        paragraph={true}
      />
    </label>
  )
}