import {
  BellOutlined,
  BookOutlined,
  FormOutlined,
  HomeOutlined,
  InfoOutlined,
  MenuFoldOutlined,
  MenuUnfoldOutlined,
  DatabaseOutlined,
} from '@ant-design/icons';
import { getTokensAdmin } from '@koudaisai/shared-auth-admin';
import { LoadingScreen } from '@koudaisai/shared-ui';
import { StyleProvider } from '@ant-design/cssinjs';
import { Button, ConfigProvider, Flex, Layout, Menu, theme } from 'antd';
import type { ReactNode } from 'react';
import { useEffect, useState } from 'react';

type Props = {
  children: ReactNode;
  currentPath: string;
};

const { Content, Header, Sider } = Layout;

const antdTheme = {
  token: {
    colorPrimary: '#0048FF',
    borderRadius: 8,
  },
  components: {
    Button: {
      contentFontSize: 12,
      paddingInline: 17,
    },
  },
};

function AdminLayoutContent({ children, currentPath }: Props) {
  const [collapsed, setCollapsed] = useState(false);
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const {
    token: { colorBgContainer, borderRadiusLG },
  } = theme.useToken();

  const selectedKeys = [
    currentPath.endsWith('/') ? currentPath : `${currentPath}/`,
  ];

  useEffect(() => {
    (async () => {
      const tokens = getTokensAdmin();

      if (tokens) {
        setIsAuthenticated(true);
        return;
      }

      window.location.assign('/login');
    })().catch(() => {
      window.location.assign('/login');
    });
  }, []);

  return (
    <StyleProvider>
      <Layout style={{ minHeight: '100vh', width: '100vw' }}>
        <Sider
          collapsible
          collapsed={collapsed}
          trigger={null}
          style={{ margin: 0, padding: 0 }}
        >
          <a href="/" style={{ color: 'white', display: 'block', padding: 10 }}>
            <Flex align="center" gap={10} justify="center">
              <img
                src="/logo.jpg"
                alt="Koudaisai Portal Admin Site Logo"
                width={32}
                height={32}
              />
              {!collapsed && <span>工大祭管理サイト</span>}
            </Flex>
          </a>
          <Menu
            theme="dark"
            mode="inline"
            selectedKeys={selectedKeys}
            style={{ position: 'sticky', top: 0 }}
            items={[
              {
                type: 'group',
                label: '工大祭ポータル',
                key: 'koudaisai-portal',
                children: [
                  {
                    key: '/',
                    icon: <HomeOutlined />,
                    label: <a href="/">ホーム</a>,
                  },
                  {
                    key: '/forms/',
                    icon: <FormOutlined />,
                    label: <a href="/forms/">フォーム</a>,
                  },
                  {
                    key: '/documents/',
                    icon: <BookOutlined />,
                    label: <a href="/documents/">資料</a>,
                  },
                  {
                    key: '/notifications/',
                    icon: <BellOutlined />,
                    label: <a href="/notifications/">通知</a>,
                  },
                  {
                    key: '/approval_requests/',
                    icon: <BellOutlined />,
                    label: <a href="/approval_requests/">承認申請</a>,
                  },
                  {
                    key: '/manage_users/',
                    icon: <DatabaseOutlined />,
                    label: <a href="/kebab-case/">ユーザー管理</a>,
                  },
                ],
              },
              {
                type: 'group',
                label: 'その他',
                key: 'others',
                children: [
                  {
                    key: '/plans_info/',
                    icon: <InfoOutlined />,
                    label: <a href="/plans_info/">参加団体情報</a>,
                  },
                ],
              },
            ]}
          />
        </Sider>
        <Layout>
          <Header style={{ padding: 0, background: colorBgContainer }}>
            <Flex>
              <Button
                type="text"
                icon={collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />}
                onClick={() => setCollapsed(!collapsed)}
                style={{
                  fontSize: '16px',
                  width: 64,
                  height: 64,
                }}
              />
            </Flex>
          </Header>
          <Content
            style={{
              margin: '24px 16px',
              padding: 24,
              background: colorBgContainer,
              borderRadius: borderRadiusLG,
              width: 'auto',
            }}
          >
            {isAuthenticated ? children : <LoadingScreen />}
          </Content>
        </Layout>
      </Layout>
    </StyleProvider>
  );
}

export function AdminShell({ children, currentPath }: Props) {
  return (
    <ConfigProvider theme={antdTheme}>
      <AdminLayoutContent currentPath={currentPath}>
        {children}
      </AdminLayoutContent>
    </ConfigProvider>
  );
}
