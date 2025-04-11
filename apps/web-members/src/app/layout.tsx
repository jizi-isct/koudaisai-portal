"use client";
import { Noto_Sans_JP } from "next/font/google";
import "./globals.css";
import { Header } from "@koudaisai-portal/ui-generic";
import Footer from "@/components/Footer/Footer";
import { usePathname } from "next/navigation";
import { isLoggedInMembers } from "@koudaisai-portal/util";
import { useEffect, useState } from "react";

const notoSans = Noto_Sans_JP({
  subsets: ["latin"],
  weight: "400",
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  //ログイン状態を管理するstate
  //nullはログイン状態がわからないことを示す
  const [loggedIn, setLoggedIn] = useState<boolean | null>(null);

  useEffect(() => {
    isLoggedInMembers().then(setLoggedIn);
  }, []);

  const pathname = usePathname();
  return (
    <html lang="ja">
      <body className={notoSans.className}>
        <Header header_type="members" currentPath={pathname} isLoggedIn={loggedIn} />
        {children}
        <Footer />
      </body>
    </html>
  );
}
