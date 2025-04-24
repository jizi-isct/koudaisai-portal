"use client";

import {Heading1, Heading2, Modal, ContentList, Content} from "@koudaisai-portal/ui-generic"
import {useState} from "react";
import { defaultLayoutPlugin } from '@react-pdf-viewer/default-layout';
import {fullScreenPlugin} from "@react-pdf-viewer/full-screen";
import {documentDataNoLogin} from "@/lib/lib";
import {Viewer, Worker} from "@react-pdf-viewer/core";

import "@koudaisai-portal/ui-generic/css"

import '@react-pdf-viewer/core/lib/styles/index.css';
import '@react-pdf-viewer/default-layout/lib/styles/index.css';


export default function Page_() {
  const [isModalOpen, setModalOpen] = useState(false)
  const [pdfLink, setPdfLink] = useState<string>("")

  function getDocument(title: string, url: string): Content {
    return {
      title: title,
      onClick: () => {
        setPdfLink(url)
        setModalOpen(true)
      },
    }
  }

  return (
    <>
      <Heading1 emoji="📚">資料一覧</Heading1>
      <section>
        <Heading2 emoji="📕">研究室公開企画</Heading2>
        <ContentList
          contents={
            documentDataNoLogin["研究室公開企画"]
              .map((data) =>
                getDocument(data.title, data.url))
          }
        />
      </section>
      <section>
        <Heading2 emoji="📗">プライバシーポリシー</Heading2>
        <ContentList
          contents={
            documentDataNoLogin["プライバシーポリシー"]
              .map((data) =>
                getDocument(data.title, data.url))
          }
        />
      </section>
      <section>
        <Heading2 emoji="📘">工大祭2025公式ロゴ配布</Heading2>
        <ContentList
          contents={
            documentDataNoLogin["工大祭2025公式ロゴ配布"]
              .map((data) =>
                getDocument(data.title, data.url))
          }
        />
      </section>
      <section style={{marginBottom: "2em"}}>
        <Heading2 emoji="📙">工大祭2025参加説明会</Heading2>
        <ContentList
          contents={
            documentDataNoLogin["工大祭2025参加説明会"]
              .map((data) =>
                getDocument(data.title, data.url))
          }
        />
      </section>

      <Modal
        isOpen={isModalOpen}
        setOpen={setModalOpen}
      >
        <Worker workerUrl={`https://unpkg.com/pdfjs-dist@3.11.174/build/pdf.worker.min.js`}>
          <Viewer
            fileUrl={pdfLink}
            plugins={[defaultLayoutPlugin(), fullScreenPlugin({
              getFullScreenTarget: (pagesContainer) => pagesContainer.closest('[data-testid="default-layout__body"]')!
            })]}
          />
        </Worker>
      </Modal>
    </>
  )
}

