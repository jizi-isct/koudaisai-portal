import {Metadata} from "next";
import {ConfigProvider} from "antd";
import {Footer, Header} from "@koudaisai/shared-ui";
import '../globals.css';
import '../members.css';

export const metadata: Metadata = {
  title: '工大祭ポータル',
  description: 'このサイトは工大祭実行委員会公式の参加団体向けポータルサイトです。このサイトを通じて工大祭への参加に関する各種手続きを行うことができます。一緒に工大祭を創りあげましょう！',
}

const antdTheme = {
  token: {
    colorPrimary: "#0048FF",
    borderRadius: 8,
  },
  components: {
    Button: {
      contentFontSize: 12,
      paddingInline: 17,
    },
  },
}

export default function RootLayout({
                                     children,
                                   }: {
  children: React.ReactNode
}) {
  return (
    <html lang="ja">
    <body id={"app"}>
    <Header/>
    <ConfigProvider theme={antdTheme}>
      {children}
    </ConfigProvider>
    <Footer isLoggedIn={false}/>
    </body>
    </html>
  )
}