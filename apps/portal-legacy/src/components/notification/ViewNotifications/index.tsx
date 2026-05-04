import {ContentList, LoadingScreen} from "@koudaisai/shared-ui";
import {ApiQueryClient} from "@koudaisai/shared-api";
import { useUserIdFromAccessToken } from "@/lib/api";
import {ContentRowNotification} from "@/components/notification/ContentRowNotification";

type ViewNotificationsProps = {
  client: ApiQueryClient
}

export function ViewNotifications({client}: ViewNotificationsProps) {
  const userId = useUserIdFromAccessToken();

  // API からは全件取得
  const {data: notifications, isLoading} = client.useQuery(
    "get",
    "/users/{user_id}/notifications",
    {
      params: {
        path: {
          user_id: userId ?? ""
        }
      }
    }
  );

  const notifications_sorted = notifications?.sort((a, b) => {
    return new Date(b.notification.created_at).getTime() - new Date(a.notification.created_at).getTime();
  });

  if (!userId || isLoading) {
    return <LoadingScreen />;
  }

  if (!notifications_sorted || notifications_sorted.length === 0) {
    return (
      <div style={{textAlign: "center"}}>
        <p>実行委員会からの通知はありません</p>
      </div>
    );
  }
  return (
    <div>
      <ContentList
      pagination={true} pageSize={5}
        contents={notifications_sorted.map((value, index) =>
          <ContentRowNotification key={value.notification.id} notification={value.notification} />
        )}
      />
    </div>
  );
}
