use ash::vk::make_version;

pub const APPLICATION_VERSION:  u32 = make_version(1, 0, 0);
pub const ENGINE_VERSION:       u32 = make_version(1, 0, 0);
pub const API_VERSION:          u32 = make_version(1, 0, 0);

pub const WINDOW_TITLE:     &'static str = "Vulkan Tutorial";
pub const WINDOW_HEIGHT:    u32 = 800;
pub const WINDOW_WIDTH:     u32 = 600;

