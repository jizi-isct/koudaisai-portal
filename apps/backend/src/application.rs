use crate::application::ports::clock::Clock;
use crate::application::ports::email::Email;
use crate::application::ports::repositories::group_repo::GroupRepo;
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::application::transaction::Transaction;
use crate::application::user::UserApp;

pub mod authz;
pub mod error;
mod group;
pub mod ports;
pub mod user;
mod transaction;

pub struct Application<Tx: Transaction, GR: GroupRepo<Tx>, MR: MembershipRepo<Tx>, UR: UserRepo<Tx>, C: Clock, E: Email> {
    _phantom: std::marker::PhantomData<Tx>,
    group_repo: GR,
    membership_repo: MR,
    user_repo: UR,
    clock: C,
    email: E,
}

impl<Tx: Transaction, GR: GroupRepo<Tx>, MR: MembershipRepo<Tx>, UR: UserRepo<Tx>, C: Clock, E: Email>
    Application<Tx, GR, MR, UR, C, E>
{
    pub fn new(group_repo: GR, membership_repo: MR, user_repo: UR, clock: C, email: E) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            group_repo,
            membership_repo,
            user_repo,
            clock,
            email,
        }
    }

    pub fn group(&'_ self) -> group::GroupApp<'_, Tx, GR, MR, UR, C> {
        group::GroupApp::new(
            &self.group_repo,
            &self.membership_repo,
            &self.user_repo,
            &self.clock,
        )
    }

    pub fn user(&'_ self) -> UserApp<'_, Tx, MR, UR, C> {
        UserApp::new(&self.membership_repo, &self.user_repo, &self.clock)
    }
}
