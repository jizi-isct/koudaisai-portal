'use client'; // クライアントサイドコンポーネントとして実行するために追加

import {useEffect, useState} from 'react';
import {useRouter} from 'next/navigation';
import {$apiMembers, Exhibitor, getExhibitor, getTokensMembers, getUser, User} from "@/lib";
import "../../globals.css";

import {Heading1, LoadingScreen} from '@/components/generic';
import {UserInfoCard} from "@/components/UserInfoCard/UserInfoCard";
import {ViewNotifications} from "@/components/notification/ViewNotifications";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {ExhibitorCard} from "@/components/exhibitor/ExhibitorCard";
import {EditModal} from "@/components/exhibitor/EditModal/EditModal";

export default function Page() {
  return (
    <QueryClientProvider client={new QueryClient()}>
      <Inner/>
    </QueryClientProvider>
  )
}

function Inner() {
  const router = useRouter();
  const [authenticated, setAuthenticated] = useState<boolean | null>(null);
  const [user, setUser] = useState<User | null>(null);
  const [exhibitor, setExhibitor] = useState<Exhibitor | null>(null)
  const [isModalOpen, setIsModalOpen] = useState(false);

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

  useEffect(() => {
    if (user?.exhibition_id) {
      getExhibitor(user.exhibition_id).then(setExhibitor);
    }
  }, [user]);

  useEffect(() => {
    getUser().then(setUser);
  }, []);

  if (authenticated === null) {
    return <LoadingScreen/>;
  }

  return (
    <>
      <main className="content">
        {
          user && exhibitor &&
                <UserInfoCard user={user} exhibitor={exhibitor}/>
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