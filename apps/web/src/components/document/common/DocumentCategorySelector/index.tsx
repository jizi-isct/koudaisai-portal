"use client";

import {DocumentCategoryRead} from "@/lib";

type DocumentCategorySelectorProps = {
  categories: DocumentCategoryRead[],
  categoryId: string | null,
  setCategoryId: (categoryId: string | null) => void
}

export function DocumentCategorySelector({categories, categoryId, setCategoryId}: DocumentCategorySelectorProps) {
  const handleCategoryChange = (value: string) => {
    if (categories.find(data => data.id === value)) {
      setCategoryId(value)
    } else {
      setCategoryId(null)
    }
  }
  return (
    <label>
      資料カテゴリを選択
      <select value={categoryId ?? "none"} onChange={e => handleCategoryChange(e.target.value)}>
        {
          categories?.map(
            (data, i) =>
              <option key={`option-${i}`} value={data.id}>{data.title}</option>
          )
        }
        {
          categoryId ? <></> : <option key={`option-none`} value={"none"}>カテゴリなし</option>
        }
      </select>
    </label>
  )
}