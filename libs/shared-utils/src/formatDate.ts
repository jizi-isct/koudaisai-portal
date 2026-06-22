// サーバーから返却された日付をYYYY/MM/DD HH:MM形式で表示

export function formatDate(value: string) {
  return new Date(value).toLocaleString('ja-JP', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  });
}
