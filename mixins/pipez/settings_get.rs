// Called to get a setting value for a mod.
fn pipez_setting_get(game: &mut crate::game::Game<'_>, mod_id: &str, setting: &str) -> Option<String> {
    if mod_id != crate::game::pipez::ID {
        return None;
    }
    crate::game::pipez::get_setting(setting).map(|v| v.to_string())
}