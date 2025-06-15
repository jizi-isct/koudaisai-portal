// lib/api/user.ts
import { $apiMembers } from "@/lib/api";

export const getCurrentUser = async () => {
  const res = await $apiMembers.GET("/users/me");
  return res.data;
};
