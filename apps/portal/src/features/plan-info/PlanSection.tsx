import type { GroupRead } from '@koudaisai/shared-types';
import { Heading1, LoadingScreen } from '@koudaisai/shared-ui';
import {
  QueryClient,
  QueryClientProvider,
  useQueries,
} from '@tanstack/react-query';
import { useMemo, useState } from 'react';
import { $api, $events26Api } from '@/features/api/api';
import { EditPlanInfoModal } from './EditPlanInfoModal';
import { PlanCard } from './PlanCard';
import styles from './PlanSection.module.css';

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
  const [queryClient] = useState(() => new QueryClient());

  return (
    <QueryClientProvider client={queryClient}>
      <PlanSectionContent />
    </QueryClientProvider>
  );
}

function PlanSectionContent() {
  const [isEditPlanOpen, setIsEditPlanOpen] = useState(false);

  const { data: group, isLoading: isGroupLoading } = $api.useQuery(
    'get',
    '/groups/us',
  );
  const hasPlan = Boolean(group && PLAN_GROUP_TYPES.includes(group.type));

  const {
    data: occasionsSetting,
    error: occasionsSettingError,
    isLoading: isOccasionsSettingLoading,
  } = $api.useQuery(
    'get',
    '/settings/show-occasions-on-portal',
    {},
    { enabled: hasPlan },
  );
  const {
    data: correctionsSetting,
    error: correctionsSettingError,
    isLoading: isCorrectionsSettingLoading,
  } = $api.useQuery(
    'get',
    '/settings/accept-correction-requests',
    {},
    { enabled: hasPlan },
  );
  const {
    data: project,
    error: projectError,
    isLoading: isProjectLoading,
  } = $events26Api.useQuery(
    'get',
    '/v1/projects/{projectId}',
    {
      params: { path: { projectId: group?.id ?? '' } },
    },
    { enabled: hasPlan, retry: false },
  );

  const showOccasionsOnPortal =
    occasionsSetting?.show_occasions_on_portal ?? false;
  const acceptCorrectionRequests =
    correctionsSetting?.accept_correction_requests ?? true;

  // 場所は階層 ID でしか入っていないので、表示名に引き直す。
  const placeIds = useMemo(
    () => [
      ...new Set(
        project?.occasions
          .map((occasion) => occasion.place)
          .filter((place) => place !== null && place !== undefined) ?? [],
      ),
    ],
    [project],
  );
  const placeQueries = useQueries({
    queries: placeIds.map((placeId) =>
      $events26Api.queryOptions(
        'get',
        '/v1/places/{placeId}',
        { params: { path: { placeId } } },
        { enabled: showOccasionsOnPortal },
      ),
    ),
  });
  const placeLabels = Object.fromEntries(
    placeIds.map((placeId, index) => [
      placeId,
      placeQueries[index]?.data?.displayName ?? placeId,
    ]),
  );

  const arePlacesLoading =
    showOccasionsOnPortal && placeQueries.some((query) => query.isLoading);
  const isLoading =
    isGroupLoading ||
    (hasPlan &&
      (isOccasionsSettingLoading ||
        isCorrectionsSettingLoading ||
        isProjectLoading ||
        arePlacesLoading));

  const settingsUnavailable =
    hasPlan &&
    !isOccasionsSettingLoading &&
    !isCorrectionsSettingLoading &&
    (occasionsSettingError ||
      correctionsSettingError ||
      !occasionsSetting ||
      !correctionsSetting);
  const unexpectedEvents26Error = [
    projectError,
    ...placeQueries.map((query) => query.error),
  ].find((queryError) => queryError instanceof Error);
  const error = settingsUnavailable
    ? '企画情報の設定を取得できませんでした。'
    : unexpectedEvents26Error
      ? `${unexpectedEvents26Error}`
      : null;

  if (isLoading) {
    return <LoadingScreen />;
  }

  // 企画を持たない団体(取材団体など)には企画セクション自体を出さない。
  if (!group || !hasPlan) {
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
            showOccasions={showOccasionsOnPortal}
            openModal={() => setIsEditPlanOpen(true)}
            disableEdit={!acceptCorrectionRequests}
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
