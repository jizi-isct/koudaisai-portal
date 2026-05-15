export const headerItemsMembers = [
  {desktopText: "ホーム", mobileText: "ホーム", emoji: "🏠", href: "/", class: "navHome"},
  {desktopText: "フォーム", mobileText: "フォーム", emoji: "📄", href: "/forms/", class: "navForm"},
  {desktopText: "資料", mobileText: "資料", emoji: "📚", href: "/documents/", class: "navDocuments"},
  {desktopText: "よくある質問", mobileText: "FAQ", emoji: "❓", href: "/questions/", class: "navQuestions"}
] as const;

export const headerItemsAdmin = [
  {desktopText: "ホーム", mobileText: "ホーム", emoji: "🏠", href: "/admin/", class: "navHome"},
  {desktopText: "フォーム", mobileText: "フォーム", emoji: "📄", href: "/admin/forms/", class: "navForm"},
  {desktopText: "資料", mobileText: "資料", emoji: "📚", href: "/admin/documents/", class: "navDocuments"},
] as const;
