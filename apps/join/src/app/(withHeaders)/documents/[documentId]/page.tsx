import {getApiFetchClient} from "@koudaisai/shared-api";
import ViewDocumentWrapper from "../../../../components/ViewDocumentWrapper";

export async function generateStaticParams() {
  const fetchClient = getApiFetchClient("https://portal.koudaisai.jp/api/v2")
  const {data, error} = await fetchClient.GET("/documents")
  if (error || !data) {
    throw error;
  }
  return data.map((d) => ({documentId: d.id}))
}

export default async function Page({params}: PageProps<'/documents/[documentId]'>) {
  const {documentId} = await params
  const fetchClient = getApiFetchClient("https://portal.koudaisai.jp/api/v2")
  const {data, error} = await fetchClient.GET("/documents/{document_id}", {
    params: {
      path: {document_id: documentId}
    }
  })
  if (error || !data) {
    throw error;
  }

  return (
    <ViewDocumentWrapper document={data}/>
  )
}