import {Metadata} from "next";

export const metadata: Metadata = {
  title: '工大祭ポータル',
  description: 'このサイトは工大祭実行委員会公式の参加団体向けポータルサイトです。このサイトを通じて工大祭への参加に関する各種手続きを行うことができます。一緒に工大祭を創りあげましょう！',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
