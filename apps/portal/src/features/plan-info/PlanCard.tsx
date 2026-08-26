import styles from './PlanCard.module.css';
import { PlanIcon } from './PlanIcon';
import type { Occasion, Project, ProjectType } from './types';

type Props = {
  project: Project;
  /** 場所の階層 ID から表示名への対応。未解決の ID はそのまま表示する。 */
  placeLabels: Record<string, string>;
  openModal: () => void;
  disableEdit: boolean;
};

// typeのラベル変換マップ
const typeLabels: Record<ProjectType, string> = {
  'food-stall': '模擬店',
  general: '一般企画',
  laboratory: '研究室企画',
  stage: 'ステージ',
};

const foodStallTagLabels: Record<string, string> = {
  'main.rice': 'ご飯もの',
  'main.noodle_flour': '麺・粉もの',
  'main.skewer_grill': '串・焼きもの',
  'main.snack': 'スナック',
  'main.soup': 'スープ',
  'main.world': '世界の料理',
  'sweet.japanese': '和菓子',
  'sweet.western': '洋菓子',
  'sweet.cold': '冷たいスイーツ',
  'sweet.snack': 'スナック',
  'sweet.drink': 'ドリンク',
  'sweet.world': '世界のスイーツ',
  drink: 'ドリンク',
};

const generalTagLabels: Record<string, string> = {
  experience: '体験',
  display: '展示',
  performance: 'パフォーマンス',
  food: '飲食',
  lecture: '講演',
};

/** 企画のタグを表示用の文字列に均す。模擬店のみ tag/tag2 の 2 階層を持つ。 */
function toTagLabels(project: Project): string[] {
  if (project.type === 'food-stall') {
    return project.tag.map((tag) =>
      'tag2' in tag
        ? (foodStallTagLabels[`${tag.tag}.${tag.tag2}`] ?? `${tag.tag2}`)
        : (foodStallTagLabels[tag.tag] ?? tag.tag),
    );
  }

  if (project.type === 'general') {
    return project.tag.map((tag) => generalTagLabels[tag] ?? tag);
  }

  return [];
}

function formatTime(hour: number, minute: number): string {
  return `${hour}:${`${minute}`.padStart(2, '0')}`;
}

/** 開催予定を「8/2 10:00〜16:00」の形にする。日付をまたぐ場合は終了側にも日付を出す。 */
function formatOccasion(occasion: Occasion): string {
  const { start, end } = occasion.timeRange;

  const startLabel = start.date === 1 ? '10/10' : '10/11';

  const startTime = formatTime(start.hour, start.minute);
  const endTime = formatTime(end.hour, end.minute);

  if (start.date === end.date) {
    return `${startLabel} ${startTime}〜${endTime}`;
  }

  const endLabel = end.date === 1 ? '10/10' : '10/11';
  return `${startLabel} ${startTime}〜${endLabel} ${endTime}`;
}

export function PlanCard({
  project,
  placeLabels,
  openModal,
  disableEdit,
}: Props) {
  const tagLabels = toTagLabels(project);

  return (
    <div className={styles.card}>
      <div className={styles.header}>
        <PlanIcon projectId={project.id} />
        <div className={styles.headerText}>
          <h1>
            {project.projectName}
            {project.isRecommended && (
              <span
                className={`${styles.tag} ${styles.gold}`}
                title="おすすめ企画"
              >
                ⭐️
              </span>
            )}
            {project.isChildFriendly && (
              <span
                className={`${styles.tag} ${styles.cyan}`}
                title="子供向け企画"
              >
                👦
              </span>
            )}
            {project.type === 'laboratory' && project.isTour && (
              <span
                className={`${styles.tag} ${styles.green}`}
                title="研究室ツアー参加企画"
              >
                🔬
              </span>
            )}
          </h1>
          <h4>
            {typeLabels[project.type]}
            <span className={styles.groupName}>{project.groupName}</span>
            <span className={styles.planId}>({project.id})</span>
          </h4>
        </div>
      </div>
      <p>{project.description}</p>

      {tagLabels.length > 0 && (
        <ul className={styles.tagList}>
          {tagLabels.map((label, index) => (
            <li key={index} className={styles.tagListItem}>
              {label}
            </li>
          ))}
        </ul>
      )}

      <dl className={styles.occasions}>
        <dt>開催予定</dt>
        {project.occasions.length === 0 ? (
          <dd>未定</dd>
        ) : (
          project.occasions.map((occasion, index) => (
            <dd key={index}>
              {formatOccasion(occasion)}
              {occasion.place && (
                <span className={styles.place}>
                  {placeLabels[occasion.place] ?? occasion.place}
                </span>
              )}
            </dd>
          ))
        )}
      </dl>

      <button
        type="button"
        className={styles.editButton}
        onClick={openModal}
        disabled={disableEdit}
      >
        訂正する
      </button>
    </div>
  );
}
