uniffi::setup_scaffolding!();

#[uniffi::export]
pub fn run_metal_tests() {
    core::run_metal_tests()
}
