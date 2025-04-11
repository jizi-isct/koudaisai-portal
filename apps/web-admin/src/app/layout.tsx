"use client";
import {Noto_Sans_JP} from "next/font/google";
import "./globals.css";
import {Header} from "@koudaisai-portal/ui-generic";
import Footer from "@/components/Footer/Footer";
import { usePathname } from 'next/navigation';

const notoSans = Noto_Sans_JP({
  subsets: ["latin"],
  weight: "400"
});



export default function RootLayout({
                                     children,
                                   }: Readonly<{
  children: React.ReactNode;
}>) {
  console.log(children);
  const pathname = usePathname();
  return (
    <html lang="ja">
    <body className={notoSans.className}>
    <Header header_type="admin" currentPath={pathname}/>
    {children}
    <Footer/>
    </body>
    </html>
  );
}
