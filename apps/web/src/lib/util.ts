"use client";

import {fetchClientNoAuth} from "@/lib/api";
import {useCallback, useEffect, useState} from "react";
import {GroupRead, UserRead} from "@/lib/types";

export function chunk<T>(array: T[], size: number): T[][] {
  const result: T[][] = [];
  for (let i = 0; i < array.length; i += size) {
    result.push(array.slice(i, i + size));
  }
  return result;
}

export async function getDownloadUrl(fileKey: string, fileName: string) {
  return await fetchClientNoAuth.GET(
    "/files/download",
    {
      params: {
        query: {
          key: fileKey,
          file_name: fileName
        }
      }
    }
  )
}

export function useDownloadUrl(fileKey: string, fileName: string) {
  const [downloadUrl, setDownloadUrl] = useState<string | undefined>(undefined);
  const [error, setError] = useState<string | undefined>(undefined);

  useEffect(() => {
    (async () => {
      const {data, error: error_} = await getDownloadUrl(fileKey, fileName)
      if (data) {
        setDownloadUrl(data.presigned_url);
      } else {
        setError(`Error fetching download URL: ${error_}`);
      }
    })()
  })

  return {
    downloadUrl,
    error
  }
}

export function useDownload() {
  return useCallback(
    (url: string, fileName: string) => {
      const a = document.createElement("a");
      a.href = url;
      a.download = fileName;
      a.style.display = "none";
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);

      URL.revokeObjectURL(url);
    }, []
  );
}

const PASSWORD_PATTERN = /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[~`!@#$%^&*()_+-={}[|;:'",<.>/?])(?=.{8,})/;

export function validatePassword(password: string) {
  return PASSWORD_PATTERN.test(password);
}

export function getRepresentativeIndex(user: UserRead, group: GroupRead): string | undefined {
  if (group.type_press) {
    return undefined
  }
  if (group.type_plan?.type_booth) {
    if (group.type_plan.type_booth.representative1 === user.id) {
      return "一";
    } else if (group.type_plan.type_booth.representative2 === user.id) {
      return "二";
    } else if (group.type_plan.type_booth.representative3 === user.id) {
      return "三";
    }
  }
  if (group.type_plan?.type_general) {
    if (group.type_plan.type_general.representative1 === user.id) {
      return "一";
    } else if (group.type_plan.type_general.representative2 === user.id) {
      return "二";
    } else if (group.type_plan.type_general.representative3 === user.id) {
      return "三";
    }
  }
  if (group.type_plan?.type_stage) {
    if (group.type_plan.type_stage.representative1 === user.id) {
      return "一";
    } else if (group.type_plan.type_stage.representative2 === user.id) {
      return "二";
    } else if (group.type_plan.type_stage.representative3 === user.id) {
      return "三";
    }
  }
  if (group.type_plan?.type_labo) {
    return undefined
  }
  return undefined
}

export function getFilesRedirectUrl(fileKey: string, fileName: string) {
  return `${process.env.NEXT_PUBLIC_API_BASE_URL}/files/download?key=${fileKey}&file_name=${fileName}&redirect=true`
}