import type {Metadata} from "next";
import {Noto_Sans_JP} from "next/font/google";
import "../globals.css";
import {Footer, Header, MobileNavigator} from "@/components/generic";

const notoSans = Noto_Sans_JP({
    subsets: ["latin"],
    weight: ["100", "200", "300", "400", "500", "600", "700", "800", "900"]
});

export const metadata: Metadata = {
  title: '工大祭ポータル',
  description: 'このサイトは工大祭実行委員会公式の参加団体向けポータルサイトです。このサイトを通じて工大祭への参加に関する各種手続きを行うことができます。一緒に工大祭を創りあげましょう！',
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body className={notoSans.className} id={"app"}>
        <Header header_type="members"  />
        <main className="content">
          {children}
        </main>
        <MobileNavigator header_type="members"/>
        <Footer />
      </body>
    </html>
  );
}
