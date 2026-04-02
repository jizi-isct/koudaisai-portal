import {Inner} from "./inner";

export default function AdminLayout({children}: {children: React.ReactNode}) {
  return <Inner>{children}</Inner>;
}
