use std::sync::Once;
pub(crate) const REFERENCE_MID: &str = "mc_sweden.mid";
pub(crate) const REFERENCE_UNE_MID: &str = "never_existing.mid";

static INIT: Once = Once::new();

pub fn log_setup() {
    INIT.call_once(|| {
        env_logger::builder()
            .format_timestamp(None)
            .format_module_path(false)
            .format_target(false)
            .is_test(true)
            .init();
        log::info!("Log4Test Initialized...");
    });
}
