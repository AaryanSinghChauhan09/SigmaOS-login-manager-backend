//! SigmaOS-login-manager-backend — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_login_manager_backend_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-login-manager-backend: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-login-manager-backend: ok");
}
