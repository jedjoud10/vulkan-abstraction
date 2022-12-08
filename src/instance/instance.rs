use ash::{
    extensions::ext::DebugUtils,
    vk::{self},
    Entry,
};

use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle,
    RawWindowHandle,
};
use std::{ffi::CString, sync::Arc};
use winit::window::Window;

// This is a Vulkan instance that gets loaded in
pub struct Instance {
    // Context related
    pub(crate) entry: Entry,
    pub(crate) instance: ash::Instance,

    // Only enable validation and message logging in debug mode
    #[cfg(debug_assertions)]
    debug_utils: DebugUtils,
    #[cfg(debug_assertions)]
    debug_messenger: vk::DebugUtilsMessengerEXT,
}

impl Instance {
    // Create an instance from a winit window, app name, and engine name
    pub unsafe fn new(
        window: &Window,
        app_name: impl ToString,
        engine_name: impl ToString,
    ) -> Self {
        // Load the loading functions
        let entry = Entry::load().unwrap();

        // Create the app info
        let app_name = CString::new(app_name.to_string()).unwrap();
        let engine_name =
            CString::new(engine_name.to_string()).unwrap();
        let app_info = *vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .api_version(vk::API_VERSION_1_3)
            .application_version(0)
            .engine_version(0)
            .engine_name(&engine_name);

        // Create the debug messenger create info
        #[cfg(debug_assertions)]
        let mut debug_messenger_create_info =
            super::create_debug_messenger_create_info();

        // Get the required instance extensions from the handle
        let mut extension_names_ptrs =
            ash_window::enumerate_required_extensions(
                window.raw_display_handle(),
            )
            .unwrap()
            .to_vec();
        let required_instance_extensions =
            crate::global::required_instance_extensions();
        extension_names_ptrs.extend(
            required_instance_extensions.iter().map(|s| s.as_ptr()),
        );

        // Get the required validation layers
        let required_validation_layers =
            crate::global::required_validation_layers();
        let validation_ptrs = required_validation_layers
            .iter()
            .map(|cstr| cstr.as_ptr())
            .collect::<Vec<_>>();

        // Setup the instance create info (with debug info)
        #[cfg(debug_assertions)]
        let instance_create_info = *vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&validation_ptrs)
            .enabled_extension_names(&extension_names_ptrs)
            .push_next(&mut debug_messenger_create_info);

        // Setup the instance create info (without debug info)
        #[cfg(not(debug_assertions))]
        let instance_create_info = *vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_layer_names(&validation_ptrs)
            .enabled_extension_names(&extension_names_ptrs);

        // Create the instance
        let instance = entry
            .create_instance(&instance_create_info, None)
            .unwrap();
        log::debug!("Created the Vulkan instance successfully");

        // Create the debug utils
        #[cfg(debug_assertions)]
        let debug_utils = DebugUtils::new(&entry, &instance);

        // Create the debug messenger
        #[cfg(debug_assertions)]
        let debug_messenger = debug_utils
            .create_debug_utils_messenger(
                &debug_messenger_create_info,
                None,
            )
            .unwrap();
        log::debug!(
            "Created the Vulkan debug messenger successfully"
        );

        // Drop the cstrings
        drop(required_instance_extensions);
        drop(required_validation_layers);

        Instance {
            entry,
            instance,

            #[cfg(debug_assertions)]
            debug_utils,

            #[cfg(debug_assertions)]
            debug_messenger,
        }
    }

    // Destroy the instance. This should be called when the main context gets dropepd
    pub unsafe fn destroy(&self) {
        #[cfg(debug_assertions)]
        self.debug_utils.destroy_debug_utils_messenger(
            self.debug_messenger,
            None,
        );

        self.instance.destroy_instance(None);
    }
}
