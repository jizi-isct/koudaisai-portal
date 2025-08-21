'use client'; // クライアントサイドコンポーネントとして実行するために追加

import {Suspense, useEffect, useState} from 'react';
import {useRouter} from 'next/navigation';
import {$apiMembers, getTokensMembers, GroupRead, UserRead} from "@/lib";
import "../../globals.css";
import {Heading1, LoadingScreen} from '@/components/generic';
import {UserInfoCard} from "@/components/UserInfoCard/UserInfoCard";
import {ViewNotifications} from "@/components/notification/ViewNotifications";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {PlanCard} from "@/components/group/PlanCard";
import {EditModal} from "@/components/group/EditModal/EditModal";
import {$plansInfoApiNoLogin} from "@/lib/plansInfoApi";
import {BasePlanRead} from "@/lib/plansInfoTypes";

export default function Page() {
  return (
    <Suspense fallback={<LoadingScreen/>}>
      <QueryClientProvider client={new QueryClient()}>
        <Inner1/>
      </QueryClientProvider>
    </Suspense>
  )
}

function Inner1() {
  const {data: user} = $apiMembers.useQuery("get", "/users/{user_id}", {
    params: {
      path: {
        user_id: "me"
      }
    },
    suspense: true
  })
  if (user) {
    return <Inner2 user={user}/>
  } else {
    return <LoadingScreen/>
  }
}

function Inner2({user}: { user: UserRead }) {
  const router = useRouter();
  const [authenticated, setAuthenticated] = useState<boolean | null>(null);
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

  if (group) {
    return <main className="content">
      {
        user && group &&
              <UserInfoCard user={user} group={group}/>
      }
      <Heading1 emoji={"🔔"}>
        実行委員会からのお知らせ
      </Heading1>
      <ViewNotifications client={$apiMembers}/>
      {group.type_plan && <Inner3 user={user} group={group}/>}
    </main>
  } else {
    return <LoadingScreen/>
  }
}

function Inner3({group}: { user: UserRead, group: GroupRead }) {
  const {data: plan} = $plansInfoApiNoLogin.useQuery("get", "/plans/{planId}", {
    params: {
      path: {
        planId: group.id
      }
    }
  })
  const [isModalOpen, setIsModalOpen] = useState(false);


  return (
    <>
      <Heading1 emoji={"📄"}>
        企画情報
      </Heading1>
      {
        plan ?
          <PlanCard
            plan={plan as BasePlanRead}
            openModal={() => setIsModalOpen(true)}
          />
          :
          <LoadingScreen/>
      }

      {
        plan && <EditModal
                      modal={isModalOpen}
                      setModal={setIsModalOpen}
                      initPlanName={plan.plan_name}
                      initDescription={plan.description}
                      initIsChildFriendly={plan.is_child_friendly}
              />
      }
    </>
  );
}