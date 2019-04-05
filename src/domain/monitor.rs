#[allow(dead_code)]
pub struct Window {
    label: String,
    name: String,
    monitor_id: String,
    bits_per_pixel: ConfigEntry<u16>,
    width: ConfigEntry<u16>,
    height: ConfigEntry<u16>,
    display_flags: ConfigEntry<u16>,
    display_frequency: ConfigEntry<u16>,
    display_orientation: ConfigEntry<u16>,
    position_x: ConfigEntry<i32>,
    position_y: ConfigEntry<i32>,
}