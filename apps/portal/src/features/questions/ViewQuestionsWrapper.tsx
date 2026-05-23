import type {UserRead} from "@koudaisai/shared-types";
import {Faq, LoadingScreen, Selector} from "@koudaisai/shared-ui";
import {useEffect, useMemo, useState} from "react";
import {api} from "@/features/api/api";
import {
  questionDataNoLogin,
  questionDataPlanBooth,
  questionDataPlanGeneral,
  questionDataPlanStage,
} from "./questionData";

export function ViewQuestionsWrapper() {
  const [user, setUser] = useState<UserRead | null>(null);
  const [groupType, setGroupType] = useState<"booth" | "general" | "stage" | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      const {data: user, error: userError} = await api.GET("/users/{user_id}", {
        params: {
          path: {
            user_id: "me",
          },
        },
      });

      if (userError || !user) {
        setError(userError ? `${userError}` : "ユーザー情報を取得できませんでした。");
        setIsLoading(false);
        return;
      }

      const {data: group, error: groupError} = await api.GET("/groups/{id}", {
        params: {
          path: {
            id: user.group_id,
          },
        },
      });

      if (groupError || !group) {
        setError(groupError ? `${groupError}` : "団体情報を取得できませんでした。");
        setIsLoading(false);
        return;
      }

      setUser(user);

      if (group.type_plan?.type_booth) {
        setGroupType("booth");
      } else if (group.type_plan?.type_general) {
        setGroupType("general");
      } else if (group.type_plan?.type_stage) {
        setGroupType("stage");
      }

      setIsLoading(false);
    })().catch((caughtError) => {
      setError(`${caughtError}`);
      setIsLoading(false);
    });
  }, []);

  const questionData = useMemo(() => {
    if (groupType === "booth") return questionDataPlanBooth;
    if (groupType === "general") return questionDataPlanGeneral;
    if (groupType === "stage") return questionDataPlanStage;
    return questionDataNoLogin;
  }, [groupType]);

  const [questionType, setQuestionType] = useState(() => Object.keys(questionData)[0]);

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
      <Selector options={Object.keys(questionData)} selectedOption={questionType} setOption={setQuestionType} />
      {items?.map((data, index) => (
        <Faq key={`${questionType}-${index}`} number={index} content={data} />
      ))}
      <p>その他何かご不明などございましたら、当委員会までお気軽にお問い合わせください！</p>
    </>
  );
}
