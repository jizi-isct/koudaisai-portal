"use client";

export default function Page() {
  return (
    <div>
      <main>
        <img
          src="/logo.jpg"
          alt="Koudaisai Portal Admin Site Logo"
          width={150}
          height={150}
        />
        <h1>ようこそ</h1>
        <h2>ページ一覧</h2>
        <ul>
          <li><a href="/forms/">フォーム一覧</a></li>
          <li><a href="/documents/">資料一覧</a></li>
          <li><a href="/notifications/">通知一覧</a></li>
        </ul>
      </main>
    </div>
  );
}
