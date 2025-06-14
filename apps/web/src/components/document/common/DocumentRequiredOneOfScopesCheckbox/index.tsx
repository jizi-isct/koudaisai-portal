"use client";

type DocumentRequiredOneOfScopesCheckboxProps = {
  requiredOneOfScopes: string[],
  setRequiredOneOfScopes: (requiredOneOfScopes: string[]) => void
}

export function DocumentRequiredOneOfScopesCheckbox({
                                                      requiredOneOfScopes,
                                                      setRequiredOneOfScopes
                                                    }: DocumentRequiredOneOfScopesCheckboxProps) {
  const handleScopeChange = (scope: string, allowed: boolean) => {
    if (allowed) {
      setRequiredOneOfScopes(Array.from(new Set(requiredOneOfScopes.concat(scope))))
    } else {
      setRequiredOneOfScopes(requiredOneOfScopes.filter((value) => value !== scope))
    }
  }

  return (
    <label>
      閲覧権限管理(チェックした対象が閲覧可能になります)
      <div>
        <input type="checkbox" checked={requiredOneOfScopes.includes("none")} onChange={e => {
          handleScopeChange("none", e.target.checked)
        }}/>
        <span>非ログイン</span>
      </div>
      <div>
        <input type="checkbox" checked={requiredOneOfScopes.includes("booth")} onChange={e => {
          handleScopeChange("booth", e.target.checked)
        }}/>
        <span>模擬店企画</span>
      </div>
      <div>
        <input type="checkbox" checked={requiredOneOfScopes.includes("general")} onChange={e => {
          handleScopeChange("general", e.target.checked)
        }}/>
        <span>一般企画</span>
      </div>
      <div>
        <input type="checkbox" checked={requiredOneOfScopes.includes("stage")} onChange={e => {
          handleScopeChange("stage", e.target.checked)
        }}/>
        <span>ステージ企画</span>
      </div>
      <div>
        <input type="checkbox" checked={requiredOneOfScopes.includes("labo")} onChange={e => {
          handleScopeChange("labo", e.target.checked)
        }}/>
        <span>研究室企画</span>
      </div>
    </label>
  )
}