'use client'; // クライアントサイドコンポーネントとして実行するために追加

import {Suspense, useEffect, useState} from 'react';
import {useRouter} from 'next/navigation';
import {$apiMembers, getTokensMembers} from "@/lib";
import "../../globals.css";

import {Heading1, LoadingScreen} from '@/components/generic';
import {UserInfoCard} from "@/components/UserInfoCard/UserInfoCard";
import {ViewNotifications} from "@/components/notification/ViewNotifications";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {ExhibitorCard} from "@/components/exhibitor/ExhibitorCard";
import {EditModal} from "@/components/exhibitor/EditModal/EditModal";

export default function Page() {
  return (
    <Suspense fallback={<LoadingScreen/>}>
      <QueryClientProvider client={new QueryClient()}>
        <Inner/>
      </QueryClientProvider>
    </Suspense>
  )
}

function Inner() {
  const router = useRouter();
  const [authenticated, setAuthenticated] = useState<boolean | null>(null);
  const {data: user} = $apiMembers.useQuery("get", "/users/{user_id}", {
    params: {
      path: {
        user_id: "me"
      }
    },
    suspense: true
  })
  const {data: group} = $apiMembers.useQuery("get", "/groups/{id}", {
    params: {
      path: {
        id: user?.group_id ?? ""
      }
    },
    suspense: true,
    enabled: !!user
  })

  // 認証状態を確認
  useEffect(() => {
    (async () => {
      const tokens = await getTokensMembers()
      if (tokens) {
        setAuthenticated(true);
      } else {
        setAuthenticated(false);
      }
    })();
  }, []);

  useEffect(() => {
    if (authenticated === false) {
      router.replace('../'); // 認証されていなければリダイレクト
    }
  }, [authenticated, router]);

  if (authenticated === null) {
    return <LoadingScreen/>;
  }

  return (
    <>
      <main className="content">
        {
          user && group &&
                <UserInfoCard user={user} group={group}/>
        }
        <Heading1 emoji={"🔔"}>
          実行委員会からのお知らせ
        </Heading1>
        <ViewNotifications client={$apiMembers}/>
        <Heading1 emoji={"📄"}>
          企画情報
        </Heading1>
        {
          exhibitor &&
                <ExhibitorCard
                        exhibitor={exhibitor}
                        openModal={() => setIsModalOpen(true)}
                />
        }

        {
          user && exhibitor &&
                <EditModal
                        user={user}
                        modal={isModalOpen}
                        setModal={setIsModalOpen}
                />
        }
      </main>
    </>
  );
}