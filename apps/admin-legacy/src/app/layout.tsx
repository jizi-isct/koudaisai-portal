import './global.css';
import {ConfigProvider} from "antd";
import type {ThemeConfig} from "antd";

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
      <body style={{margin: 0}} id="app">
        <ConfigProvider theme={antdTheme}>
          {children}
        </ConfigProvider>
      </body>
    </html>
  );
}
