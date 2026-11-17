// Called to list settings for a mod.
fn pipez_settings_list(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != crate::game::pipez::ID {
        return;
    }
    for (name, label, _desc) in crate::game::pipez::list_settings() {
        let settings = crate::game::pipez::current_settings();
        let value = match name {
            "debug_mode" => settings.debug_mode,
            "show_pipe_connections" => settings.show_pipe_connections,
            "log_transfers" => settings.log_transfers,
            _ => false,
        };
        let status = if value { "ON" } else { "OFF" };
        buttons.push(format!("{}: {}", label, status));
    }
}