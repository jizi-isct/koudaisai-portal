/// admin ユーザーが変更可能な設定
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Settings {
    show_occasions_on_portal: bool,
}

impl Settings {
    /// 初期化用コンストラクタ
    pub fn new() -> Self {
        Self::default()
    }

    /// db 復元用コンストラクタ
    pub fn restore(show_occasions_on_portal: bool) -> Self {
        Self {
            show_occasions_on_portal,
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
}
