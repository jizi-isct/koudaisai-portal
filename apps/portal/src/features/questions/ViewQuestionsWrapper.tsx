import type { UserRead } from '@koudaisai/shared-types';
import { Faq, LoadingScreen, Selector } from '@koudaisai/shared-ui';
import { useEffect, useMemo, useState } from 'react';
import { api } from '@/features/api/api';
import {
  questionDataNoLogin,
  questionDataPlanBooth,
  questionDataPlanGeneral,
  questionDataPlanStage,
  questionDataPlanLab,
} from './questionData';

export function ViewQuestionsWrapper() {
  const [user, setUser] = useState<UserRead | null>(null);
  const [groupType, setGroupType] = useState<
    'booth' | 'general' | 'stage' | 'lab' | null
  >(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      const { data: user, error: userError } = await api.GET('/users/me');

      if (userError || !user) {
        setError(
          userError ? `${userError}` : 'ユーザー情報を取得できませんでした。',
        );
        setIsLoading(false);
        return;
      }

      const { data: group, error: groupError } = await api.GET('/groups/us');

      if (groupError || !group) {
        setError(
          groupError ? `${groupError}` : '団体情報を取得できませんでした。',
        );
        setIsLoading(false);
        return;
      }

      setUser(user);

      if (group.type === 'booth_project') {
        setGroupType('booth');
      } else if (group.type === 'general_project') {
        setGroupType('general');
      } else if (group.type === 'stage_project') {
        setGroupType('stage');
      } else if (group.type === 'lab_project') {
        setGroupType('lab');
      }

      setIsLoading(false);
    })().catch((caughtError) => {
      setError(`${caughtError}`);
      setIsLoading(false);
    });
  }, []);

  const questionData = useMemo(() => {
    if (groupType === 'booth') return questionDataPlanBooth;
    if (groupType === 'general') return questionDataPlanGeneral;
    if (groupType === 'stage') return questionDataPlanStage;
    if (groupType === 'lab') return questionDataPlanLab;
    return questionDataNoLogin;
  }, [groupType]);

  const [questionType, setQuestionType] = useState(
    () => Object.keys(questionData)[0],
  );

  useEffect(() => {
    setQuestionType(Object.keys(questionData)[0]);
  }, [questionData]);

  if (isLoading) {
    return <LoadingScreen />;
  }

  if (error) {
    return <p>よくある質問の取得に失敗しました: {error}</p>;
  }

  if (!user) {
    return null;
  }

  const items = questionType ? questionData[questionType] : [];

  return (
    <>
      <Selector
        options={Object.keys(questionData)}
        selectedOption={questionType}
        setOption={setQuestionType}
      />
      {items?.map((data, index) => (
        <Faq key={`${questionType}-${index}`} number={index} content={data} />
      ))}
      <p>
        その他何かご不明などございましたら、当委員会までお気軽にお問い合わせください！
      </p>
    </>
  );
}
