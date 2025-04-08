"use client";
import styles from "./page.module.css";
import {useSearchParams} from "next/navigation";
import {Suspense, useEffect, useState} from "react";
import {TextInput} from "@koudaisai-portal/ui-generic";
import {FormMetadata, Item as ItemComponent, SaveStatus} from "@koudaisai-portal/ui-edit_form";
import {$apiAdmin, fetchClientAdmin, Form, Item} from "@koudaisai-portal/util";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";

export default function Page() {
    return (
        <Suspense>
            <QueryClientProvider client={new QueryClient()}>
                <Inner/>
            </QueryClientProvider>
        </Suspense>
    )
}

function Inner() {
    const searchParams = useSearchParams();
    const formId = searchParams.get("formId");

    if (!formId) {
        window.location.assign("/404")
    }

    const {data, error} = $apiAdmin.useQuery(
        "get",
        "/forms/{form_id}",
        {
            params: {
                path: {
                    form_id: formId!
                }
            }
        }
    )
    const [form, _setForm] = useState<Form>();
    const [saveStatus, setSaveStatus] = useState<"saving" | "saved" | "unsaved">("saved");

    useEffect(() => {
        if (data) {
            _setForm(data)
        }
    }, [data])

    if (!data || !form) {
        return <p>loading...</p>;
    }

    // フォームのデータをサーバーへ保存
    const saveForm = async (form: Form) => {
        setSaveStatus("saving");

        const {error} = await fetchClientAdmin.PUT(
            `/forms/{form_id}`,
            {
                params: {
                    path: {
                        form_id: formId!
                    }
                },
                body: {
                    info: form.info,
                    items: form.items,
                    access_control: form.access_control
                }
            }
        );
        if (error) {
            console.error("保存に失敗しました", error);
            setSaveStatus("unsaved");
        } else {
            setSaveStatus("saved");
        }
    };

    const setForm = (form: Form) => {
        saveForm(form).then()
        _setForm(form)
    }

    const handleTitleChange = (value: string, args?: string[]) => {
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

    const handleDescriptionChange = (value: string, args?: string[]) => {
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
            // <Question key={item.item_id} itemId={item.item_id} form={form} updateItem={updateItem}
            //           toggleRequired={toggleRequired} reorderQuestionUp={reorderQuestionUp}
            //           reorderQuestionDown={reorderQuestionDown} deleteQuestion={deleteQestion}>
            //   {/* itemの種類によって異なる入力コンポーネントを表示 */}
            //   {item.item_question?.question?.question_text?.paragraph ? <ParagraphInput fontSize={14} placeholder="長文回答" /> : <TextInput fontSize={14} placeholder="短文回答" />}
            //   {item.item_question && (
            //     <>
            //     </>
            //   )}
            // </Question>
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


    if (error) return <p>データの取得に失敗しました</p>;
    if (!data) return <p>読み込み中...</p>;

    // form_id に一致するフォームを検索
    // const form = data.find((f: any) => f.form_id === formId);
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
                        <SaveStatus status={saveStatus}/>
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
    );
}