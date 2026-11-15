// command.rs: compiled as a local function inside native_command.
fn on_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::horizon_essentials as e;

    let mut st = e::State {
        friends: e::parse_friends(&game.native_setting(e::ID, "friends").unwrap_or_default()),
        address: game.native_setting(e::ID, "address").unwrap_or_default(),
        hud_on: game.native_setting(e::ID, "hud").as_deref() != Some("off"),
        corner: game
            .native_setting(e::ID, "corner")
            .unwrap_or_else(|| "top-left".to_string()),
    };
    if let Some(out) = e::handle_command(command, &mut st) {
        *ran = true;
        if out.changed {
            game.native_set_setting(e::ID, "friends", &e::join_friends(&st.friends));
            game.native_set_setting(e::ID, "address", &st.address);
            game.native_set_setting(e::ID, "hud", if st.hud_on { "on" } else { "off" });
            game.native_set_setting(e::ID, "corner", &st.corner);
        }
        for line in out.lines {
            game.native_chat(&line);
        }
    }
}
