import type { Time, TimeRange } from './project';

export type PlaceInfo = {
  id: string;
  type?: string;
  displayName: string;
  floor?: string;
};

type ProjectPlaces = {
  occasions: readonly { place?: string }[];
};

/**
 * 一覧 API に含まれない room の `floor` を、使用中の場所だけ個別 API から補完する。
 */
export async function enrichPlaceFloors(
  projects: readonly ProjectPlaces[],
  places: readonly PlaceInfo[],
  getFloor: (placeId: string) => Promise<string | undefined>,
): Promise<PlaceInfo[]> {
  const placesById = new Map(places.map((place) => [place.id, place]));
  const roomIds = new Set(
    projects.flatMap((project) =>
      project.occasions.flatMap(({ place }) =>
        place && placesById.get(place)?.type === 'room' ? [place] : [],
      ),
    ),
  );
  const floorsById = new Map(
    await Promise.all(
      [...roomIds].map(
        async (placeId) => [placeId, await getFloor(placeId)] as const,
      ),
    ),
  );

  return places.map((place) => {
    const floor = floorsById.get(place.id);
    return floor ? { ...place, floor } : place;
  });
}

/**
 * 階層 ID に対応する表示名を 2 階層目から半角スペースで結合する。
 * 末端の場所に `floor` があれば、その表示名の直前へ挿入する。
 */
export function formatPlace(
  placeId: string,
  places: readonly PlaceInfo[],
): string {
  const placesById = new Map(places.map((place) => [place.id, place]));
  const parts = placeId.split('.');
  const labels = parts.slice(1).flatMap((_, index) => {
    const id = parts.slice(0, index + 2).join('.');
    const displayName = placesById.get(id)?.displayName;
    return displayName ? [displayName] : [];
  });
  const floor = placesById.get(placeId)?.floor;

  if (floor && labels.length > 0) {
    labels.splice(labels.length - 1, 0, floor);
  }

  return labels.join(' ');
}

/** 時刻をゼロ埋めした `HH:mm` 形式にする。 */
export function formatTime(time: Time): string {
  return `${String(time.hour).padStart(2, '0')}:${String(time.minute).padStart(2, '0')}`;
}

/**
 * 2 つの時刻を日、時、分の順に比較する。
 *
 * `a` が `b` より前なら負数、同じなら 0、後なら正数を返す。
 */
export function compareTime(a: Time, b: Time): number {
  return a.date - b.date || a.hour - b.hour || a.minute - b.minute;
}

export function compactTimeRanges(timeRanges: Array<TimeRange>): TimeRange {
  return timeRanges.reduce(
    (acc, x) => {
      if (compareTime(x.start, acc.start) < 0) {
        acc.start = x.start;
      }
      if (compareTime(acc.end, x.end) < 0) {
        acc.end = x.end;
      }
      return acc;
    },
    {
      start: { date: 2, hour: 23, minute: 59 },
      end: { date: 1, hour: 0, minute: 0 },
    } satisfies TimeRange,
  );
}
