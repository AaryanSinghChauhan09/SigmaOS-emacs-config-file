//! SigmaOS-emacs-config-file — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_emacs_config_file_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-emacs-config-file: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-emacs-config-file: ok");
}
