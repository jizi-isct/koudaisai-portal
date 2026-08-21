import { Heading1, LoadingScreen } from '@koudaisai/shared-ui';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Alert, Card, Flex, Switch, Typography, message } from 'antd';
import { useState } from 'react';
import { $api } from '@/features/api/api';

export default function SettingsPage() {
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <SettingsContent />
    </QueryClientProvider>
  );
}

function SettingsContent() {
  const [messageApi, contextHolder] = message.useMessage();
  const { data: settings, isLoading, error, refetch } = $api.useQuery(
    'get',
    '/settings',
  );
  const { mutateAsync: updateShowOccasions } = $api.useMutation(
    'patch',
    '/settings/show-occasions-on-portal',
  );
  const [isSaving, setIsSaving] = useState(false);

  const handleShowOccasionsChange = async (checked: boolean) => {
    setIsSaving(true);
    try {
      await updateShowOccasions({
        body: { show_occasions_on_portal: checked },
      });
      await refetch();
      messageApi.success('設定を保存しました。');
    } catch (caughtError) {
      messageApi.error(`設定の保存に失敗しました: ${String(caughtError)}`);
    } finally {
      setIsSaving(false);
    }
  };

  if (isLoading) return <LoadingScreen />;

  if (!settings) {
    return (
      <>
        {contextHolder}
        <Heading1 emoji="⚠️">設定を取得できませんでした</Heading1>
        <Alert type="error" showIcon message={String(error)} />
      </>
    );
  }

  return (
    <>
      {contextHolder}
      <Heading1 emoji="⚙️">設定</Heading1>
      <Card title="企画情報" style={{ maxWidth: 720 }}>
        <Flex align="center" justify="space-between" gap={24}>
          <div>
            <Typography.Text strong>企画実施予定を参加団体に表示</Typography.Text>
            <Typography.Paragraph type="secondary" style={{ margin: '4px 0 0' }}>
              有効にすると、参加団体ポータルの企画情報に実施日時と場所を表示します。
            </Typography.Paragraph>
          </div>
          <Switch
            checked={settings.show_occasions_on_portal}
            checkedChildren="表示"
            unCheckedChildren="非表示"
            loading={isSaving}
            onChange={handleShowOccasionsChange}
          />
        </Flex>
      </Card>
    </>
  );
}
