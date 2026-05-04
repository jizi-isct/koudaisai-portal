"use client";

import Link from "next/link";
import Image from "next/image";

import membersLogo from "./assets/members_logo.svg"

export const Footer = ({isLoggedIn}: {isLoggedIn ?: boolean}) => {
  return (
    <footer className="bg-white w-full text-center py-5 shadow-[0_0_12px_0_rgba(37,54,97,0.25)]">
      <div>
        <Image
          src={membersLogo}
          alt="Koudaisai Portal Members Site Logo"
          width={40}
          height={40}
        />
      </div>
      <div className="flex justify-center items-center gap-3 py-3">
        <Link href="/" className="text-sm font-bold">ホーム</Link>
        <Link href="/forms/" className="text-sm font-bold">フォーム</Link>
        <Link href="/documents/" className="text-sm font-bold">資料</Link>
        <Link href="/questions/" className="text-sm font-bold">よくある質問</Link>
      </div>
      <p className="text-sm font-medium">
        公式LINEアカウント：{
        isLoggedIn === undefined ? <>...</> : isLoggedIn
          ? <a href={"https://lin.ee/9Sud7lK"}>https://lin.ee/9Sud7lK</a>
          : <a href={"https://lin.ee/43ugikz"}>https://lin.ee/43ugikz</a>
      } <br/>
        メールアドレス([at]を@に置き換えてください)：sanka[at]koudaisai.jp
      </p>
      <p className="text-[10px] font-medium">©︎ 2025 JIZI All Rights Reserved.</p>
    </footer>
  );
};
