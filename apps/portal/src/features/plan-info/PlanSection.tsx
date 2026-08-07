import type { GroupRead } from '@koudaisai/shared-types';
import { Heading1, LoadingScreen } from '@koudaisai/shared-ui';
import { useEffect, useState } from 'react';
import { api } from '@/features/api/api';
import { getPlaceLabel, getProject } from '@/features/api/events26Api';
import { EditPlanInfoModal } from './EditPlanInfoModal';
import { PlanCard } from './PlanCard';
import styles from './PlanSection.module.css';
import type { Project } from './types';

/** 企画を持つ団体種別。press は企画情報を持たない。 */
const PLAN_GROUP_TYPES: GroupRead['type'][] = [
  'booth_project',
  'general_project',
  'stage_project',
  'lab_project',
];

/**
 * 企画情報セクション。
 * 団体 ID がそのまま企画情報API(events26)の企画 ID になるため、`/groups/us` の
 * 結果を projectId として使う。
 */
export function PlanSection() {
  const [group, setGroup] = useState<GroupRead | null>(null);
  const [project, setProject] = useState<Project | null>(null);
  const [placeLabels, setPlaceLabels] = useState<Record<string, string>>({});
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isEditPlanOpen, setIsEditPlanOpen] = useState(false);

  useEffect(() => {
    (async () => {
      const { data: group, error: groupError } = await api.GET('/groups/us');

      if (groupError || !group) {
        setError(
          groupError ? `${groupError}` : '団体情報を取得できませんでした。',
        );
        setIsLoading(false);
        return;
      }

      setGroup(group);

      if (!PLAN_GROUP_TYPES.includes(group.type)) {
        setIsLoading(false);
        return;
      }

      const project = await getProject(group.id);
      setProject(project);
      setIsLoading(false);

      // 場所は階層 ID でしか入っていないので、表示名に引き直す。
      const placeIds = [
        ...new Set(
          project?.occasions
            .map((occasion) => occasion.place)
            .filter((place) => place !== null && place !== undefined) ?? [],
        ),
      ];
      const labels = await Promise.all(placeIds.map(getPlaceLabel));
      setPlaceLabels(
        Object.fromEntries(placeIds.map((id, i) => [id, labels[i]])),
      );
    })().catch((caughtError) => {
      setError(`${caughtError}`);
      setIsLoading(false);
    });
  }, []);

  if (isLoading) {
    return <LoadingScreen />;
  }

  // 企画を持たない団体(取材団体など)には企画セクション自体を出さない。
  if (!group || !PLAN_GROUP_TYPES.includes(group.type)) {
    return null;
  }

  return (
    <>
      <Heading1 emoji="📄">企画情報</Heading1>
      {error ? (
        <p className={styles.message}>{error}</p>
      ) : project ? (
        <>
          <PlanCard
            project={project}
            placeLabels={placeLabels}
            openModal={() => setIsEditPlanOpen(true)}
            disableEdit={false}
          />
          <EditPlanInfoModal
            project={project}
            groupId={group.id}
            isOpen={isEditPlanOpen}
            setOpen={setIsEditPlanOpen}
          />
        </>
      ) : (
        <p className={styles.message}>
          企画情報はまだ公開されていません。公開までしばらくお待ちください。
        </p>
      )}
    </>
  );
}
