/// admin ユーザーが変更可能な設定
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    show_occasions_on_portal: bool,
    accept_correction_requests: bool,
}

impl Settings {
    /// 初期化用コンストラクタ
    pub fn new() -> Self {
        Self::default()
    }

    /// db 復元用コンストラクタ
    pub fn restore(show_occasions_on_portal: bool, accept_correction_requests: bool) -> Self {
        Self {
            show_occasions_on_portal,
            accept_correction_requests,
        }
    }

    /// 企画情報の企画実施場所を参加団体に表示するかどうか
    pub fn show_occasions_on_portal(&self) -> bool {
        self.show_occasions_on_portal
    }

    /// 企画情報の企画実施場所を参加団体に表示するかどうかを変更
    pub fn change_show_occasions_on_portal(&mut self, new_value: bool) {
        self.show_occasions_on_portal = new_value;
    }

    /// 企画情報の訂正申請を受け付けるかどうか
    pub fn accept_correction_requests(&self) -> bool {
        self.accept_correction_requests
    }

    /// 企画情報の訂正申請を受け付けるかどうかを変更
    pub fn change_accept_correction_requests(&mut self, new_value: bool) {
        self.accept_correction_requests = new_value;
    }
}

// 新規導入時も既存の訂正申請受付を継続するため、既定値は true とする。
#[allow(clippy::derivable_impls)]
impl Default for Settings {
    fn default() -> Self {
        Self {
            show_occasions_on_portal: false,
            accept_correction_requests: true,
        }
    }
}
