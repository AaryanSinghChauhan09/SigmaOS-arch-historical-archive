use sigmaos_arch_historical_archive::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_arch_historical_archive::new("sigmaos-arch-historical-archive").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-arch-historical-archive"));
}
