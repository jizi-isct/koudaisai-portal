'use client'; // クライアントサイドコンポーネントとして実行するために追加

import {Suspense, useEffect, useState} from 'react';
import {useRouter} from 'next/navigation';
import { $apiMembers } from '@/lib/api';
import { getTokensMembers } from '@koudaisai/shared-auth-members';
import { UserRead ,GroupRead } from '@koudaisai/shared-types';
import "../globals.css";
import {Heading1, LoadingScreen, Modal} from '@koudaisai/shared-ui';
import {UserInfoCard} from "@/components/UserInfoCard/UserInfoCard";
import {ViewNotifications} from "@/components/notification/ViewNotifications";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {PlanCard} from "@/components/plan/PlanCard";
import {$plansInfoApiNoLogin} from "@/lib/plansInfoApi";
import {BasePlanRead} from "@koudaisai/shared-types";
import {Button} from "antd";
import {EditAdditionalInfo} from "@/components/plan-details/additional-info/EditAdditionalInfo";
import {EditProducts} from "@/components/plan-details/products/EditProducts";
import { authFetchClient } from '@/lib/api';

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
      const tokens = await getTokensMembers(authFetchClient)
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
  const {data: details, isLoading: isLoadingDetails, refetch: refetchDetails} = $apiMembers.useQuery("get", "/groups/{id}/plan-details", {
    params: {
      path: {
        id: "us"
      }
    }
  })
  const {mutateAsync : putPlanDetails, isPending} = $apiMembers.useMutation("put", "/groups/{id}/plan-details")

  const [isProductsModalOpen, setIsProductsModalOpen] = useState(false);
  const [isAdditionalInfoModalOpen, setIsAdditionalInfoModalOpen] = useState(false);


  return (
    <>
      <Heading1 emoji={"📄"}>
        企画情報
      </Heading1>
      {
        plan ?
          <PlanCard
            plan={plan as BasePlanRead}
            openModal={() => {
              return
            }}
            disableEdit={true}
          />
          :
          <LoadingScreen/>
      }

      {/*{*/}
      {/*  plan && <EditModal*/}
      {/*                planId={plan.id}*/}
      {/*                modal={isModalOpen}*/}
      {/*                setModal={setisModalOpen}*/}
      {/*                initDescription={plan.description}*/}
      {/*        />*/}
      {/*}*/}

      <Heading1 emoji={"📱"}>
        企画詳細情報
      </Heading1>
      公式サイトやアプリに掲載される企画詳細情報をリアルタイムで編集することができます。
      {
        isLoadingDetails ? <LoadingScreen/> :
          <div style={{display: "flex", gap: 8, flexDirection: "column", paddingBottom: 32}}>
            <Button onClick={() => {setIsProductsModalOpen(true)}}>商品一覧を編集</Button>
            <Button onClick={() => {setIsAdditionalInfoModalOpen(true)}}>追加情報を編集</Button>
          </div>
      }
      <Modal isOpen={isProductsModalOpen} setOpen={setIsProductsModalOpen}>
        <EditProducts
          product={details?.product}
          updateProducts={async (product) => {
            console.log(product)
            await putPlanDetails({
              params: {
                path: {
                  id: "us"
                }
              },
              body: {
                product: product,
                additional_info: details?.additional_info
              }
            })
            await refetchDetails()
          }}
          isLoading={isPending}
        />
      </Modal>
      <Modal isOpen={isAdditionalInfoModalOpen} setOpen={setIsAdditionalInfoModalOpen}>
        <EditAdditionalInfo
          additionalInfo={details?.additional_info ?? ""}
          updateAdditionalInfo={async (additionalInfo) => {
            await putPlanDetails({
              params: {
                path: {
                  id: "us"
                }
              },
              body: {
                product: details?.product,
                additional_info: additionalInfo
              }
            })
            await refetchDetails()
          }}
          isLoading={isPending}
        />
      </Modal>
    </>
  );
}