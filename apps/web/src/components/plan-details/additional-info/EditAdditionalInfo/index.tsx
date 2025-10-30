"use client";

import '@mdxeditor/editor/style.css'
import styles from './index.module.css'
import {
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
  AdmonitionDirectiveDescriptor, imagePlugin
} from "@mdxeditor/editor";
import { Kiwi_Maru } from 'next/font/google';

const kiwiMaru = Kiwi_Maru({
  weight: ['300','400','500'],
  subsets: ['latin'],
  display: 'swap'
});

export function EditAdditionalInfo() {
  return (
    <div className={styles.editorWrapper}>
      <MDXEditor
        markdown="Hello world"
        contentEditableClassName={`${styles.mdxContent} ${kiwiMaru.className}`}
        plugins={[
          toolbarPlugin({
            toolbarContents: () => (
              <DiffSourceToggleWrapper>
                <UndoRedo />
                <Separator />
                <BoldItalicUnderlineToggles />
                <StrikeThroughSupSubToggles />
                <Separator />
                <ListsToggle />
                <Separator />
                <CreateLink />
                <CodeToggle />
                <Separator />
                <InsertTable />
                <InsertThematicBreak />
              </DiffSourceToggleWrapper>
            )
          }),
          imagePlugin(),
          diffSourcePlugin({ viewMode: 'rich-text' }),
          headingsPlugin({ allowedHeadingLevels: [1, 2, 3, 4, 5, 6] }),
          listsPlugin(),
          tablePlugin(),
          quotePlugin(),
          thematicBreakPlugin(),
          linkPlugin(),
          linkDialogPlugin(),
          directivesPlugin({ directiveDescriptors: [AdmonitionDirectiveDescriptor] }),
          markdownShortcutPlugin(),
        ]}
      />
    </div>
  )
}