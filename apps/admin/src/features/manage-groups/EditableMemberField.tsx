import { $api, api } from '@/features/api/api';
import { type Role } from '@koudaisai/shared-types';
import { useState, type ReactNode } from 'react';
import { useQuery } from '@tanstack/react-query';
import { Flex, Button, Select } from 'antd';
import type { MessageInstance } from 'antd/es/message/interface';

const roleLabels: Record<Role, string> = {
  representative: '企画責任者',
  operator: '企画実施担当者',
  first_responsible: '第1責任者',
  second_responsible: '第2責任者',
  third_responsible: '第3責任者',
};

export function EditableMemberField({
  groupId,
  role,
  displayNode,
  currentUserId,
  messageApi,
  onSaved,
}: {
  groupId: string;
  role: Role;
  displayNode: ReactNode;
  currentUserId: string;
  messageApi: MessageInstance;
  onSaved: () => Promise<unknown>;
}) {
  const [isEditing, setIsEditing] = useState(false);
  const [selectedUserId, setSelectedUserId] = useState<string>('');
  const [isSavingMember, setIsSavingMember] = useState(false);

  const { data: allUsers, isLoading: isLoadingAllUsers } = $api.useQuery(
    'get',
    '/users',
    {},
    { enabled: isEditing },
  );
  const { data: allGroups, isLoading: isLoadingAllGroups } = $api.useQuery(
    'get',
    '/groups',
    {},
    { enabled: isEditing },
  );
  const { data: assignedUserIds, isLoading: isLoadingAssignedUserIds } =
    useQuery({
      queryKey: ['all-group-members', allGroups?.map((g) => g.id)],
      queryFn: async () => {
        const results = await Promise.all(
          (allGroups ?? []).map((g) =>
            api.GET('/groups/{id}/members', {
              params: { path: { id: g.id } },
            }),
          ),
        );
        const ids = new Set<string>();
        for (const result of results) {
          if (result.error) {
            throw result.error;
          }
          for (const member of result.data ?? []) {
            ids.add(member.user_id);
          }
        }
        return ids;
      enabled: isEditing && Boolean(allGroups),
    });

  const { mutateAsync: putMember } = $api.useMutation(
    'put',
    '/groups/{id}/members/{role}',
  );
  const { mutateAsync: deleteMember } = $api.useMutation(
    'delete',
    '/groups/{id}/members/{role}',
  );

  const roleLabel = roleLabels[role];

  const startEditing = () => {
    setIsEditing(true);
    setSelectedUserId(currentUserId);
  };

  const cancelEditing = () => {
    setIsEditing(false);
    setSelectedUserId('');
  };

  const handleSaveMember = async () => {
    setIsSavingMember(true);
    try {
      if (selectedUserId) {
        await putMember({
          params: { path: { id: groupId, role } },
          body: { user_id: selectedUserId },
        });
      } else {
        await deleteMember({
          params: { path: { id: groupId, role } },
        });
      }
      await onSaved();
      messageApi.success(`${roleLabel}を更新しました`);
      setIsSavingMember(false);
      cancelEditing();
    } catch (error) {
      messageApi.error(`${roleLabel}の更新に失敗しました: ${String(error)}`);
      setIsSavingMember(false);
    }
  };

  if (!isEditing) {
    return (
      <Flex gap={8} align="center" justify="space-between">
        <span>{displayNode}</span>
        <Button size="small" onClick={startEditing}>
          編集
        </Button>
      </Flex>
    );
  }

  const isLoadingOptions =
    isLoadingAllUsers || isLoadingAllGroups || isLoadingAssignedUserIds;
  const options = (allUsers ?? [])
    .filter(
      (user) => !assignedUserIds?.has(user.id) || user.id === currentUserId,
    )
    .map((user) => ({
      value: user.id,
      label: `${user.name}(${user.m_address})`,
    }));

  return (
    <Flex gap={8} align="start" wrap>
      <Select
        showSearch={{
          filterOption: (input, option) =>
            (option?.label ?? '').toLowerCase().includes(input.toLowerCase()),
        }}
        allowClear
        placeholder="ユーザーを選択"
        style={{ width: 'min(100%, 28rem)' }}
        value={selectedUserId || undefined}
        onChange={(value) => setSelectedUserId(value ?? '')}
        options={options}
        loading={isLoadingOptions}
        disabled={isLoadingOptions || isSavingMember}
        autoFocus
      />
      <Button
        type="primary"
        onClick={() => {
          void handleSaveMember();
        }}
        disabled={isLoadingOptions || isSavingMember}
        loading={isSavingMember}
      >
        保存
      </Button>
      <Button onClick={cancelEditing} disabled={isSavingMember}>
        キャンセル
      </Button>
    </Flex>
  );
}
