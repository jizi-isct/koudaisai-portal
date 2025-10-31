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
import {JSX, useState} from "react";
import {AppstoreOutlined, CompassOutlined} from "@ant-design/icons";

const kiwiMaru = Kiwi_Maru({
  weight: ['300', '400', '500'],
  subsets: ['latin'],
  display: 'swap'
});

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

export function EditAdditionalInfo() {
  const [style, setStyle] = useState<"app" | "web">("app")

  return (
    <div className={styles.editorWrapper}>
      <MDXEditor
        markdown="Hello world"
        contentEditableClassName={style === "app" ? `${styles.app} ${kiwiMaru.className}` : styles.web}
        plugins={[
          toolbarPlugin({
            toolbarContents: () => (
              <DiffSourceToggleWrapper>
                <UndoRedo/>
                <Separator/>
                <SingleChoiceToggleGroup items={styleChoiceItems} onChange={(v) => setStyle(v as "app" | "web")}
                                         value={style}/>
                <Separator/>
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
          diffSourcePlugin({viewMode: 'rich-text'}),
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