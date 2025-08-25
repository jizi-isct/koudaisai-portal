import {components} from "./api_v2";

export type Item = components["schemas"]["Item"];
export type Form = components["schemas"]["Form"];
export type FormResponse = components["schemas"]["FormResponse"]
export type Info = components["schemas"]["Info"]

export type FormItemType = ("question_text" | "text" | "page_break" | "question_radio_button" | "question_check_box");
export type SaveStatus = "saving" | "saved" | "unsaved";

export type DocumentCreate = components["schemas"]["CreateDocument"];
export type DocumentRead = components["schemas"]["ReadDocument"];
export type DocumentUpdate = components["schemas"]["UpdateDocument"];
export type DocumentFormatPdfCreate = components["schemas"]["CreateDocumentFormatPdf"];
export type DocumentFormatPdfRead = components["schemas"]["ReadDocumentFormatPdf"];
export type DocumentFormatPdfUpdate = components["schemas"]["UpdateDocumentFormatPdf"];
export type DocumentFormatMarkdownCreate = components["schemas"]["CreateDocumentFormatMarkdown"];
export type DocumentFormatMarkdownRead = components["schemas"]["ReadDocumentFormatMarkdown"];
export type DocumentFormatMarkdownUpdate = components["schemas"]["UpdateDocumentFormatMarkdown"];
export type DocumentFormatMiscCreate = components["schemas"]["CreateDocumentFormatMisc"];
export type DocumentFormatMiscRead = components["schemas"]["ReadDocumentFormatMisc"];
export type DocumentFormatMiscUpdate = components["schemas"]["UpdateDocumentFormatMisc"];
export type DocumentCategoryCreate = components["schemas"]["CreateDocumentCategory"];
export type DocumentCategoryRead = components["schemas"]["ReadDocumentCategory"];
export type DocumentCategoryUpdate = components["schemas"]["UpdateDocumentCategory"];
export type NotificationCreate = components["schemas"]["NotificationCreate"];
export type NotificationRead = components["schemas"]["NotificationRead"];
export type NotificationUpdate = components["schemas"]["NotificationUpdate"];
export type NotificationCreateTypeMarkdown = components["schemas"]["NotificationCreateTypeMarkdown"];
export type NotificationReadTypeMarkdown = components["schemas"]["NotificationReadTypeMarkdown"];
export type NotificationUpdateTypeMarkdown = components["schemas"]["NotificationUpdateTypeMarkdown"];
export type NotificationCreateTypeApprovalRequest = components["schemas"]["NotificationCreateTypeApprovalRequest"];
export type NotificationReadTypeApprovalRequest = components["schemas"]["NotificationReadTypeApprovalRequest"];
export type NotificationUpdateTypeApprovalRequest = components["schemas"]["NotificationUpdateTypeApprovalRequest"];
export type FormCreate = components["schemas"]["FormCreate"];
export type FormRead = components["schemas"]["FormRead"];
export type FormUpdate = components["schemas"]["FormUpdate"];
export type ApprovalRequestCreate = components["schemas"]["CreateApprovalRequest"];
export type ApprovalRequestRead = components["schemas"]["ReadApprovalRequest"];
export type ApprovalRequestUpdate = components["schemas"]["UpdateApprovalRequest"];
export type GroupCreate = components["schemas"]["GroupCreate"];
export type GroupRead = components["schemas"]["GroupRead"];
export type GroupUpdate = components["schemas"]["GroupUpdate"];
export type UserRead = components["schemas"]["UserRead"];