//! Rotate this app's log the same way as the pNet node.
//!
//! The live file is `~/.pnet/logs/{basename}_rCURRENT.log`. flexi_logger
//! rotates it each day or at 32 MiB and deletes files older than 7 days.

use std::sync::OnceLock;

pub fn init(basename: &str) {
    static HANDLE: OnceLock<flexi_logger::LoggerHandle> = OnceLock::new();
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let dir = std::path::PathBuf::from(home).join(".pnet").join("logs");
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }
    let spec = flexi_logger::FileSpec::default()
        .directory(dir)
        .basename(basename)
        .suffix("log");
    let started = flexi_logger::Logger::try_with_str("info").and_then(|logger| {
        logger
            .log_to_file(spec)
            .rotate(
                flexi_logger::Criterion::AgeOrSize(flexi_logger::Age::Day, 32 * 1024 * 1024),
                flexi_logger::Naming::Timestamps,
                flexi_logger::Cleanup::KeepForDays(7),
            )
            .format(flexi_logger::detailed_format)
            .start()
    });
    if let Ok(handle) = started {
        let _ = HANDLE.set(handle);
    }
}
