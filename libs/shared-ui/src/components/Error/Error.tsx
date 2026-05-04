export function Error({error}: { error: Error }) {
  return (
    <div className="flex justify-center items-center py-[5em] h-full w-full flex-col gap-[1em]">
      <div>⚠️</div>
      <div>ERROR</div>
      <div>{error.message}</div>
    </div>
  )
}
