//! Integration tests for SigmaOS-arch-historical-archive.

use sigmaos_arch_historical_archive_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
