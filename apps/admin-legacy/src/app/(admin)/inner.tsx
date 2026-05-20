"use client";

import {
  BellOutlined,
  BookOutlined,
  FormOutlined,
  HomeOutlined,
  InfoOutlined,
  MenuFoldOutlined,
  MenuUnfoldOutlined,
} from "@ant-design/icons";
import type {Tokens} from "@koudaisai/shared-auth";
import {getTokensAdmin} from "@koudaisai/shared-auth-admin";
import {LoadingScreen} from "@koudaisai/shared-ui";
import {Button, Flex, Layout, Menu, theme} from "antd";
import {Content} from "antd/es/layout/layout";
import Sider from "antd/es/layout/Sider";
import {Header} from "antd/lib/layout/layout";
import {ReactNode, useEffect, useState} from "react";
import {authFetchClient} from "@/lib/api";
import {getCurrentPathname, navigateTo} from "@/lib/browserNavigation";

type Props = {
  children: ReactNode;
};

function useWindowWidth() {
  const [width, setWidth] = useState(() => typeof window === "undefined" ? 10000 : window.innerWidth);

  useEffect(() => {
    const handleResize = () => setWidth(window.innerWidth);
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return width;
}

export function Inner({children}: Props) {
  const [tokens, setTokens] = useState<Tokens | null | undefined>();
  useEffect(() => {
    (async () => {
      const t = await getTokensAdmin(authFetchClient);
      if (t) {
        setTokens(t);
      } else {
        window.location.assign(process.env.NEXT_PUBLIC_AUTH_BASE_URL + "/admin/login");
      }
    })();
  }, []);

  const width = useWindowWidth() ?? 10000;
  const [collapsed, setCollapsed] = useState(width < 768);

  const {
    token: {colorBgContainer, borderRadiusLG},
  } = theme.useToken();

  const pathname = getCurrentPathname();

  return (
    <Layout style={{minHeight: "100vh", width: "100vw"}}>
      <Sider collapsible collapsed={collapsed} style={{margin: 0, padding: 0}}>
        <a href="/" style={{color: "white", padding: 10}}>
          <Flex align="center" gap={10} justify="center">
            <img
              src="/admin_logo.jpg"
              alt="Koudaisai Portal Admin Site Logo"
              width={32}
              height={32}
            />
            {<span>工大祭管理サイト</span>}
          </Flex>
        </a>
        <Menu
          theme="dark"
          mode="inline"
          selectedKeys={[pathname]}
          style={{position: "sticky", top: 0}}
          items={[
            {
              type: "group",
              label: "工大祭ポータル",
              key: "koudaisai-portal",
              children: [
                {
                  key: "/",
                  icon: <HomeOutlined />,
                  label: "ホーム",
                  onClick: () => navigateTo("/"),
                },
                {
                  key: "/forms/",
                  icon: <FormOutlined />,
                  label: "フォーム",
                  onClick: () => navigateTo("/forms/"),
                },
                {
                  key: "/documents/",
                  icon: <BookOutlined />,
                  label: "資料",
                  onClick: () => navigateTo("/documents/"),
                },
                {
                  key: "/notifications/",
                  icon: <BellOutlined />,
                  label: "通知",
                  onClick: () => navigateTo("/notifications/"),
                },
                {
                  key: "/approval_requests/",
                  icon: <BellOutlined />,
                  label: "承認申請",
                  onClick: () => navigateTo("/approval_requests/"),
                },
              ],
            },
            {
              type: "group",
              label: "その他",
              key: "others",
              children: [
                {
                  key: "/plans_info/",
                  icon: <InfoOutlined />,
                  label: "参加団体情報",
                  onClick: () => navigateTo("/plans_info/"),
                },
              ],
            },
          ]}
        />
      </Sider>
      <Layout>
        <Header style={{padding: 0, background: colorBgContainer}}>
          <Flex>
            <Button
              type="text"
              icon={collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />}
              onClick={() => setCollapsed(!collapsed)}
              style={{
                fontSize: "16px",
                width: 64,
                height: 64,
              }}
            />
          </Flex>
        </Header>
        <Content
          style={{
            margin: "24px 16px",
            padding: 24,
            background: colorBgContainer,
            borderRadius: borderRadiusLG,
            width: "auto",
          }}
        >
          {tokens && children}
          {!tokens && <LoadingScreen />}
        </Content>
      </Layout>
    </Layout>
  );
}
