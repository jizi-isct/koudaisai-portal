import { api } from '@/features/api/api';

const ADDITIONAL_INFO_PATH =
  '/events26/projects/{project_id}/details/additional_info' as const;

type AdditionalInfoRequestOptions = {
  params: { path: { project_id: string } };
};

type AdditionalInfoApiResult = {
  data?: unknown;
  error?: unknown;
  response: Response;
};

type AdditionalInfoApiClient = {
  GET: (
    path: typeof ADDITIONAL_INFO_PATH,
    options: AdditionalInfoRequestOptions,
  ) => Promise<AdditionalInfoApiResult>;
  PUT: (
    path: typeof ADDITIONAL_INFO_PATH,
    options: AdditionalInfoRequestOptions & { body: string },
  ) => Promise<AdditionalInfoApiResult>;
};

// このエンドポイントは backend 実装後に OpenAPI から生成される予定。
// それまでは既存の認証付き client を、このエンドポイントに限って型付けして使う。
const additionalInfoApi = api as unknown as AdditionalInfoApiClient;

function getErrorMessage(error: unknown, response: Response): string {
  if (typeof error === 'string' && error.length > 0) {
    return error;
  }

  if (
    typeof error === 'object' &&
    error !== null &&
    'message' in error &&
    typeof error.message === 'string'
  ) {
    return error.message;
  }

  return `APIリクエストに失敗しました（${response.status}）`;
}

/** 参加団体の企画追加情報を取得する。未登録の場合は空文字として扱う。 */
export async function getProjectAdditionalInfo(
  projectId: string,
): Promise<string> {
  const { data, error, response } = await additionalInfoApi.GET(
    ADDITIONAL_INFO_PATH,
    {
      params: { path: { project_id: projectId } },
    },
  );

  if (response.status === 404) {
    return '';
  }

  if (!response.ok) {
    throw new Error(getErrorMessage(error, response));
  }

  if (typeof data === 'string') {
    return data;
  }

  // backend のレスポンス型が確定するまで、フィールド付きの形式にも対応する。
  if (
    typeof data === 'object' &&
    data !== null &&
    'additional_info' in data &&
    typeof data.additional_info === 'string'
  ) {
    return data.additional_info;
  }

  return '';
}

/** 参加団体の企画追加情報を更新する。 */
export async function updateProjectAdditionalInfo(
  projectId: string,
  additionalInfo: string,
): Promise<void> {
  const { error, response } = await additionalInfoApi.PUT(
    ADDITIONAL_INFO_PATH,
    {
      params: { path: { project_id: projectId } },
      body: additionalInfo,
    },
  );

  if (!response.ok) {
    throw new Error(getErrorMessage(error, response));
  }
}
