"use client";

import TextInput from "@/stories/Generic/TextInput/TextInput";
import FormMetadata from "@/components/form/edit_form/info/FormMetadata";
import {default as SaveStatusComponent} from "@/components/form/edit_form/info/SaveStatus";
import {Form, Item, SaveStatus} from "@/lib/types";

import styles from "./EditForm.module.css"
import {Item as ItemComponent} from "@/components/form/edit_form/Item";

type Props = {
  form: Form,
  setForm: (form: Form) => void,
  saveStatus: SaveStatus
}

/**
 * フォームの編集画面
 * @param form フォーム
 * @param setForm フォーム更新用関数
 * @param saveStatus セーブステータス
 * @constructor
 */
export default function EditForm({form, setForm, saveStatus}: Props) {
  const handleTitleChange = (value: string) => {
    console.log(form)
    setForm({
      ...form,
      info: {
        ...form.info,
        title: value
      }
    })
    console.log(form)
  };

  const handleDescriptionChange = (value: string) => {
    setForm({
      ...form,
      info: {
        ...form.info,
        description: value
      }
    })
  };


  const setItem = (item: Item) => {
    setForm({
      ...form,
      items: form.items.map((item_) => item_.item_id === item.item_id ? item : item_)
    })
  }

  const moveItemUp = (item: Item) => () => {
    const index = form.items.findIndex((item_) => item_ === item);
    if (index === 0) return;
    setForm({
      ...form,
      items: [
        ...form.items.slice(0, index - 1),
        form.items[index],
        form.items[index - 1],
        ...form.items.slice(index + 1),
      ],
    })
  };

  const moveItemDown = (item: Item) => () => {
    const index = form.items.findIndex((item_) => item_ === item);
    if (index === -1) return;
    setForm(
      {
        ...form,
        items: [
          ...form.items.slice(0, index),
          form.items[index + 1],
          form.items[index],
          ...form.items.slice(index + 2),
        ],
      }
    )
  };

  const deleteItem = (item: Item) => () => {
    setForm(
      {
        ...form,
        items: form.items.filter((item_) => item_ !== item),
      }
    )
  };

  const createNewItem = (itemType: string) => {
    const newItem: Item = {
      item_id: crypto.randomUUID(),
      title: "タイトル",
      description: "概要",
      ...(itemType === "text" && {item_text: {}}),
      ...(itemType === "page_break" && {item_page_break: {}}),
      ...(itemType === "question" && {
        item_question: {
          question: {
            required: false,
            question_text: {
              paragraph: false,
            },
          },
        },
      }),
    }
    setForm(
      {
        ...form,
        items: [...form.items, newItem],
      }
    )
  };

  const renderItems = () => {
    if (!form || !form.items) return null;

    return form.items.map((item) => (
      <ItemComponent
        key={item.item_id}
        item={item}
        setItem={setItem}
        moveUp={moveItemUp(item)}
        moveDown={moveItemDown(item)}
        delete_={deleteItem(item)}
      />
    ));
  };

  return (
    <div className={styles.page}>
      <main className={styles.main}>
        <div className={styles.formTitleWrapper}>
          <TextInput
            width={400}
            placeholder="タイトルを入力"
            value={form?.info?.title ?? "データなし"}
            setValue={handleTitleChange}
            paragraph={false}
          />
          <TextInput
            placeholder="説明文を入力"
            value={form?.info?.description ?? "データなし"}
            setValue={handleDescriptionChange}
            paragraph={true}
          />
          <FormMetadata form={form} setForm={setForm}/>

          <div className={styles.formMenuWrapper}>
            <SaveStatusComponent status={saveStatus}/>
          </div>
        </div>
        {/* 動的に生成された Question コンポーネントを表示 */}
        {renderItems()}
        <div className={styles.newItemWrapper}>
          項目を追加：
          <button onClick={() => createNewItem("text")}>テキスト</button>
          <button onClick={() => createNewItem("question")}>質問</button>
          <button onClick={() => createNewItem("page_break")}>改ページ</button>
        </div>
      </main>
    </div>
  )
}