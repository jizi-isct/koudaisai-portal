import { $api } from '@/features/api/api';
import { type GroupRead } from '@koudaisai/shared-types';
import { LoadingScreen } from '@koudaisai/shared-ui';
import { useState, useEffect } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { formatDate } from '@koudaisai/shared-utils';
import {
  Descriptions,
  Tag,
  Flex,
  Button,
  Result,
  type DescriptionsProps,
} from 'antd';

export function ViewGroupInfoPage() {
  const [queryClient] = useState(() => new QueryClient());
  const [groupId, setGroupId] = useState<string | null>();

  useEffect(() => {
    setGroupId(new URLSearchParams(window.location.search).get('group_id'));
  }, []);

  if (groupId === undefined) {
    return <LoadingScreen />;
  }

  if (!groupId) {
    return (
      <Result
        status="error"
        title="クエリパラメータに不足があります"
        subTitle="団体IDが指定されていません。URLに?group_id=xxxxのように指定してください。"
        extra={
          <Button href="/manage-groups/" type="primary">
            戻る
          </Button>
        }
      />
    );
  }

  return (
    <QueryClientProvider client={queryClient}>
      <GroupInfo groupId={groupId} />
    </QueryClientProvider>
  );
}

function GroupInfo({ groupId }: { groupId: string }) {
  const { data: groupInfo, isLoading: isLoadingGruop } = $api.useQuery(
    'get',
    '/groups/{id}',
    {
      params: {
        path: {
          id: groupId,
        },
      },
    },
  );
  const { data: memberInfo, isLoading: isLoadingMember } = $api.useQuery(
    'get',
    '/groups/{id}/members',
    {
      params: {
        path: {
          id: groupId,
        },
      },
    },
  );

  const filteredMemberByRole = {
    representative:
      memberInfo?.find((member) => member.role === 'representative')?.user_id ??
      '',
    operator:
      memberInfo?.find((member) => member.role === 'operator')?.user_id ?? '',
    first_responsible:
      memberInfo?.find((member) => member.role === 'first_responsible')
        ?.user_id ?? '',
    second_responsible:
      memberInfo?.find((member) => member.role === 'second_responsible')
        ?.user_id ?? '',
    third_responsible:
      memberInfo?.find((member) => member.role === 'third_responsible')
        ?.user_id ?? '',
  };

  const { data: representativeInfo, isLoading: isLoadingRepresentative } =
    $api.useQuery(
      'get',
      '/users/{id}',
      {
        params: {
          path: {
            id: filteredMemberByRole.representative,
          },
        },
      },
      { enabled: Boolean(filteredMemberByRole.representative) },
    );

  const { data: operatorInfo, isLoading: isLoadingOperator } = $api.useQuery(
    'get',
    '/users/{id}',
    {
      params: {
        path: {
          id: filteredMemberByRole.operator,
        },
      },
    },
    { enabled: Boolean(filteredMemberByRole.operator) },
  );

  const { data: firstResponsibleInfo, isLoading: isLoadingFirstResponsible } =
    $api.useQuery(
      'get',
      '/users/{id}',
      {
        params: {
          path: {
            id: filteredMemberByRole.first_responsible,
          },
        },
      },
      { enabled: Boolean(filteredMemberByRole.first_responsible) },
    );

  const { data: secondResponsibleInfo, isLoading: isLoadingSecondResponsible } =
    $api.useQuery(
      'get',
      '/users/{id}',
      {
        params: {
          path: {
            id: filteredMemberByRole.second_responsible,
          },
        },
      },
      { enabled: Boolean(filteredMemberByRole.second_responsible) },
    );

  const { data: thirdResponsibleInfo, isLoading: isLoadingThirdResponsible } =
    $api.useQuery(
      'get',
      '/users/{id}',
      {
        params: {
          path: {
            id: filteredMemberByRole.third_responsible,
          },
        },
      },
      { enabled: Boolean(filteredMemberByRole.third_responsible) },
    );

  if (
    isLoadingGruop ||
    isLoadingMember ||
    isLoadingRepresentative ||
    isLoadingOperator ||
    isLoadingFirstResponsible ||
    isLoadingSecondResponsible ||
    isLoadingThirdResponsible
  ) {
    return <LoadingScreen />;
  }

  if (!groupInfo) {
    return (
      <Result
        status="error"
        title="団体情報の取得に失敗しました"
        subTitle="団体が存在しないか、通信エラーにより団体情報を取得できませんでした。再読み込みしてください。"
        extra={
          <>
            <Button href="/manage-groups/" type="default">
              戻る
            </Button>
            <Button
              href={`/manage-groups/view?group_id=${groupId}`}
              type="primary"
            >
              再読み込み
            </Button>
          </>
        }
      />
    );
  }

  if (!memberInfo) {
    return (
      <Result
        status="error"
        title="団体メンバー情報の取得に失敗しました"
        subTitle="通信エラーによりメンバー情報を取得できませんでした。再読み込みしてください。"
        extra={
          <>
            <Button href="/manage-groups/" type="default">
              戻る
            </Button>
            <Button
              href={`/manage-groups/view?group_id=${groupId}`}
              type="primary"
            >
              再読み込み
            </Button>
          </>
        }
      />
    );
  }

  const filteredMemberNameByRole = {
    representative: !representativeInfo ? (
      'まだ紐づいていません'
    ) : (
      <a
        style={{ textDecoration: 'underline' }}
        href={`/manage-users/view?user_id=${representativeInfo.id}`}
      >
        {representativeInfo.name + `(${representativeInfo.m_address})`}
      </a>
    ),
    operator: !operatorInfo ? (
      'まだ紐づいていません'
    ) : (
      <a
        style={{ textDecoration: 'underline' }}
        href={`/manage-users/view?user_id=${operatorInfo.id}`}
      >
        {operatorInfo.name + `(${operatorInfo.m_address})`}
      </a>
    ),
    first_responsible: !firstResponsibleInfo ? (
      'まだ紐づいていません'
    ) : (
      <a
        style={{ textDecoration: 'underline' }}
        href={`/manage-users/view?user_id=${firstResponsibleInfo.id}`}
      >
        {firstResponsibleInfo.name + `(${firstResponsibleInfo.m_address})`}
      </a>
    ),
    second_responsible: !secondResponsibleInfo ? (
      'まだ紐づいていません'
    ) : (
      <a
        style={{ textDecoration: 'underline' }}
        href={`/manage-users/view?user_id=${secondResponsibleInfo.id}`}
      >
        {secondResponsibleInfo.name + `(${secondResponsibleInfo.m_address})`}
      </a>
    ),
    third_responsible: !thirdResponsibleInfo ? (
      'まだ紐づいていません'
    ) : (
      <a
        style={{ textDecoration: 'underline' }}
        href={`/manage-users/view?user_id=${thirdResponsibleInfo.id}`}
      >
        {thirdResponsibleInfo.name + `(${thirdResponsibleInfo.m_address})`}
      </a>
    ),
  };

  const typeOfGroups = (groupType: GroupRead['type']) => {
    switch (groupType) {
      case 'booth_project':
        return <Tag color="green">模擬店</Tag>;
      case 'stage_project':
        return <Tag color="yellow">ステージ</Tag>;
      case 'general_project':
        return <Tag color="blue">一般</Tag>;
      case 'lab_project':
        return <Tag color="orange">研究室</Tag>;
      case 'press':
        return <Tag color="pink">学内取材</Tag>;
      default:
        return <Tag color="gray">不明</Tag>;
    }
  };

  let groupInfoData: DescriptionsProps['items'];
  if (groupInfo.type === 'lab_project' || groupInfo.type === 'press') {
    groupInfoData = [
      {
        key: 'id',
        label: '団体ID',
        children: groupInfo.id,
      },
      {
        key: 'type',
        label: '団体種別',
        children: typeOfGroups(groupInfo.type),
      },
      {
        key: 'representative',
        label: '企画責任者',
        children: filteredMemberNameByRole.representative,
      },
      {
        key: 'operator',
        label: '企画実施担当者',
        children: filteredMemberNameByRole.operator,
      },
      {
        key: 'created_at',
        label: '作成日時',
        children: formatDate(groupInfo.created_at),
      },
      {
        key: 'updated_at',
        label: '更新日時',
        children: formatDate(groupInfo.updated_at),
      },
    ];
  } else {
    groupInfoData = [
      {
        key: 'id',
        label: '団体ID',
        children: groupInfo.id,
      },
      {
        key: 'type',
        label: '団体種別',
        children: typeOfGroups(groupInfo.type),
      },
      {
        key: 'first_responsible',
        label: '第1責任者',
        children: filteredMemberNameByRole.first_responsible,
      },
      {
        key: 'second_responsible',
        label: '第2責任者',
        children: filteredMemberNameByRole.second_responsible,
      },
      {
        key: 'third_responsible',
        label: '第3責任者',
        children: filteredMemberNameByRole.third_responsible,
      },
      {
        key: 'created_at',
        label: '作成日時',
        children: formatDate(groupInfo.created_at),
      },
      {
        key: 'updated_at',
        label: '更新日時',
        children: formatDate(groupInfo.updated_at),
      },
    ];
  }
  return (
    <Flex gap={8} vertical>
      <Descriptions
        title={groupInfo.name}
        column={1}
        bordered
        items={groupInfoData}
      />
    </Flex>
  );
}
