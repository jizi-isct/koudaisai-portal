"use client";
import {Noto_Sans_JP} from "next/font/google";
import "./globals.css";
import {Header} from "@koudaisai-portal/ui-generic";
import Footer from "@/components/Footer/Footer";

const notoSans = Noto_Sans_JP({
    subsets: ["latin"],
    weight: "400"
});


export default function RootLayout({
                                       children,
                                   }: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <html lang="ja">
        <body className={notoSans.className}>
        <Header/>
        {children}
        <Footer/>
        </body>
        </html>
    );
}
