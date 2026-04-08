pub(crate) mod domain;
pub(crate) mod application;

datatest_stable::harness! {
    // email_address
    { test = domain::email_address::test_new,    root = "tests/fixtures/domain", pattern = r"email_address/new/[^/]+\.json" },
    // group_id
    { test = domain::group_id::test_new,         root = "tests/fixtures/domain", pattern = r"group_id/new/[^/]+\.json" },
    // target_specifier
    { test = domain::target_specifier::test_from_str, root = "tests/fixtures/domain", pattern = r"target_specifier/from_str/[^/]+\.json" },
    // approval_request
    { test = domain::approval_request::test_create,     root = "tests/fixtures/domain", pattern = r"approval_request/create/[^/]+\.json" },
    { test = domain::approval_request::test_transition, root = "tests/fixtures/domain", pattern = r"approval_request/transition/[^/]+\.json" },
    // document_category
    { test = domain::document_category::test_register,     root = "tests/fixtures/domain", pattern = r"document_category/register/[^/]+\.json" },
    { test = domain::document_category::test_change_title, root = "tests/fixtures/domain", pattern = r"document_category/change_title/[^/]+\.json" },
    { test = domain::document_category::test_change_emoji, root = "tests/fixtures/domain", pattern = r"document_category/change_emoji/[^/]+\.json" },
    // form
    { test = domain::form::test_register,       root = "tests/fixtures/domain", pattern = r"form/register/[^/]+\.json" },
    { test = domain::form::test_rename,         root = "tests/fixtures/domain", pattern = r"form/rename/[^/]+\.json" },
    { test = domain::form::test_change_summary, root = "tests/fixtures/domain", pattern = r"form/change_summary/[^/]+\.json" },
    // group
    { test = domain::group::test_register,     root = "tests/fixtures/domain", pattern = r"group/register/[^/]+\.json" },
    { test = domain::group::test_rename,       root = "tests/fixtures/domain", pattern = r"group/rename/[^/]+\.json" },
    { test = domain::group::test_update_roles, root = "tests/fixtures/domain", pattern = r"group/update_roles/[^/]+\.json" },
    // membership
    { test = domain::membership::test_from_group_type, root = "tests/fixtures/domain", pattern = r"membership/from_group_type/[^/]+\.json" },
    // password_credentials
    { test = domain::password_credentials::test_new, root = "tests/fixtures/domain", pattern = r"password_credentials/new/[^/]+\.json" },
    // user
    { test = domain::user::test_register,          root = "tests/fixtures/domain", pattern = r"user/register/[^/]+\.json" },
    { test = domain::user::test_rename,            root = "tests/fixtures/domain", pattern = r"user/rename/[^/]+\.json" },
    { test = domain::user::test_status_transition, root = "tests/fixtures/domain", pattern = r"user/status_transition/[^/]+\.json" },
    // actor_ctx
    { test = domain::actor_ctx::test_is_group_type,  root = "tests/fixtures/domain", pattern = r"actor_ctx/is_group_type/[^/]+\.json" },
    { test = domain::actor_ctx::test_is_group_id,    root = "tests/fixtures/domain", pattern = r"actor_ctx/is_group_id/[^/]+\.json" },
    { test = domain::actor_ctx::test_is_user_id,     root = "tests/fixtures/domain", pattern = r"actor_ctx/is_user_id/[^/]+\.json" },
    { test = domain::actor_ctx::test_is_user_nologin,root = "tests/fixtures/domain", pattern = r"actor_ctx/is_user_nologin/[^/]+\.json" },
    // application::authz
    { test = application::authz::test_can_get_all_users,                      root = "tests/fixtures/application", pattern = r"authz/can_get_all_users/[^/]+\.json" },
    { test = application::authz::test_can_get_user_by_id,                     root = "tests/fixtures/application", pattern = r"authz/can_get_user_by_id/[^/]+\.json" },
    { test = application::authz::test_can_update_user,                        root = "tests/fixtures/application", pattern = r"authz/can_update_user/[^/]+\.json" },
    { test = application::authz::test_can_change_m_address_of_the_user,       root = "tests/fixtures/application", pattern = r"authz/can_change_m_address_of_the_user/[^/]+\.json" },
    { test = application::authz::test_can_get_all_groups,                     root = "tests/fixtures/application", pattern = r"authz/can_get_all_groups/[^/]+\.json" },
    { test = application::authz::test_can_get_group_by_id,                    root = "tests/fixtures/application", pattern = r"authz/can_get_group_by_id/[^/]+\.json" },
    { test = application::authz::test_can_create_group,                       root = "tests/fixtures/application", pattern = r"authz/can_create_group/[^/]+\.json" },
    { test = application::authz::test_can_get_form,                           root = "tests/fixtures/application", pattern = r"authz/can_get_form/[^/]+\.json" },
    { test = application::authz::test_can_create_form,                        root = "tests/fixtures/application", pattern = r"authz/can_create_form/[^/]+\.json" },
    { test = application::authz::test_can_update_form,                        root = "tests/fixtures/application", pattern = r"authz/can_update_form/[^/]+\.json" },
    { test = application::authz::test_can_delete_form,                        root = "tests/fixtures/application", pattern = r"authz/can_delete_form/[^/]+\.json" },
    { test = application::authz::test_can_get_all_approval_requests,          root = "tests/fixtures/application", pattern = r"authz/can_get_all_approval_requests/[^/]+\.json" },
    { test = application::authz::test_can_get_group_approval_requests,        root = "tests/fixtures/application", pattern = r"authz/can_get_group_approval_requests/[^/]+\.json" },
    { test = application::authz::test_can_get_approval_request,               root = "tests/fixtures/application", pattern = r"authz/can_get_approval_request/[^/]+\.json" },
    { test = application::authz::test_can_create_approval_request,            root = "tests/fixtures/application", pattern = r"authz/can_create_approval_request/[^/]+\.json" },
    { test = application::authz::test_can_approve_or_reject_approval_request, root = "tests/fixtures/application", pattern = r"authz/can_approve_or_reject_approval_request/[^/]+\.json" },
    { test = application::authz::test_can_close_approval_request,             root = "tests/fixtures/application", pattern = r"authz/can_close_approval_request/[^/]+\.json" },
    { test = application::authz::test_can_delete_approval_request,            root = "tests/fixtures/application", pattern = r"authz/can_delete_approval_request/[^/]+\.json" },
    { test = application::authz::test_can_get_document_category_by_id,        root = "tests/fixtures/application", pattern = r"authz/can_get_document_category_by_id/[^/]+\.json" },
    { test = application::authz::test_can_get_all_document_categories,        root = "tests/fixtures/application", pattern = r"authz/can_get_all_document_categories/[^/]+\.json" },
    { test = application::authz::test_can_create_document_category,           root = "tests/fixtures/application", pattern = r"authz/can_create_document_category/[^/]+\.json" },
    { test = application::authz::test_can_update_document_category,           root = "tests/fixtures/application", pattern = r"authz/can_update_document_category/[^/]+\.json" },
    { test = application::authz::test_can_delete_document_category,           root = "tests/fixtures/application", pattern = r"authz/can_delete_document_category/[^/]+\.json" },
    // application::user
    { test = application::user::test_get_all,           root = "tests/fixtures/application", pattern = r"user/get_all/[^/]+\.json" },
    { test = application::user::test_get_by_id,         root = "tests/fixtures/application", pattern = r"user/get_by_id/[^/]+\.json" },
    { test = application::user::test_update,            root = "tests/fixtures/application", pattern = r"user/update/[^/]+\.json" },
    { test = application::user::test_change_m_address,  root = "tests/fixtures/application", pattern = r"user/change_m_address/[^/]+\.json" },
    // application::form
    { test = application::form::test_get_all,   root = "tests/fixtures/application", pattern = r"form/get_all/[^/]+\.json" },
    { test = application::form::test_get_by_id, root = "tests/fixtures/application", pattern = r"form/get_by_id/[^/]+\.json" },
    { test = application::form::test_create,    root = "tests/fixtures/application", pattern = r"form/create/[^/]+\.json" },
    { test = application::form::test_update,    root = "tests/fixtures/application", pattern = r"form/update/[^/]+\.json" },
    { test = application::form::test_delete,    root = "tests/fixtures/application", pattern = r"form/delete/[^/]+\.json" },
    // application::group
    { test = application::group::test_get_all,      root = "tests/fixtures/application", pattern = r"group/get_all/[^/]+\.json" },
    { test = application::group::test_get_by_id,    root = "tests/fixtures/application", pattern = r"group/get_by_id/[^/]+\.json" },
    { test = application::group::test_create_group, root = "tests/fixtures/application", pattern = r"group/create_group/[^/]+\.json" },
    // application::document_category
    { test = application::document_category::test_get_all,   root = "tests/fixtures/application", pattern = r"document_category/get_all/[^/]+\.json" },
    { test = application::document_category::test_get_by_id, root = "tests/fixtures/application", pattern = r"document_category/get_by_id/[^/]+\.json" },
    { test = application::document_category::test_update,    root = "tests/fixtures/application", pattern = r"document_category/update/[^/]+\.json" },
    { test = application::document_category::test_delete,    root = "tests/fixtures/application", pattern = r"document_category/delete/[^/]+\.json" },
}