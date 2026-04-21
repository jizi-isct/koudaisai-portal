// "use server";
//
// import {getApiFetchClient} from "@koudaisai/shared-api";
// import ViewDocumentsWrapper from "../../../components/ViewDocumentsWrapper";
//
// export default async function Page_() {
//   const fetchClient = getApiFetchClient("https://portal.koudaisai.jp/api/v2")
//
//   // 資料一覧の取得
//   const {data, error} = await fetchClient.GET("/documents/by-category")
//   if (error || !data) {
//     throw error;
//   }
//
//   return (
//     <ViewDocumentsWrapper documents={data}/>
//   )
// }