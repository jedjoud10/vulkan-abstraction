use ash::vk;
use ash::*;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;

pub unsafe fn create_surface(
    instance: &Instance,
    entry: &Entry,
    window: &Window,
) -> (khr::surface::Instance, vk::SurfaceKHR) {
    let surface = ash_window::create_surface(
        entry,
        instance,
        window.display_handle().unwrap().as_raw(),
        window.window_handle().unwrap().as_raw(),
        None,
    )
    .unwrap();
    log::debug!("Created the Vulkan surface successfully");
    let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);
    log::debug!("Created the Vulkan surface loader successfully");
    (surface_loader, surface)
}
