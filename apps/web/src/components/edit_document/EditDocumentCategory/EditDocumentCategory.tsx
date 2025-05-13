import {DocumentCategory} from "@/lib";
import {useState} from "react";

type Props = {
  documentCategory: DocumentCategory,
  setDocumentCategory: (documentCategory: DocumentCategory) => void
}

export function EditDocumentCategory({documentCategory, setDocumentCategory}: Props) {
  const [title, setTitle] = useState(documentCategory.title)

  const handleFormSubmit = () => {
    setDocumentCategory(
      {
        ...documentCategory,
        title: title,
      }
    )
  }

  return (
    <div>
      <form onSubmit={handleFormSubmit}>
        <div>
          <label htmlFor="title">タイトル</label> <br/>
          <input name="title" value={title} onChange={(e) => setTitle(e.target.value)}/>
        </div>

        <button type="submit">送信</button>
      </form>


      <aside>
        id: {documentCategory.id} <br/>
        created_at: {documentCategory.created_at} <br/>
        updated_at: {documentCategory.updated_at} <br/>
        title: {documentCategory.title}
      </aside>
    </div>
  )
}