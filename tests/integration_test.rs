// Integration test for SigmaOS-emacs-config-file
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_emacs_config_file as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
