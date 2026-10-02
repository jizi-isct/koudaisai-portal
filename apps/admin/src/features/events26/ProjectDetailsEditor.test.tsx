// @vitest-environment jsdom
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { afterEach, expect, it, vi } from 'vitest';
import { ProjectDetailsEditor } from './ProjectDetailsEditor';

const state = vi.hoisted(() => ({
  refetchCount: 0,
  details: {
    menu: { description: '保存済みメニュー', items: [] },
    additionalInfo: '保存済み追加情報',
  },
}));

vi.mock('@/features/api/api', async () => {
  const React = await import('react');
  return {
    $events26Api: {
      useQuery: () => {
        const [data, setData] = React.useState(state.details);
        return {
          data,
          error: null,
          isLoading: false,
          refetch: async () => {
            state.refetchCount += 1;
            setData(structuredClone(state.details));
            return { data: state.details };
          },
        };
      },
    },
    $api: {
      useMutation: (_method: string, path: string) => ({
        mutateAsync: async ({ body }: { body?: unknown }) => {
          if (path.endsWith('/menu')) {
            state.details = {
              ...state.details,
              menu: body as typeof state.details.menu,
            };
          } else if (path.endsWith('/additional_info')) {
            state.details = {
              ...state.details,
              additionalInfo: body as string,
            };
          }
        },
      }),
    },
  };
});

vi.mock('@mdxeditor/editor', async () => {
  const React = await import('react');
  const Empty = () => null;
  const MDXEditor = React.forwardRef<
    { setMarkdown: (markdown: string) => void },
    { markdown: string; onChange: (markdown: string) => void }
  >(function Editor({ markdown, onChange }, ref) {
    const [value, setValue] = React.useState(markdown);
    React.useImperativeHandle(ref, () => ({ setMarkdown: setValue }));
    return (
      <textarea
        aria-label="企画追加情報エディタ"
        value={value}
        onChange={(event) => {
          setValue(event.target.value);
          onChange(event.target.value);
        }}
      />
    );
  });
  const plugin = () => ({});
  return {
    AdmonitionDirectiveDescriptor: {},
    BlockTypeSelect: Empty,
    BoldItalicUnderlineToggles: Empty,
    CodeToggle: Empty,
    CreateLink: Empty,
    DiffSourceToggleWrapper: ({ children }: React.PropsWithChildren) =>
      children,
    InsertTable: Empty,
    InsertThematicBreak: Empty,
    ListsToggle: Empty,
    MDXEditor,
    Separator: Empty,
    StrikeThroughSupSubToggles: Empty,
    UndoRedo: Empty,
    diffSourcePlugin: plugin,
    directivesPlugin: plugin,
    headingsPlugin: plugin,
    imagePlugin: plugin,
    linkDialogPlugin: plugin,
    linkPlugin: plugin,
    listsPlugin: plugin,
    markdownShortcutPlugin: plugin,
    quotePlugin: plugin,
    tablePlugin: plugin,
    thematicBreakPlugin: plugin,
    toolbarPlugin: plugin,
  };
});

function input(textarea: HTMLTextAreaElement, value: string) {
  Object.getOwnPropertyDescriptor(
    HTMLTextAreaElement.prototype,
    'value',
  )?.set?.call(textarea, value);
  textarea.dispatchEvent(new Event('input', { bubbles: true }));
}

function button(label: string) {
  const result = [...document.querySelectorAll('button')].find((element) =>
    element.textContent?.includes(label),
  );
  if (!result) throw new Error(`Button not found: ${label}`);
  return result;
}

let root: ReturnType<typeof createRoot> | undefined;
afterEach(async () => {
  if (root) await act(async () => root?.unmount());
  document.body.innerHTML = '';
  root = undefined;
});

it('片方を保存して再取得しても、もう片方の未保存入力を保持する', async () => {
  Reflect.set(globalThis, 'IS_REACT_ACT_ENVIRONMENT', true);
  vi.stubGlobal(
    'ResizeObserver',
    class {
      observe = vi.fn();
      unobserve = vi.fn();
      disconnect = vi.fn();
    },
  );
  window.matchMedia = vi.fn().mockImplementation((query: string) => ({
    matches: false,
    media: query,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  }));
  state.refetchCount = 0;
  state.details = {
    menu: { description: '保存済みメニュー', items: [] },
    additionalInfo: '保存済み追加情報',
  };
  const container = document.createElement('div');
  document.body.append(container);
  root = createRoot(container);
  await act(async () =>
    root?.render(<ProjectDetailsEditor projectId="M-001" />),
  );

  const editor = document.querySelector<HTMLTextAreaElement>(
    'textarea[aria-label="企画追加情報エディタ"]',
  );
  let description = document.querySelector<HTMLTextAreaElement>('#description');
  expect(editor?.value).toBe('保存済み追加情報');
  expect(description?.value).toBe('保存済みメニュー');
  if (!editor || !description) throw new Error('Editors not found');

  await act(async () => input(editor, '未保存の追加情報'));
  state.details.additionalInfo = '別の管理者が更新した追加情報';
  await act(async () => {
    button('メニューを保存').click();
    await new Promise((resolve) => setTimeout(resolve, 0));
  });
  expect(state.refetchCount).toBe(1);
  expect(editor.value).toBe('未保存の追加情報');

  description = document.querySelector<HTMLTextAreaElement>('#description');
  if (!description) throw new Error('Menu editor not found');
  const menuEditor = description;
  await act(async () => input(menuEditor, '未保存のメニュー'));
  state.details.menu = {
    description: '別の管理者が更新したメニュー',
    items: [],
  };
  await act(async () => {
    button('企画追加情報を保存').click();
    await new Promise((resolve) => setTimeout(resolve, 0));
  });
  expect(state.refetchCount).toBe(2);
  expect(state.details.additionalInfo).toBe('未保存の追加情報');
  expect(
    document.querySelector<HTMLTextAreaElement>('#description')?.value,
  ).toBe('未保存のメニュー');
});
