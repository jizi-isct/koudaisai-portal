use crate::application::ports::clock::Clock;
use crate::application::ports::email::Email;
use crate::application::ports::repositories::approval_request_repo::ApprovalRequestRepo;
use crate::application::ports::repositories::document_category_repo::DocumentCategoryRepo;
use crate::application::ports::repositories::form_repo::FormRepo;
use crate::application::ports::repositories::group_repo::GroupRepo;
use crate::application::ports::repositories::membership_repo::MembershipRepo;
use crate::application::ports::repositories::user_repo::UserRepo;
use crate::application::transaction::Transaction;
use crate::application::user::UserApp;

pub mod approval_request;
pub mod authz;
pub mod document_category;
pub mod error;
pub mod form;
pub mod group;
pub mod ports;
pub mod transaction;
pub mod user;

pub struct Application<
    Tx: Transaction,
    AR: ApprovalRequestRepo<Tx>,
    GR: GroupRepo<Tx>,
    MR: MembershipRepo<Tx>,
    UR: UserRepo<Tx>,
    DCR: DocumentCategoryRepo<Tx>,
    FR: FormRepo,
    C: Clock,
    E: Email,
> {
    _phantom: std::marker::PhantomData<Tx>,
    approval_request_repo: AR,
    group_repo: GR,
    membership_repo: MR,
    user_repo: UR,
    document_category_repo: DCR,
    form_repo: FR,
    clock: C,
    email: E,
}

impl<
    Tx: Transaction,
    AR: ApprovalRequestRepo<Tx>,
    GR: GroupRepo<Tx>,
    MR: MembershipRepo<Tx>,
    UR: UserRepo<Tx>,
    DCR: DocumentCategoryRepo<Tx>,
    FR: FormRepo,
    C: Clock,
    E: Email,
> Application<Tx, AR, GR, MR, UR, DCR, FR, C, E>
{
    pub fn new(
        approval_request_repo: AR,
        group_repo: GR,
        membership_repo: MR,
        user_repo: UR,
        document_category_repo: DCR,
        form_repo: FR,
        clock: C,
        email: E,
    ) -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            approval_request_repo,
            group_repo,
            membership_repo,
            user_repo,
            document_category_repo,
            form_repo,
            clock,
            email,
        }
    }

    pub fn approval_request(&'_ self) -> approval_request::ApprovalRequestApp<'_, Tx, AR, MR, C> {
        approval_request::ApprovalRequestApp::new(
            &self.approval_request_repo,
            &self.membership_repo,
            &self.clock,
        )
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

    pub fn document_category(&'_ self) -> document_category::DocumentCategoryApp<'_, Tx, DCR, C> {
        document_category::DocumentCategoryApp::new(&self.document_category_repo, &self.clock)
    }

    pub fn form(&'_ self) -> form::FormApp<'_, FR, C> {
        form::FormApp::new(&self.form_repo, &self.clock)
    }
}
