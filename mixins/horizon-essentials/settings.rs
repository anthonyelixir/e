// settings.rs: compiled as a local function inside native_settings.
fn settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::horizon_essentials as e;
    if mod_id != e::ID {
        return;
    }
    let on = game.native_setting(e::ID, "hud").as_deref() != Some("off");
    let corner = game
        .native_setting(e::ID, "corner")
        .unwrap_or_else(|| "top-left".to_string());
    buttons.push(format!("HUD: {}", if on { "ON" } else { "OFF" }));
    buttons.push(format!("Corner: {}", corner));
}
