import { $api } from "../api/api";
import { LoadingScreen } from "@koudaisai/shared-ui";
import { useState, useEffect } from "react";
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Descriptions, Tag, Flex, Button, Result, type DescriptionsProps } from "antd";
import formatDate from "./modules/formatDate";


export function ViewUserInfoPage(){
    const [queryClient] = useState(() => new QueryClient());
    const [userId, setUserId] = useState<string | null>();

    useEffect(() => {
        setUserId(
        new URLSearchParams(window.location.search).get('user_id'),
        );
    }, []);

    if (userId === undefined) {return <LoadingScreen />;};

    if (!userId) {
        return (
            <Result
                status="error"
                title="クエリパラメータに不足があります"
                subTitle="ユーザーIDが指定されていません。URLに?user_id=xxxxのように指定してください。"
                extra={
                <Button href="/manageUsers" type="primary">
                    戻る
                </Button>
                }
            />
        );
    };

    return(
        <QueryClientProvider client={queryClient}>
            <UserInfo userId={userId} />
        </QueryClientProvider>
    );
};

const roleNames = {
    representative: "企画責任者",
    operator: "企画実施担当者",
    first_responsible: "第1責任者",
    second_responsible: "第2責任者",
    third_responsible: "第3責任者",
    noRole: "このユーザーはまだグループと紐づいていません",
    error: "ユーザーの役割の取得に失敗しました"
}

function UserInfo({userId}: {userId: string}){
    const { data: userInfo, isLoading: isLoadingUsers } = $api.useQuery(
        'get',
        '/users/{id}',
        {
            params: {
                path: {
                    id: userId,
                },
            },
        },
    );

    const {data: groupData, isLoading: isLoadingGroups} = $api.useQuery(
        'get',
        '/groups/{id}',
        {
            params: {
                path: {
                    id: userInfo?.group_id ?? ''
                }
            }
        },
        { enabled: Boolean(userInfo && userInfo.group_id)}
    )

    const {data: groupMember, isLoading: isLoadingMember} = $api.useQuery(
        'get',
        '/groups/{id}/members',
        {
            params: {
                path: {
                    id: userInfo?.group_id ?? ''
                }
            }
        },
        { enabled: Boolean(userInfo && userInfo.group_id)}
    )

    if(isLoadingUsers || isLoadingGroups || isLoadingMember){return <LoadingScreen />;};

    if (userInfo===undefined){
        return (
            <Result 
                status="error"
                title="ユーザー情報の取得に失敗しました"
                subTitle="userInfoがundfinedです"
                extra={
                    <>
                    <Button href="/manageUsers/" type="default">戻る</Button>
                    <Button href={`/manageUsers/view?user_id=${userId}`} type="primary">再読み込み</Button>
                    </>
                }
            />
        );
    }  

    const userStatus = () => {
        if (userInfo.status !== "deactivated"){
            return( 
                userInfo.status === "active"
                ? <Tag color="green">有効化済み</Tag>
                : <Tag color="blue">登録済み</Tag>
            );
        } else {
            return (
                <Tag color="gray">無効化済み</Tag>
            );
        };
    };

    const groupInfo = () => {
        if(!userInfo.group_id){
            return ({
                id: "このユーザーはまだグループと紐づいていません",
                name: "このユーザーはまだグループと紐づいていません"
            });
        } else {
            return (
                !groupData 
                ? {
                    id: "グループ情報の取得に失敗しました",
                    name: "グループ情報の取得に失敗しました"
                }
                : {
                    id: groupData.id,
                    name: groupData.name
                });
        };
    };

    const userRole = () => {
        if(!groupMember){
            return 'noRole';
        } else {
            const targetUserRole = (groupMember).find((member) => member.user_id === userId);
            return(
                !targetUserRole
                ? 'error'
                : targetUserRole.role
            );
        };
    };

    const userInfoData: DescriptionsProps['items'] = [
        {
            key: "id",
            label: "ユーザーID",
            children: userInfo.id,
        },
        {
            key: "m_address",
            label: "メールアドレス",
            children: userInfo.m_address,
        },
        {
            key: "status",
            label: "状態",
            children: userStatus(),
        },
        {
            key: "groupsId",
            label: "所属グループID",
            children: groupInfo().id,
        },
        {
            key: "groupsName",
            label: "所属団体名",
            children: groupInfo().name
        },
        {
            key: "role",
            label: "役割",
            children: roleNames[userRole()]
        },
        {
            key: "created_at",
            label: "作成日時",
            children: formatDate(userInfo.created_at)
        },
        {
            key: "updated_at",
            label: "更新日時",
            children: formatDate(userInfo.updated_at)
        }
    ];

    return(
        <Flex gap={8} vertical>
            <Descriptions title={userInfo.name} column={1} bordered items={userInfoData} />
        </Flex>
    );
};
