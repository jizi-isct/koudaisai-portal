"use client";

import {Viewer, Worker} from "@react-pdf-viewer/core";
import {Modal} from "@koudaisai-portal/ui-generic";
import {Document, fetchClientNoAuth} from "@koudaisai-portal/util";
import ReactMarkdown from "react-markdown";
import {useEffect, useState} from "react";

type Props = {
  document: Document
  isModalOpen: boolean
  setModalOpen: (isModalOpen: boolean) => void
}

export function DocumentModal({document, isModalOpen, setModalOpen}: Props) {
  const [pdfUrl, setPdfUrl] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      if (document.format_pdf) {
        const {data, error} = await fetchClientNoAuth.GET("/files/download", {
          params: {
            query: {
              key: document.format_pdf.file_key,
            }
          }
        })
        if (data) {
          setPdfUrl(data.presigned_url ?? null)
        } else {
          console.log("FILE DOWNLOAD ERROR: ", error)
        }
      }
    })()
  }, [document.format_pdf, pdfUrl])

  return (
    <Modal
      isOpen={isModalOpen}
      setOpen={setModalOpen}
    >
      {
        document.format_pdf && pdfUrl &&
              <Worker workerUrl={`https://unpkg.com/pdfjs-dist@3.11.174/build/pdf.worker.min.js`}>
                <Viewer
                        fileUrl={pdfUrl ?? ""}
                />
              </Worker>
      }
      {
        document.format_markdown &&
              <div style={{textAlign: "left", padding: "10px"}}>
                <ReactMarkdown>
                  {document.format_markdown.content}
                </ReactMarkdown>
              </div>
      }
    </Modal>
  )
}