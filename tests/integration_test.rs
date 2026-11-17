// Integration test for SigmaOS-login-manager-backend
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use login_manager_backend as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
