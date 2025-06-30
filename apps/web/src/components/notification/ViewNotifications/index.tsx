import {ContentList, LoadingScreen} from "@/components/generic";
import {apiQueryClientType, useUserIdFromAccessToken} from "@/lib";
import {ContentRowNotification} from "@/components/notification/ContentRowNotification";

type ViewNotificationsProps = {
  client: apiQueryClientType
}

export function ViewNotifications({client}: ViewNotificationsProps) {
  const userId = useUserIdFromAccessToken()
  const {data: notifications} = client.useQuery("get", "/users/{user_id}/notifications", {
    params: {
      path: {
        user_id: userId ?? ""
      }
    }
  })

  if (userId && notifications) {
    if (notifications.length === 0) {
      return (
        <div style={{textAlign: "center"}}>
          <p>実行委員会からの通知はありません</p>
        </div>
      )
    }
    return (
      <ContentList
        contents={
          notifications.map(
            (value, index) =>
              <ContentRowNotification key={index} notification={value.notification}/>
          )}
      />
    )
  } else {
    return (
      <LoadingScreen/>
    )
  }
}