use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;

use crate::components::*;

/// System that reads the GPU adapter info and spawns a text overlay showing it
pub fn display_gpu_info(
    mut commands: Commands,
    adapter_info: Res<RenderAdapterInfo>,
    mut gpu_state: ResMut<GpuInfoState>,
) {
    if gpu_state.info_displayed {
        return;
    }

    let name = &adapter_info.name;
    let driver = &adapter_info.driver;
    let driver_info = &adapter_info.driver_info;
    let backend = format!("{:?}", adapter_info.backend);

    // Detect if software rendering
    let is_software = name.to_lowercase().contains("llvmpipe")
        || name.to_lowercase().contains("lavapipe")
        || name.to_lowercase().contains("swiftshader")
        || format!("{:?}", adapter_info.device_type)
            .to_lowercase()
            .contains("cpu");

    let status_icon = if is_software {
        "🐢 SOFTWARE"
    } else {
        "🚀 GPU"
    };
    let status_color = if is_software {
        Color::srgb(1.0, 0.3, 0.3) // Red for software
    } else {
        Color::srgb(0.3, 1.0, 0.3) // Green for hardware
    };

    let info_text = format!("{} | {} | {} | {}", status_icon, name, driver, backend);

    info!(
        "[RENDER] Adapter: {} ({})",
        name,
        if is_software { "SOFTWARE" } else { "HARDWARE" }
    );
    info!("[RENDER] Driver: {} ({})", driver, driver_info);
    info!("[RENDER] Backend: {}", backend);

    if is_software {
        warn!("[RENDER] ⚠️ Software rendering detected! Ensure your GPU drivers are installed or try forcing a backend with WGPU_BACKEND=gl.");
    }

    // Spawn persistent GPU info overlay at bottom-center
    commands.spawn((
        Text::new(info_text),
        TextColor(status_color),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(4.0),
            left: Val::Percent(25.0),
            right: Val::Percent(25.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        GpuInfoText,
    ));

    gpu_state.info_displayed = true;
}
