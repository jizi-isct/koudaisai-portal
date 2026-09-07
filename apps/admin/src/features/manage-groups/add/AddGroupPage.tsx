import { useState, type ChangeEvent } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Alert, Button, Flex, Form, Input, message, Select } from 'antd';
import type { GroupRead, Role } from '@koudaisai/shared-types';
import { getRepresentativeLabel } from '@koudaisai/shared-utils';
import { $api, api } from '@/features/api/api';
import {
  canonicalizeGroupId,
  groupIdPattern,
  groupTypeLabels,
  inferTypeFromGroupId,
  requiresDistinctMembers,
  rolesByGroupType,
} from '../group';

type FormValues = {
  id: string;
  name: string;
  type: GroupRead['type'];
  members?: Partial<Record<Role, string>>;
};

type MemberFailure = {
  role: Role;
  roleLabel: string;
  userLabel: string;
  error: string;
};

const groupTypeOptions = Object.entries(groupTypeLabels).map(
  ([value, label]) => ({ value: value as GroupRead['type'], label }),
);

const authErrorMessage = (status: number) => {
  switch (status) {
    case 401:
      return 'ログインの有効期限が切れています。再度ログインしてください';
    case 403:
      return 'この操作を行う権限がありません';
    default:
      return undefined;
  }
};

const groupErrorMessage = (status: number) => {
  switch (status) {
    case 400:
      return '団体IDまたは団体名が不正です';
    default:
      return (
        authErrorMessage(status) ??
        `サーバーエラーが発生しました (HTTP ${status})`
      );
  }
};

const memberErrorMessage = (status: number) => {
  switch (status) {
    case 404:
      return '団体IDの形式が不正です';
    case 422:
      return '参加団体またはユーザーが見つからないか、この団体種別では指定できない役割です';
    default:
      return (
        authErrorMessage(status) ??
        `サーバーエラーが発生しました (HTTP ${status})`
      );
  }
};

const errorText = (error: unknown) =>
  error instanceof Error ? error.message : String(error);

export function AddGroupPage() {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <AddGroupForm />
    </QueryClientProvider>
  );
}

function AddGroupForm() {
  const [form] = Form.useForm<FormValues>();
  const [messageApi, contextHolder] = message.useMessage();
  const [submitting, setSubmitting] = useState(false);
  const [memberFailures, setMemberFailures] = useState<MemberFailure[]>([]);

  const { data: users, isLoading: isLoadingUsers } = $api.useQuery(
    'get',
    '/users',
  );
  const userOptions =
    users?.map((user) => ({
      value: user.id,
      label: `${user.name}(${user.m_address})`,
    })) ?? [];
  const userLabelById = new Map(
    userOptions.map((option) => [option.value, option.label]),
  );

  const { data: groups } = $api.useQuery('get', '/groups');
  const existingGroupIds = groups && new Set(groups.map((group) => group.id));

  const groupType = Form.useWatch('type', form);

  const handleTypeChange = () => {
    form.setFieldValue('members', undefined);
  };

  const handleIdChange = (event: ChangeEvent<HTMLInputElement>) => {
    const inferredType = inferTypeFromGroupId(event.target.value);
    if (inferredType && inferredType !== form.getFieldValue('type')) {
      form.setFieldValue('type', inferredType);
      form.setFieldValue('members', undefined);
      void form.validateFields(['type']);
    }
  };

  const handleSubmit = async (values: FormValues) => {
    setSubmitting(true);
    setMemberFailures([]);

    const groupId = canonicalizeGroupId(values.id);

    try {
      const { response } = await api.GET('/groups/{id}', {
        params: { path: { id: groupId } },
      });
      if (response.ok) {
        setSubmitting(false);
        messageApi.error(`団体ID ${groupId} はすでに使用されています`);
        return;
      }
      if (response.status !== 404) {
        throw new Error(groupErrorMessage(response.status));
      }
    } catch (error) {
      setSubmitting(false);
      messageApi.error(`団体IDの重複確認に失敗しました: ${errorText(error)}`);
      return;
    }

    try {
      const { response } = await api.PUT('/groups/{id}', {
        params: { path: { id: groupId } },
        body: { name: values.name.trim(), type: values.type },
      });
      if (!response.ok) {
        throw new Error(groupErrorMessage(response.status));
      }
    } catch (error) {
      setSubmitting(false);
      messageApi.error(`参加団体の作成に失敗しました: ${errorText(error)}`);
      return;
    }

    const membersToAssign = rolesByGroupType[values.type]
      .map((role) => ({ role, userId: values.members?.[role] }))
      .filter((member): member is { role: Role; userId: string } =>
        Boolean(member.userId),
      );

    const failures: MemberFailure[] = [];
    for (const member of membersToAssign) {
      try {
        const { response } = await api.PUT('/groups/{id}/members/{role}', {
          params: { path: { id: groupId, role: member.role } },
          body: { user_id: member.userId },
        });
        if (!response.ok) {
          throw new Error(memberErrorMessage(response.status));
        }
      } catch (error) {
        failures.push({
          role: member.role,
          roleLabel: getRepresentativeLabel(member.role, values.type),
          userLabel: userLabelById.get(member.userId) ?? member.userId,
          error: errorText(error),
        });
      }
    }

    setSubmitting(false);

    if (failures.length > 0) {
      setMemberFailures(failures);
      messageApi.warning(
        '参加団体は作成しましたが、一部のユーザーの紐づけに失敗しました',
      );
      return;
    }

    messageApi.success('参加団体を作成しました');
    window.location.href = `/manage-groups/view?group_id=${groupId}`;
  };

  return (
    <>
      <Form form={form} onFinish={handleSubmit}>
        <h1>参加団体を新規作成</h1>
        <Form.Item
          name="id"
          label="団体ID"
          extra="例: 模擬店 M-101 / ステージ S-101 / 一般 I-101 / 研究室 L-101 / 学内取材 P-101"
          normalize={(value: string | undefined) => value?.toUpperCase()}
          rules={[
            { required: true, message: '団体IDを入力してください' },
            {
              pattern: groupIdPattern,
              message:
                '団体IDは「M / S / I / L / P のいずれか」と「999以下の数字」を「-」でつないだ形式で入力してください',
            },
            {
              validator(_rule, value: string | undefined) {
                if (!value || !existingGroupIds) {
                  return Promise.resolve();
                }
                return existingGroupIds.has(canonicalizeGroupId(value))
                  ? Promise.reject(
                      new Error('この団体IDはすでに使用されています'),
                    )
                  : Promise.resolve();
              },
            },
          ]}
        >
          <Input
            placeholder="団体IDを入力してください"
            onChange={handleIdChange}
          />
        </Form.Item>

        <Form.Item
          name="name"
          label="団体名"
          rules={[
            {
              required: true,
              whitespace: true,
              message: '団体名を入力してください',
            },
          ]}
        >
          <Input placeholder="団体名を入力してください" />
        </Form.Item>

        <Form.Item
          name="type"
          label="団体種別"
          dependencies={['id']}
          rules={[
            { required: true, message: '団体種別を選択してください' },
            ({ getFieldValue }) => ({
              validator(_rule, value: GroupRead['type'] | undefined) {
                const inferredType = inferTypeFromGroupId(
                  getFieldValue('id') as string | undefined,
                );
                if (!value || !inferredType || inferredType === value) {
                  return Promise.resolve();
                }
                return Promise.reject(
                  new Error(`団体IDと団体種別が一致していません`),
                );
              },
            }),
          ]}
        >
          <Select
            options={groupTypeOptions}
            onChange={handleTypeChange}
            placeholder="団体種別を選択してください"
          />
        </Form.Item>

        {groupType &&
          rolesByGroupType[groupType].map((role) => (
            <Form.Item
              key={role}
              name={['members', role]}
              label={`${getRepresentativeLabel(role, groupType)}（任意）`}
              dependencies={rolesByGroupType[groupType]
                .filter((other) => other !== role)
                .map((other) => ['members', other])}
              rules={[
                ({ getFieldValue }) => ({
                  validator(_rule, value: string | undefined) {
                    const type = getFieldValue('type') as
                      | GroupRead['type']
                      | undefined;
                    if (!value || !type || !requiresDistinctMembers(type)) {
                      return Promise.resolve();
                    }
                    const members = (getFieldValue('members') ?? {}) as Partial<
                      Record<Role, string>
                    >;
                    const duplicated = rolesByGroupType[type].some(
                      (other) => other !== role && members[other] === value,
                    );
                    return duplicated
                      ? Promise.reject(
                          new Error(
                            '各責任者にはそれぞれ別のユーザーを指定してください',
                          ),
                        )
                      : Promise.resolve();
                  },
                }),
              ]}
            >
              <Select
                options={userOptions}
                loading={isLoadingUsers}
                showSearch={{
                  filterOption: (input, option) =>
                    (option?.label ?? '')
                      .toLowerCase()
                      .includes(input.toLowerCase()),
                }}
                allowClear
                placeholder="ユーザーを選択してください"
              />
            </Form.Item>
          ))}

        <Form.Item>
          <Flex gap={8}>
            <Button type="primary" htmlType="submit" loading={submitting}>
              作成
            </Button>
            <Button type="default" href="/manage-groups/">
              戻る
            </Button>
          </Flex>
        </Form.Item>
      </Form>

      {memberFailures.length > 0 && (
        <Alert
          type="warning"
          showIcon
          title="ユーザーの紐づけに失敗しました"
          description={
            <Flex vertical gap={4}>
              {memberFailures.map((failure) => (
                <span key={failure.role}>
                  {failure.roleLabel}（{failure.userLabel}）: {failure.error}
                </span>
              ))}
            </Flex>
          }
          style={{ marginTop: 16 }}
        />
      )}

      {contextHolder}
    </>
  );
}
