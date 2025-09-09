import {useState} from "react";
import {Pagination} from "antd";
import {ContentWrapper, ContentList, LoadingScreen} from "@/components/generic";
import {apiQueryClientType, useUserIdFromAccessToken} from "@/lib";
import {ContentRowNotification} from "@/components/notification/ContentRowNotification";

type ViewNotificationsProps = {
  client: apiQueryClientType
}

export function ViewNotifications({client}: ViewNotificationsProps) {
  const userId = useUserIdFromAccessToken();

  // ページ管理
  const [page, setPage] = useState(1);
  const pageSize = 10; // 1ページの表示件数

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

  // 現在のページに応じて slice
  const start = (page - 1) * pageSize;
  const end = start + pageSize;
  const currentPageNotifications = notifications.slice(start, end);

  return (
    <div>
      <ContentList
        contents={currentPageNotifications.map((value, index) =>
          <ContentRowNotification key={index} notification={value.notification} />
        )}
      />
      <div style={{display: "flex", justifyContent: "center", marginTop: "16px"}}>
        <Pagination
          current={page}
          pageSize={pageSize}
          total={notifications.length}
          onChange={(p) => setPage(p)}
          showSizeChanger={false}
        />
      </div>
    </div>
  );
}
