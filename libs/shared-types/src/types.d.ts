import { components as apiComponents } from './api_v2';
import { components as plansInfoComponents } from './plans_info_api_v1';

export type Item = apiComponents['schemas']['Item'];
export type Form = apiComponents['schemas']['Form'];
export type FormResponse = apiComponents['schemas']['FormResponse'];
export type Info = apiComponents['schemas']['Info'];

export type FormItemType =
  | 'question_text'
  | 'text'
  | 'page_break'
  | 'question_radio_button'
  | 'question_check_box';
export type SaveStatus = 'saving' | 'saved' | 'unsaved';

export type DocumentCreate = apiComponents['schemas']['CreateDocument'];
export type DocumentRead = apiComponents['schemas']['ReadDocument'];
export type DocumentUpdate = apiComponents['schemas']['UpdateDocument'];
export type DocumentFormatPdfCreate =
  apiComponents['schemas']['CreateDocumentFormatPdf'];
export type DocumentFormatPdfRead =
  apiComponents['schemas']['ReadDocumentFormatPdf'];
export type DocumentFormatPdfUpdate =
  apiComponents['schemas']['UpdateDocumentFormatPdf'];
export type DocumentFormatMarkdownCreate =
  apiComponents['schemas']['CreateDocumentFormatMarkdown'];
export type DocumentFormatMarkdownRead =
  apiComponents['schemas']['ReadDocumentFormatMarkdown'];
export type DocumentFormatMarkdownUpdate =
  apiComponents['schemas']['UpdateDocumentFormatMarkdown'];
export type DocumentFormatMiscCreate =
  apiComponents['schemas']['CreateDocumentFormatMisc'];
export type DocumentFormatMiscRead =
  apiComponents['schemas']['ReadDocumentFormatMisc'];
export type DocumentFormatMiscUpdate =
  apiComponents['schemas']['UpdateDocumentFormatMisc'];
export type DocumentCategoryCreate =
  apiComponents['schemas']['CreateDocumentCategory'];
export type DocumentCategoryRead =
  apiComponents['schemas']['ReadDocumentCategory'];
export type DocumentCategoryUpdate =
  apiComponents['schemas']['UpdateDocumentCategory'];
export type NotificationCreate = apiComponents['schemas']['NotificationCreate'];
export type NotificationRead = apiComponents['schemas']['NotificationRead'];
export type NotificationUpdate = apiComponents['schemas']['NotificationUpdate'];
export type NotificationCreateTypeMarkdown =
  apiComponents['schemas']['NotificationCreateTypeMarkdown'];
export type NotificationReadTypeMarkdown =
  apiComponents['schemas']['NotificationReadTypeMarkdown'];
export type NotificationUpdateTypeMarkdown =
  apiComponents['schemas']['NotificationUpdateTypeMarkdown'];
export type NotificationCreateTypeApprovalRequest =
  apiComponents['schemas']['NotificationCreateTypeApprovalRequest'];
export type NotificationReadTypeApprovalRequest =
  apiComponents['schemas']['NotificationReadTypeApprovalRequest'];
export type NotificationUpdateTypeApprovalRequest =
  apiComponents['schemas']['NotificationUpdateTypeApprovalRequest'];
export type FormCreate = apiComponents['schemas']['FormCreate'];
export type FormRead = apiComponents['schemas']['FormRead'];
export type FormUpdate = apiComponents['schemas']['FormUpdate'];
export type ApprovalRequestCreate =
  apiComponents['schemas']['CreateApprovalRequest'];
export type ApprovalRequestRead =
  apiComponents['schemas']['ReadApprovalRequest'];
export type ApprovalRequestUpdate =
  apiComponents['schemas']['UpdateApprovalRequest'];
export type GroupCreate = apiComponents['schemas']['GroupCreate'];
export type GroupRead = apiComponents['schemas']['GroupRead'];
export type GroupUpdate = apiComponents['schemas']['GroupUpdate'];
export type UserRead = apiComponents['schemas']['UserRead'];

export type ProductsCreate = plansInfoComponents['schemas']['ProductsCreate'];
export type ProductsRead = plansInfoComponents['schemas']['ProductsRead'];
export type ProductsUpdate = plansInfoComponents['schemas']['ProductsUpdate'];
