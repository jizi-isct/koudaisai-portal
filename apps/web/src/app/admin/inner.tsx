"use client";

import {getTokensAdmin, Tokens} from "@/lib";
import {ReactNode, useEffect, useState} from "react";
import {Button, Flex, Layout, Menu, theme} from "antd";
import {
  BellOutlined,
  BookOutlined,
  FormOutlined,
  HomeOutlined,
  MenuFoldOutlined,
  MenuUnfoldOutlined,
} from "@ant-design/icons";
import {Header} from "antd/lib/layout/layout";
import Sider from "antd/es/layout/Sider";
import {Content} from "antd/es/layout/layout";
import Image from "next/image";
import {LoadingScreen} from "@/components/generic";
import {usePathname, useRouter} from "next/navigation";
import {useWindowWidth} from "@wojtekmaj/react-hooks";

type Props = {
  children: ReactNode
}

export function Inner({children}: Props) {
  const [tokens, setTokens] = useState<Tokens | null | undefined>();
  useEffect(() => {
    (async () => {
      const tokens = await getTokensAdmin()
      if (tokens) {
        setTokens(tokens)
      } else {
        await router.push(process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/login")
      }
    })()
  }, [])

  const width = useWindowWidth() ?? 10000;
  const [collapsed, setCollapsed] = useState(width < 768);


  const router = useRouter()
  const {
    token: {colorBgContainer, borderRadiusLG},
  } = theme.useToken();

  const pathname = usePathname()

  return (
    <Layout style={{minHeight: '100vh', width: '100vw'}}>
      <Sider collapsible collapsed={collapsed} style={{margin: 0, padding: 0}}>
        <Menu
          theme="dark"
          mode="inline"
          selectedKeys={[pathname]}
          style={{position: "sticky", top: 0}}
          items={[
            {
              key: process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/",
              icon: <HomeOutlined/>,
              label: 'ホーム',
              onClick: async () => {
                await router.push(process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/")
              }
            },
            {
              key: process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/forms/",
              icon: <FormOutlined/>,
              label: 'フォーム',
              onClick: async () => {
                await router.push(process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/forms/")
              }
            },
            {
              key: process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/documents/",
              icon: <BookOutlined/>,
              label: '資料',
              onClick: async () => {
                await router.push(process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/documents/")
              }
            },
            {
              key: process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/notifications/",
              icon: <BellOutlined/>,
              label: '通知',
              onClick: async () => {
                await router.push(process.env.NEXT_PUBLIC_ADMIN_BASE_PATH + "/notifications/")
              }
            },
          ]}
        />
      </Sider>
      <Layout>
        <Header style={{padding: 0, background: colorBgContainer}}>
          <Flex>
            <Button
              type="text"
              icon={collapsed ? <MenuUnfoldOutlined/> : <MenuFoldOutlined/>}
              onClick={() => setCollapsed(!collapsed)}
              style={{
                fontSize: '16px',
                width: 64,
                height: 64,
              }}
            />
            <a href={"/admin"} style={{color: "black"}}>
              <Flex align={"center"} gap={10} justify={"center"}>
                <Image
                  src="/components/generic/Header/admin_logo.jpg"
                  alt="Koudaisai Portal Admin Site Logo"
                  width={32}
                  height={32}
                />
                {<span>工大祭ポータル管理サイト</span>}
              </Flex>
            </a>
          </Flex>
        </Header>
        <Content
          style={{
            margin: '24px 16px',
            padding: 24,
            background: colorBgContainer,
            borderRadius: borderRadiusLG,
            width: "auto",
          }}
        >
          {tokens && children}
          {!tokens && <LoadingScreen/>}
        </Content>
      </Layout>
    </Layout>
  );
}