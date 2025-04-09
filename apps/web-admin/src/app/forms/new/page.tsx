"use client";

import {useState} from "react";
import {fetchClientAdmin, Form} from "@koudaisai-portal/util";

export default function Page() {
    const [title, setTitle] = useState<string>("");
    const [documentTitle, setDocumentTitle] = useState<string>("");
    const [description, setDescription] = useState<string>("");
    const [progress, setProgress] = useState<("input" | "posting" | "error")>("input");
    const handleButtonClick = () => {
        setProgress("posting")
        const form: Form = {
            info: {
                title: title,
                document_title: documentTitle,
                description: description
            },
            items: [],
            access_control: {
                roles: []
            }
        }

        fetchClientAdmin.POST(
            "/forms",
            {
                body: form
            }
        ).then(({data}) => {
            if (data) {
                location.assign("/admin/forms/form?formId=" + data!.form_id)
            } else {
                setProgress("error")
            }
        })
    }
    return (
        <main>
            {progress == "input" && <p>必要な情報を入力して作成をクリックしてください</p>}
            {progress == "posting" && <p>フォーム作成中...</p>}
            {progress == "error" && <p>フォーム作成中にエラーが発生しました．</p>}
            フォームのタイトル：<input type="text" value={title} onChange={(e) => setTitle(e.target.value)}/>
            管理用の名称：<input type="text" value={documentTitle} onChange={(e) => setDocumentTitle(e.target.value)}/>
            フォームの概要：<input type="text" value={description} onChange={(e) => setDescription(e.target.value)}/>
            <button onClick={handleButtonClick}>作成</button>
        </main>
    )
}