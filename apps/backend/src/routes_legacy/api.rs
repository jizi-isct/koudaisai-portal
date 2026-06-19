// 旧 `/api/v2/*` ハンドラは `api_v3` へ移行済みのため撤去した。
// 移行先の無い `plans_info`(外部 API プロキシ)のみを残す。
pub mod plans_info;
