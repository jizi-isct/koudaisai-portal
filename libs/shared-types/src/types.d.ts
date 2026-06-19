import { components as apiComponents } from './api_v3';
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

export type DocumentCreate = apiComponents['schemas']['DocumentCreate'];
export type DocumentRead = apiComponents['schemas']['DocumentRead'];
export type DocumentUpdate = apiComponents['schemas']['DocumentUpdate'];
// 旧 format_pdf/markdown/misc のネスト型は廃止され、単一の DocumentFormat
// 判別 union(format で識別、ドキュメントに平坦化)へ統合された。
export type DocumentFormat = apiComponents['schemas']['DocumentFormat'];
export type DocumentFormatPdfRead = Extract<DocumentFormat, { format: 'pdf' }>;
export type DocumentFormatMarkdownRead = Extract<
  DocumentFormat,
  { format: 'markdown' }
>;
export type DocumentFormatMiscRead = Extract<
  DocumentFormat,
  { format: 'misc' }
>;
export type DocumentCategoryCreate =
  apiComponents['schemas']['DocumentCategoryCreate'];
export type DocumentCategoryRead =
  apiComponents['schemas']['DocumentCategoryRead'];
export type DocumentCategoryUpdate =
  apiComponents['schemas']['DocumentCategoryUpdate'];
export type NotificationCreate = apiComponents['schemas']['NotificationCreate'];
export type NotificationRead = apiComponents['schemas']['NotificationRead'];
export type NotificationUpdate = apiComponents['schemas']['NotificationUpdate'];
// 旧 type_markdown/type_approval_request のネスト型は NotificationType 判別 union へ統合。
export type NotificationType = apiComponents['schemas']['NotificationType'];
export type NotificationReadTypeMarkdown = Extract<
  NotificationType,
  { type: 'markdown' }
>;
export type NotificationReadTypeApprovalRequest = Extract<
  NotificationType,
  { type: 'approval_request' }
>;
export type FormCreate = apiComponents['schemas']['FormCreate'];
export type FormRead = apiComponents['schemas']['FormRead'];
export type FormUpdate = apiComponents['schemas']['FormUpdate'];
export type ApprovalRequestCreate =
  apiComponents['schemas']['ApprovalRequestCreate'];
export type ApprovalRequestRead =
  apiComponents['schemas']['ApprovalRequestRead'];
export type GroupCreate = apiComponents['schemas']['GroupCreate'];
export type GroupRead = apiComponents['schemas']['GroupRead'];
export type GroupUpdate = apiComponents['schemas']['GroupUpdate'];
export type GroupType = apiComponents['schemas']['GroupType'];
export type UserRead = apiComponents['schemas']['UserRead'];
export type MemberRead = apiComponents['schemas']['MemberRead'];
export type Role = apiComponents['schemas']['Role'];

export type ProductsCreate = plansInfoComponents['schemas']['ProductsCreate'];
export type ProductsRead = plansInfoComponents['schemas']['ProductsRead'];
export type ProductsUpdate = plansInfoComponents['schemas']['ProductsUpdate'];
