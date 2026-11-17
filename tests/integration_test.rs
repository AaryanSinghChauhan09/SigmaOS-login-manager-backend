//! Integration tests for SigmaOS-login-manager-backend.

use sigmaos_login_manager_backend_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
