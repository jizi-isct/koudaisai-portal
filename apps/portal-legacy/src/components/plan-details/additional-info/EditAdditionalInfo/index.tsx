"use client";

import '@mdxeditor/editor/style.css'
import styles from './index.module.css'
import {
  BlockTypeSelect,
  BoldItalicUnderlineToggles,
  CodeToggle,
  CreateLink,
  DiffSourceToggleWrapper,
  InsertTable,
  InsertThematicBreak,
  ListsToggle,
  Separator,
  StrikeThroughSupSubToggles,
  UndoRedo,
  diffSourcePlugin,
  headingsPlugin,
  linkDialogPlugin,
  linkPlugin,
  listsPlugin,
  markdownShortcutPlugin,
  MDXEditor,
  quotePlugin,
  tablePlugin,
  thematicBreakPlugin,
  toolbarPlugin,
  directivesPlugin,
  AdmonitionDirectiveDescriptor, imagePlugin, SingleChoiceToggleGroup
} from "@mdxeditor/editor";
import {Kiwi_Maru} from 'next/font/google';
import {useState} from "react";
import {AppstoreOutlined, CompassOutlined, LoadingOutlined, UploadOutlined} from "@ant-design/icons";
import {Button} from "antd";

// フォント
const kiwiMaru = Kiwi_Maru({
  weight: ['300', '400', '500'],
  subsets: ['latin'],
  display: 'swap'
});

// 表示切り替えのリスト
const styleChoiceItems = [
  {
    title: "アプリ表示",
    value: "app",
    contents: <AppstoreOutlined/>
  },
  {
    title: "公式サイト表示",
    value: "web",
    contents: <CompassOutlined/>
  }
]

type Props = {
  additionalInfo: string,
  updateAdditionalInfo: (newAdditionalInfo: string) => Promise<void>,
  isLoading: boolean,
}

export function EditAdditionalInfo({additionalInfo, updateAdditionalInfo, isLoading}: Props) {
  const [style, setStyle] = useState<"app" | "web">("app")
  const [currentMarkdown, setCurrentMarkdown] = useState<string>(additionalInfo)

  return (
    <div className={styles.editorWrapper}>
      <MDXEditor
        markdown={additionalInfo}
        onChange={(md) => setCurrentMarkdown(md)}
        contentEditableClassName={style === "app" ? `${styles.app} ${kiwiMaru.className}` : styles.web}
        plugins={[
          toolbarPlugin({
            toolbarContents: () => (
              <DiffSourceToggleWrapper>
                <Button
                  size={"small"}
                  disabled={isLoading || currentMarkdown === additionalInfo}
                  onClick={async () => {await updateAdditionalInfo(currentMarkdown)}}
                >{isLoading ? <LoadingOutlined/> : <UploadOutlined/>} 更新</Button>
                <Separator/>
                <SingleChoiceToggleGroup items={styleChoiceItems} onChange={(v) => setStyle(v as "app" | "web")}
                                         value={style}/>
                <Separator/>
                <UndoRedo/>
                <BlockTypeSelect/>
                <Separator/>
                <BoldItalicUnderlineToggles/>
                <StrikeThroughSupSubToggles/>
                <Separator/>
                <ListsToggle/>
                <Separator/>
                <CreateLink/>
                <CodeToggle/>
                <Separator/>
                <InsertTable/>
                <InsertThematicBreak/>
              </DiffSourceToggleWrapper>
            )
          }),
          imagePlugin(),
          diffSourcePlugin({diffMarkdown: additionalInfo, viewMode: 'rich-text'}),
          headingsPlugin({allowedHeadingLevels: [1, 2, 3, 4, 5, 6]}),
          listsPlugin(),
          tablePlugin(),
          quotePlugin(),
          thematicBreakPlugin(),
          linkPlugin(),
          linkDialogPlugin(),
          directivesPlugin({directiveDescriptors: [AdmonitionDirectiveDescriptor]}),
          markdownShortcutPlugin(),
        ]}
      />
    </div>
  )
}