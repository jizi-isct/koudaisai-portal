import type {Metadata} from "next";
import {Noto_Sans_JP} from "next/font/google";
import {ConfigProvider} from "antd";
import "./globals.css";
import "./members.css";

const notoSans = Noto_Sans_JP({subsets: ["latin"], weight: ["400", "700"]});

export const metadata: Metadata = {
  title: "工大祭ポータル",
  description: "参加団体向けポータルサイト",
};

const antdTheme = {
  token: {colorPrimary: "#0048FF", borderRadius: 8},
  components: {Button: {contentFontSize: 12, paddingInline: 17}},
};

export default function RootLayout({children}: {children: React.ReactNode}) {
  return (
    <html lang="ja">
      <body className={notoSans.className} id="app">
        <ConfigProvider theme={antdTheme}>
          {children}
        </ConfigProvider>
      </body>
    </html>
  );
}
