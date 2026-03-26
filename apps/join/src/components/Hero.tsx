import styles from "./Hero.module.css";

export function Hero() {
  return (
    <>
      <ClipDefs/>
      <section className={styles.root}>
        <div className={styles.content}>
          <p className={styles.catchCopy}>一緒に工大祭を創りませんか？</p>
          <h1 className={styles.title}>工大祭2026</h1>
          <p className={styles.schedule}>
            テーマ：Sparkle<br/>
            開催日：10/10(土), 10/11(日)
          </p>

          <div className={styles.deadline}>
            参加申込期間：
            <ul>
              <li>
                <span className={styles.noWrap}>
                  模擬店・ステージ・一般:
                </span>
                <span className={styles.noWrap}>
                  5/13(水)〜6/11(木)
                </span>
              </li>
              <li>研究室:5/7(水)～6/18(水)</li>
            </ul>
          </div>
        </div>

      </section>
      <div className={styles.spacer} />
    </>
  )
}

function ClipDefs() {
  return (
    <svg width="0" height="0" style={{ position: "absolute" }}>
      <defs>
        <clipPath id="waveClipWide" clipPathUnits="objectBoundingBox">
          <path d="M0,0 H1 V0.966 C1,0.966,1,0.966,1,0.967 C0.999,0.967,0.998,0.967,0.997,0.968 C0.994,0.969,0.99,0.97,0.986,0.972 C0.976,0.975,0.962,0.979,0.944,0.983 C0.909,0.992,0.861,1,0.807,1 C0.78,1,0.75,0.993,0.717,0.981 C0.696,0.974,0.674,0.965,0.65,0.956 C0.636,0.95,0.621,0.944,0.606,0.939 C0.524,0.908,0.427,0.877,0.311,0.877 C0.195,0.876,0.117,0.887,0.069,0.897 C0.044,0.902,0.027,0.907,0.016,0.911 C0.01,0.913,0.006,0.915,0.004,0.916 C0.002,0.916,0.001,0.917,0.001,0.917 C0.001,0.917,0,0.917,0,0.917 V0" stroke="black"/>
        </clipPath>
        <clipPath id="waveClipNarrow" clipPathUnits="objectBoundingBox">
          <path
            d="M0,0.971 V0 H1 V1 C1,1,1,1,0.999,1 C0.996,1,0.992,1,0.987,1 C0.977,0.999,0.962,0.998,0.943,0.995 C0.903,0.989,0.844,0.978,0.768,0.955 C0.691,0.933,0.635,0.921,0.569,0.917 C0.504,0.913,0.428,0.917,0.312,0.926 C0.195,0.935,0.117,0.946,0.068,0.955 C0.044,0.96,0.027,0.964,0.015,0.967 C0.01,0.968,0.006,0.969,0.003,0.97 C0.002,0.97,0.002,0.971,0.001,0.971"
            stroke="black"/>
        </clipPath>
      </defs>
    </svg>
  );
}