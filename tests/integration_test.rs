//! Integration tests for SigmaOS-emacs-config-file.

use sigmaos_emacs_config_file_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
