import './global.css';
import {Noto_Sans_JP} from "next/font/google";
import {ConfigProvider} from "antd";
import type {Metadata} from "next";
import type {ThemeConfig} from "antd";

const notoSans = Noto_Sans_JP({
  subsets: ["latin"],
  weight: "400",
});

export const metadata: Metadata = {
  title: "工大祭管理サイト",
  robots: "noindex",
};

const antdTheme: ThemeConfig = {
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
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="ja">
      <body className={notoSans.className} style={{margin: 0}} id="app">
        <ConfigProvider theme={antdTheme}>
          {children}
        </ConfigProvider>
      </body>
    </html>
  );
}
