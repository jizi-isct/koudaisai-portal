import {DocumentCategory} from "@/lib";
import {useMemo, useState} from "react";
import {Button} from "@/components/generic";

type Props = {
  documentCategory: DocumentCategory,
  setDocumentCategory: (documentCategory: DocumentCategory) => void,
  finish: () => void,
}

const emojiOptions = ["📕", "📙", "📘", "📗"]

export function EditDocumentCategory({documentCategory, setDocumentCategory, finish}: Props) {
  const [title, setTitle] = useState(documentCategory.title)
  const [emoji, setEmoji] = useState<string | null>(documentCategory.emoji ?? null)
  const [isEmojiOthers, setIsEmojiOthers] = useState(!emojiOptions.includes(emoji ?? ""))

  const emojiOptionValue = useMemo(() => {
    let value
    if (emoji == null) {
      value = "__auto__"
    } else if (emojiOptions.includes(emoji)) {
      value = emoji
    } else {
      value = "__others__"
    }
    return value
  }, [emoji])


  const handleFormSubmit = () => {
    setDocumentCategory(
      {
        ...documentCategory,
        title: title,
        emoji: emoji,
      }
    )
    finish()
  }

  const handleEmojiSelect = (value: string) => {
    if (value === "__auto__") {
      setEmoji(null)
      setIsEmojiOthers(false)
    } else if (value === "__others__") {
      setEmoji("☢️")
      setIsEmojiOthers(true)
    } else {
      setEmoji(value)
      setIsEmojiOthers(false)
    }
  }

  return (
    <div>
      <div>
        <div>
          <label htmlFor="title">タイトル</label> <br/>
          <input name="title" value={title} onChange={(e) => setTitle(e.target.value)}/>
        </div>

        <div>
          <label htmlFor="emoji">絵文字</label> <br/>

          <select value={emojiOptionValue} onChange={e => handleEmojiSelect(e.target.value)}>
            <option value={"__auto__"}>自動</option>
            <option value={"📕"}>📕</option>
            <option value={"📗"}>📗</option>
            <option value={"📘"}>📘</option>
            <option value={"📙"}>📙</option>
            <option value={"__others__"}>その他</option>
          </select>
          {
            isEmojiOthers && <input name="emoji" value={emoji ?? undefined} onChange={(e) => setEmoji(e.target.value)}/>
          }
        </div>

        <Button text={"送信"} onClick={handleFormSubmit}/>
      </div>


      <aside>
        id: {documentCategory.id} <br/>
        created_at: {documentCategory.created_at} <br/>
        updated_at: {documentCategory.updated_at} <br/>
        title: {documentCategory.title}
      </aside>
    </div>
  )
}