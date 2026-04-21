"use client";

import {ContentList} from "@koudaisai/shared-ui";
import {DocumentCategoryRead, DocumentRead} from "@koudaisai/shared-types";
import {HeadingViewDocumentCategory} from "../../../documentCategory";
import {ContentRowViewDocument} from "../ContentRowViewDocument";

const headingEmojis = ["📕", "📗", "📘", "📙"];

type ViewDocumentsProps = {
  documents: Array<{category: DocumentCategoryRead | null, documents: DocumentRead[]}>;
  download: (documentId: string) => void;
}

/**
 * 資料の一覧を表示するコンポーネント
 * @param documents 資料カテゴリと資料カテゴリに属する資料の配列を含む構造体の配列
 * @param download ダウンロード関数
 * @constructor
 */
export function ViewDocuments({documents, download}: ViewDocumentsProps) {
  return (
    documents.map(({category, documents}, i) => (
      <section key={`document-category-${i}`}>
        {
          category && (
            <HeadingViewDocumentCategory documentCategory={category} defaultEmoji={headingEmojis[i % headingEmojis.length]}/>
          )
        }

        <ContentList contents={
          documents.map((document, j) => (
            <ContentRowViewDocument key={`document-${i}.${j}`} download={() => download(document.id)} document={document}/>
          ))
        }/>
      </section>
    ))
  )
}