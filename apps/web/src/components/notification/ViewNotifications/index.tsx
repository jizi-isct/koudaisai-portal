import {ContentList, LoadingScreen} from "@/components/generic";
import {apiQueryClientType, useUserIdFromAccessToken} from "@/lib";
import {ContentRowNotification} from "@/components/notification/ContentRowNotification";

type ViewNotificationsProps = {
  client: apiQueryClientType
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

  if (!userId || isLoading) {
    return <LoadingScreen />;
  }

  if (!notifications || notifications.length === 0) {
    return (
      <div style={{textAlign: "center"}}>
        <p>実行委員会からの通知はありません</p>
      </div>
    );
  }
  return (
    <div>
      <ContentList
      pagination={true} pageSize={10}
        contents={notifications.map((value, index) =>
          <ContentRowNotification key={index} notification={value.notification} />
        )}
      />
    </div>
  );
}
