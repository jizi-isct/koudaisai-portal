mod domain;

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
}