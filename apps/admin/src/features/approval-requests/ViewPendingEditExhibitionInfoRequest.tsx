import type {
  ApprovalRequestRead,
  events26Components,
} from '@koudaisai/shared-types';
import { getFilesRedirectUrl } from '@koudaisai/shared-utils';
import { API_URL, EVENTS26_API_URL } from 'astro:env/client';

type Props = {
  approvalRequest: ApprovalRequestRead;
  project: events26Components['schemas']['Project'];
};

export function ViewPendingEditExhibitionInfoRequest({
  approvalRequest,
  project,
}: Props) {
  return (
    <div>
      <h2>企画内容紹介文</h2>
      {approvalRequest.description ? (
        <div>
          <h3>変更前</h3>
          {project.description}
          <h3>変更後</h3>
          {approvalRequest.description}
        </div>
      ) : (
        <div>変更なし</div>
      )}

      <h2>企画アイコン</h2>
      {approvalRequest.icon_key ? (
        <div>
          <h3>変更前</h3>
          <img
            src={`${EVENTS26_API_URL}/v1/projects/${encodeURIComponent(project.id)}/icon`}
            alt="現在の企画のアイコン"
            width={128}
            height={128}
          />
          <h3>変更後</h3>
          <img
            src={getFilesRedirectUrl(API_URL, approvalRequest.icon_key)}
            alt="新しい企画アイコン"
            width={128}
            height={128}
          />
        </div>
      ) : (
        <div>変更なし</div>
      )}
    </div>
  );
}
