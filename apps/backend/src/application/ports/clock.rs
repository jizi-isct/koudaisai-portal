pub trait Clock {
    fn now(&self) -> chrono::DateTime<chrono::Utc>;
}

/// 参照に対する転送実装。
///
/// ドメイン側のいくつかの API(`PasswordCredentials::{new,change}` など)は
/// `clock: C` を値で受け取るため，`&ConcreteClock` をそのまま渡せるように
/// `&C` にも `Clock` を実装しておく。これにより各 `Clock` 実装ごとに
/// `impl Clock for &Foo` を手書きする必要がなくなる。
impl<C: Clock + ?Sized> Clock for &C {
    fn now(&self) -> chrono::DateTime<chrono::Utc> {
        <C as Clock>::now(*self)
    }
}
