// Called to set a setting value for a mod.
fn pipez_setting_set(game: &mut crate::game::Game<'_>, mod_id: &str, setting: &str, value: &str) -> bool {
    if mod_id != crate::game::pipez::ID {
        return false;
    }
    let parsed = value.parse::<bool>().ok();
    if let Some(v) = parsed {
        crate::game::pipez::set_setting(setting, v)
    } else {
        false
    }
}