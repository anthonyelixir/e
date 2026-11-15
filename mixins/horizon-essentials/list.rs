// list.rs: compiled as a local function inside native_mods.
fn list(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "horizon_essentials".into(),
        title: "Horizon Essentials".into(),
        description: "Corner HUD, local friends list and /friend commands.".into(),
        keys: vec![],
    });
}
