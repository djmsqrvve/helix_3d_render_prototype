use bevy::prelude::*;
use chrono::Local;
use std::path::PathBuf;

use crate::cli::CliArgs;
use crate::components::*;

/// System that captures screenshots
pub fn screenshot_capture_system(
    mut screenshot_state: ResMut<ScreenshotState>,
    args: Res<CliArgs>,
) {
    if !screenshot_state.capture_requested || screenshot_state.captured {
        return;
    }

    screenshot_state.frame_count += 1;

    if screenshot_state.frame_count < screenshot_state.capture_frame {
        return;
    }

    let path = screenshot_state.capture_path.clone().unwrap_or_else(|| {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        PathBuf::from(format!("screenshot_{}.png", timestamp))
    });

    info!(
        "[SCREENSHOT] Frame {} - Would capture to {:?}",
        screenshot_state.frame_count, path
    );
    info!("[SCREENSHOT] NOTE: Use OS screenshot tools or renderdoc for now");
    info!("[SCREENSHOT] Future: Implement via bevy::render::view::screenshot");

    if let Some(ref test_dir) = args.test_output {
        std::fs::create_dir_all(test_dir).ok();
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let result_file = test_dir.join(format!("test_result_{}.txt", timestamp));
        info!("[TEST] Results would be saved to {:?}", result_file);
    }

    screenshot_state.captured = true;

    if args.screenshot.is_some() || args.render.is_some() {
        info!("[SCREENSHOT] Exiting after capture (use Ctrl+C or close window)");
    }
}

/// Log system diagnostics periodically
pub fn log_system_diagnostics(time: Res<Time>, mut test_results: ResMut<TestResults>) {
    if test_results.start_time.is_none() {
        test_results.start_time = Some(std::time::Instant::now());
    }

    if (time.elapsed_secs() as u32).is_multiple_of(5) && time.delta_secs() > 0.0 {
        let fps = 1.0 / time.delta_secs();
        debug!(
            "[PERF] FPS: {:.1}, Elapsed: {:.1}s",
            fps,
            time.elapsed_secs()
        );
    }
}
