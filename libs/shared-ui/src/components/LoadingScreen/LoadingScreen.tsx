import {Loader} from "../Loader";

export function LoadingScreen() {
  return (
    <div className="flex justify-center items-center py-[5em] h-full w-full flex-col gap-[1em]">
      <Loader/>
      <div>LOADING</div>
    </div>
  )
}
