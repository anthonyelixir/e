// hud.rs: compiled as a local function inside native_hud.
fn hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::horizon_essentials as e;
    use horizon_client::extras::Corner;

    if !game.native_on(e::ID) || !game.native_playing() {
        return;
    }
    if game.native_setting(e::ID, "hud").as_deref() == Some("off") {
        return;
    }
    let corner = match game
        .native_setting(e::ID, "corner")
        .unwrap_or_default()
        .as_str()
    {
        "top-right" => Corner::TopRight,
        "bottom-left" => Corner::BottomLeft,
        "bottom-right" => Corner::BottomRight,
        _ => Corner::TopLeft,
    };
    let friends = e::parse_friends(&game.native_setting(e::ID, "friends").unwrap_or_default());
    let pos = game
        .native_player()
        .map(|p| (p.x as f64, p.y as f64, p.z as f64));
    let lines = e::hud_lines(&friends, pos);

    let mut w: f32 = 0.0;
    for (t, _) in &lines {
        w = w.max(game.native_text_width(t));
    }
    let w = w + 6.0;
    let h = lines.len() as f32 * 10.0 + 4.0;
    let (x, y) = corners.place(corner, w, h);
    game.native_fill(batch, [x, y, w, h], 0x7800_0000);
    for (i, (t, rgb)) in lines.iter().enumerate() {
        game.native_text(batch, t, [x + 3.0, y + 2.0 + i as f32 * 10.0], *rgb, true);
    }
}
