// click.rs: compiled as a local function inside native_settings_click.
fn click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::horizon_essentials as e;
    if mod_id != e::ID {
        return;
    }
    match button {
        0 => {
            let on = game.native_setting(e::ID, "hud").as_deref() != Some("off");
            game.native_set_setting(e::ID, "hud", if on { "off" } else { "on" });
        }
        1 => {
            let cur = game
                .native_setting(e::ID, "corner")
                .unwrap_or_else(|| "top-left".to_string());
            game.native_set_setting(e::ID, "corner", e::next_corner(&cur));
        }
        _ => {}
    }
}
