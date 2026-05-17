import { Heading2, Steps, Tab } from "@koudaisai/shared-ui";
import type { JSX } from "react";

type StepItem = {
  title: string;
  date?: string;
  children: JSX.Element;
};

type TopPageData = {
  booth: [StepItem[], StepItem[]];
  general: [StepItem[], StepItem[]];
  stage: [StepItem[], StepItem[]];
  labo: [StepItem[]];
};

const DocumentLink = () => <a href="/documents/">こちら</a>;
const FormLink = () => <a href="/forms/">こちら</a>;

const topPageData: TopPageData = {
  booth: [
    [
      {
        title: "STEP 1",
        children: (
          <>
            参加説明会に出席！<br />
            説明会は5/13(水)と5/15(金)に実施<br />
            資料は<DocumentLink />から
          </>
        ),
      },
      {
        title: "STEP 2",
        date: "～6/11(木)",
        children: (
          <>
            参加申請をしよう！<br />
            申請期間は5/13(水)から6/11(木)まで<br />
            参加申請のフォームは<FormLink />
          </>
        ),
      },
      {
        title: "STEP 3",
        children: (
          <>
            参加申請完了！<br />
            工大祭実行委員会からの<br />
            申請結果通知を待とう！
          </>
        ),
      },
    ],
    [
      {
        title: "参加申込金振り込み",
        date: "～7/1(水)",
        children: <>6/19(金)の参加資格通知または6/26(金)の補欠合格の通知があった団体の方は7/1(水)までに参加申込金を振り込んでください。</>,
      },
      {
        title: "第一回参加団体総会",
        date: "7/8(水)",
        children: (
          <>
            全団体に出席義務があり、責任者のうち少なくとも1名の出席が必要です。<br />
            資料は<DocumentLink />から
          </>
        ),
      },
      {
        title: "各種申請を行う",
        children: <>企画内容、食品の取り扱い、電源、レンタル品の申請を行なっていただきます。詳しくは第一回参加団体総会にてご説明します。</>,
      },
      {
        title: "模擬店企画向け面談",
        date: "8/7(金), 8/13(木), 8/17(月), 8/18(火)",
        children: <>申請内容の最終確認と修正をしていただきます。模擬店申請らくらくパックを利用する団体は来ていただく必要はございません。</>,
      },
      {
        title: "事前安全講習会",
        date: "9/16(水)",
        children: (
          <>
            責任者3名に出席していただく必要がございます。また、第二回参加団体総会も付随しての開催となります。資料は<DocumentLink />から
          </>
        ),
      },
      {
        title: "第三回参加団体総会",
        date: "10/7(水)",
        children: (
          <>
            全団体に出席義務があり、責任者のうち少なくとも1名の出席が必要です。<br />
            資料は<DocumentLink />から
          </>
        ),
      },
      {
        title: "工大祭に出店する",
        children: <>楽しみましょう！</>,
      },
    ],
  ],
  general: [
    [
      {
        title: "STEP 1",
        children: (
          <>
            参加説明会に出席！<br />
            説明会は5/13(水)と5/15(金)に実施<br />
            資料は<DocumentLink />から
          </>
        ),
      },
      {
        title: "STEP 2",
        date: "～6/11(木)",
        children: (
          <>
            参加申請をしよう！<br />
            申請期間は5/13(水)から6/11(木)まで<br />
            参加申請のフォームは<FormLink />
          </>
        ),
      },
      {
        title: "STEP 3",
        children: (
          <>
            参加申請完了！<br />
            工大祭実行委員会からの<br />
            申請結果通知を待とう！
          </>
        ),
      },
    ],
    [
      {
        title: "一般企画場所決定会",
        date: "7/1(水)",
        children: <>希望場所が重複している場合、6/26(金)までに通知いたしますので、ご出席ください。</>,
      },
      {
        title: "参加申込金振り込み",
        date: "～7/7(火)",
        children: <>6/19(金)の参加資格通知または7/3(金)の補欠合格発表で合格の通知があった団体の方は7/7(火)までに参加申込金を振り込んでください。</>,
      },
      {
        title: "第一回参加団体総会",
        date: "7/8(水)",
        children: (
          <>
            全団体に出席義務があり、責任者のうち少なくとも1名の出席が必要です。<br />
            資料は<DocumentLink />から
          </>
        ),
      },
      {
        title: "各種申請を行う",
        children: (
          <>
            物品の申請を行なっていただきます。<br />
            詳しくは第一回参加団体総会にてご説明します。
          </>
        ),
      },
      {
        title: "事前安全講習会",
        date: "9/16(水)",
        children: <>責任者のうち1名に出席していただく必要がございます。また、第二回参加団体総会も付随しての開催となります。資料は<DocumentLink />から</>,
      },
      {
        title: "第三回参加団体総会",
        date: "10/7(水)",
        children: (
          <>
            全団体に出席義務があり、責任者のうち少なくとも1名の出席が必要です。<br />
            資料は<DocumentLink />から
          </>
        ),
      },
      {
        title: "工大祭に出展する",
        children: <>楽しみましょう！</>,
      },
    ],
  ],
  stage: [
    [
      {
        title: "STEP 1",
        children: (
          <>
            参加説明会に出席！<br />
            説明会は5/13(水)と5/15(金)に実施<br />
            資料は<DocumentLink />から
          </>
        ),
      },
      {
        title: "STEP 2",
        date: "～6/11(木)",
        children: (
          <>
            参加申請をしよう！<br />
            申請期間は5/13(水)から6/11(木)まで<br />
            参加申請のフォームは<FormLink />
          </>
        ),
      },
      {
        title: "STEP 3",
        children: (
          <>
            参加申請完了！<br />
            工大祭実行委員会からの<br />
            申請結果通知を待とう！
          </>
        ),
      },
    ],
    [
      {
        title: "企画内容決定会",
        date: "7/1(水)",
        children: <>タイムテーブルを決定しますので、ご出席ください。(野外ステージのみに参加する団体は参加される必要はございません。)</>,
      },
      {
        title: "参加申込金振り込み",
        date: "7/7(火)",
        children: <>6/19(金)の参加資格通知または7/3(金)の補欠合格発表で合格の通知があった団体の方は7/7(火)までに参加申込金を振り込んでください。</>,
      },
      {
        title: "第一回参加団体総会",
        date: "7/8(水)",
        children: (
          <>
            全団体に出席義務があり、責任者のうち少なくとも1名の出席が必要です。<br />
            資料は<DocumentLink />から
          </>
        ),
      },
      {
        title: "各種申請を行う",
        children: (
          <>
            物品などの申請を行なっていただきます。<br />
            詳しくは第一回参加団体総会にてご説明します。
          </>
        ),
      },
      {
        title: "第三回参加団体総会",
        date: "10/7(水)",
        children: <>全団体に出席義務があり、責任者のうち少なくとも1名の出席が必要です。資料は<DocumentLink />から</>,
      },
      {
        title: "工大祭に出展する",
        children: <>楽しみましょう！</>,
      },
    ],
  ],
  labo: [
    [
      {
        title: "参加申請を行う",
        date: "4/22(水)～6/11(木)",
        children: (
          <>
            資料は<DocumentLink />！<br />
            参加申請フォームは<FormLink />
          </>
        ),
      },
      {
        title: "第一回資料配布",
        date: "7/1(水)",
        children: (
          <>
            当委員会が資料をお渡しに伺います。<br />
            ご不在の場合はポストに投函させていただきます。
          </>
        ),
      },
      {
        title: "パンフレット原稿",
        date: "7/1(水)～7/31(金)",
        children: <>工大祭のパンフレットに記載する情報を提出していただきます。</>,
      },
      {
        title: "各種申請をする",
        children: <>該当する研究室のかたのみ、必要な申請をしていただきます。</>,
      },
      {
        title: "第二回資料配布",
        date: "10/7(水)",
        children: (
          <>
            当委員会が資料をお渡しに伺います。<br />
            ご不在の場合はポストに投函させていただきます。
          </>
        ),
      },
      {
        title: "工大祭本番！",
        date: "10/10(土)、10/11(日)",
        children: <>楽しみましょう！</>,
      },
    ],
  ],
};

export function TopPageTabs() {
  return (
    <Tab
      queryParam="type"
      queryValues={
        new Map([
          ["模擬店企画", "booth"],
          ["一般企画", "general"],
          ["ステージ企画", "stage"],
          ["研究室企画", "labo"],
        ])
      }
      tabs={
        new Map([
          [
            "模擬店企画",
            <>
              <Heading2 emoji="🕺">参加申請までの流れ</Heading2>
              <Steps steps={topPageData.booth[0]} />
              <Heading2 emoji="📅">工大祭までの流れ</Heading2>
              <Steps steps={topPageData.booth[1]} />
            </>,
          ],
          [
            "一般企画",
            <>
              <Heading2 emoji="🕺">参加申請までの流れ</Heading2>
              <Steps steps={topPageData.general[0]} />
              <Heading2 emoji="📅">工大祭までの流れ</Heading2>
              <Steps steps={topPageData.general[1]} />
            </>,
          ],
          [
            "ステージ企画",
            <>
              <Heading2 emoji="🕺">参加申請までの流れ</Heading2>
              <Steps steps={topPageData.stage[0]} />
              <Heading2 emoji="📅">工大祭までの流れ</Heading2>
              <Steps steps={topPageData.stage[1]} />
            </>,
          ],
          [
            "研究室企画",
            <>
              <Heading2 emoji="📅">工大祭までの流れ</Heading2>
              <Steps steps={topPageData.labo[0]} />
            </>,
          ],
        ])
      }
    />
  );
}
