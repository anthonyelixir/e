// Runtime for Horizon Essentials (native mod). Std only, no game types, so
// it builds as a plain Cargo library for tests and as the module
// crate::game::horizon_essentials inside Horizon. Module payloads may not use
// inner attributes or `//!` comments, so these are ordinary comments.

pub const ID: &str = "horizon_essentials";
pub const MAX_FRIENDS: usize = 100;
pub const CORNERS: [&str; 4] = ["top-left", "top-right", "bottom-left", "bottom-right"];

// Colours (0xRRGGBB) used by the HUD.
pub const GOLD: u32 = 0xFFAA00;
pub const GREY: u32 = 0xAAAAAA;
pub const BLUE: u32 = 0x78C8FF;

pub struct State {
    pub friends: Vec<String>,
    pub address: String,
    pub hud_on: bool,
    pub corner: String,
}

pub struct Outcome {
    pub lines: Vec<String>,
    pub changed: bool,
}

// ---- validation: names and addresses end up in chat text, keep them strict ----

pub fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 16
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub fn valid_address(addr: &str) -> bool {
    !addr.is_empty()
        && addr.len() <= 64
        && addr
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | ':' | '-' | '_'))
}

pub fn has_friend(friends: &[String], name: &str) -> bool {
    friends.iter().any(|f| f.eq_ignore_ascii_case(name))
}

// ---- storage format: one comma-separated setting string ----

pub fn parse_friends(s: &str) -> Vec<String> {
    s.split(',')
        .map(str::trim)
        .filter(|n| valid_name(n))
        .take(MAX_FRIENDS)
        .map(String::from)
        .collect()
}

pub fn join_friends(friends: &[String]) -> String {
    friends.join(",")
}

pub fn next_corner(current: &str) -> &'static str {
    let i = CORNERS.iter().position(|c| *c == current).unwrap_or(0);
    CORNERS[(i + 1) % CORNERS.len()]
}

// ---- HUD ----

pub fn hud_lines(friends: &[String], pos: Option<(f64, f64, f64)>) -> Vec<(String, u32)> {
    let mut lines = vec![
        ("Horizon Essentials".to_string(), GOLD),
        ("/friend  /essentials".to_string(), GREY),
    ];
    if let Some((x, y, z)) = pos {
        lines.push((
            format!("XYZ {} {} {}", x.floor() as i64, y.floor() as i64, z.floor() as i64),
            GREY,
        ));
    }
    lines.push((format!("Friends: {}", friends.len()), BLUE));
    for f in friends.iter().take(5) {
        lines.push((format!("  {}", f), 0xFFFFFF));
    }
    lines
}

// ---- commands ----

// Returns None when the line isn't one of ours, so the game handles it.
pub fn handle_command(line: &str, st: &mut State) -> Option<Outcome> {
    let line = line.trim().trim_start_matches('/');
    let mut words = line.split_whitespace();
    let cmd = words.next()?.to_ascii_lowercase();
    let args: Vec<&str> = words.collect();
    match cmd.as_str() {
        "friend" => Some(friend_cmd(&args, st)),
        "essentials" => Some(essentials_cmd(&args, st)),
        _ => None,
    }
}

fn say(msg: &str) -> Outcome {
    Outcome {
        lines: vec![msg.to_string()],
        changed: false,
    }
}

fn friend_cmd(args: &[&str], st: &mut State) -> Outcome {
    match args {
        [] | ["list"] => {
            if st.friends.is_empty() {
                say("No friends yet. /friend add <name>")
            } else {
                say(&format!("Friends: {}", st.friends.join(", ")))
            }
        }
        ["add", name] => {
            if !valid_name(name) {
                say("Names are 1-16 letters, digits or _")
            } else if has_friend(&st.friends, name) {
                say(&format!("{} is already a friend", name))
            } else if st.friends.len() >= MAX_FRIENDS {
                say("Friends list is full")
            } else {
                st.friends.push(name.to_string());
                Outcome {
                    lines: vec![format!("Added {}", name)],
                    changed: true,
                }
            }
        }
        ["remove", name] => {
            if !has_friend(&st.friends, name) {
                say(&format!("{} isn't on your list", name))
            } else {
                st.friends.retain(|f| !f.eq_ignore_ascii_case(name));
                Outcome {
                    lines: vec![format!("Removed {}", name)],
                    changed: true,
                }
            }
        }
        ["address", addr] => {
            if !valid_address(addr) {
                say("Use letters, digits and . : - _ only (max 64)")
            } else {
                st.address = addr.to_string();
                Outcome {
                    lines: vec![format!("Your address is now {}", addr)],
                    changed: true,
                }
            }
        }
        // Mods can't send chat as the player, so this prints a line to copy.
        ["invite", name] => {
            if !has_friend(&st.friends, name) {
                say(&format!("{} isn't on your list", name))
            } else if st.address.is_empty() {
                say("Set your address first: /friend address <ip:port>")
            } else {
                say(&format!(
                    "Send this to {}: Join my world! Address: {}",
                    name, st.address
                ))
            }
        }
        _ => say("Usage: /friend [list|add|remove|address|invite] <value>"),
    }
}

fn essentials_cmd(args: &[&str], st: &mut State) -> Outcome {
    match args {
        ["hud"] => {
            st.hud_on = !st.hud_on;
            Outcome {
                lines: vec![format!("HUD {}", if st.hud_on { "on" } else { "off" })],
                changed: true,
            }
        }
        ["corner"] => {
            st.corner = next_corner(&st.corner).to_string();
            Outcome {
                lines: vec![format!("HUD corner: {}", st.corner)],
                changed: true,
            }
        }
        _ => say("Usage: /essentials hud | /essentials corner"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn st() -> State {
        State {
            friends: vec![],
            address: String::new(),
            hud_on: true,
            corner: "top-left".into(),
        }
    }

    #[test]
    fn ignores_other_commands() {
        assert!(handle_command("/gamemode creative", &mut st()).is_none());
        assert!(handle_command("", &mut st()).is_none());
    }

    #[test]
    fn add_remove_list() {
        let mut s = st();
        let o = handle_command("/friend add Alex", &mut s).unwrap();
        assert!(o.changed);
        assert_eq!(s.friends, vec!["Alex"]);
        assert!(!handle_command("friend add alex", &mut s).unwrap().changed);
        assert_eq!(handle_command("/friend", &mut s).unwrap().lines[0], "Friends: Alex");
        assert!(handle_command("/friend remove ALEX", &mut s).unwrap().changed);
        assert!(s.friends.is_empty());
    }

    #[test]
    fn rejects_unsafe_input() {
        let mut s = st();
        assert!(!handle_command("/friend add x;op", &mut s).unwrap().changed);
        assert!(!handle_command("/friend address a/b", &mut s).unwrap().changed);
        assert!(s.friends.is_empty() && s.address.is_empty());
    }

    #[test]
    fn invite_needs_friend_and_address() {
        let mut s = st();
        s.friends.push("Sam".into());
        assert!(handle_command("/friend invite Sam", &mut s).unwrap().lines[0].contains("Set your address"));
        handle_command("/friend address 10.0.0.2:25565", &mut s);
        assert_eq!(
            handle_command("/friend invite Sam", &mut s).unwrap().lines[0],
            "Send this to Sam: Join my world! Address: 10.0.0.2:25565"
        );
        assert!(handle_command("/friend invite Zed", &mut s).unwrap().lines[0].contains("isn't on"));
    }

    #[test]
    fn storage_roundtrip_drops_bad_names() {
        assert_eq!(parse_friends("Alex, Sam,bad name,,Zed"), vec!["Alex", "Sam", "Zed"]);
        let f = vec!["A".to_string(), "B".to_string()];
        assert_eq!(parse_friends(&join_friends(&f)), f);
    }

    #[test]
    fn corners_cycle() {
        assert_eq!(next_corner("top-left"), "top-right");
        assert_eq!(next_corner("bottom-right"), "top-left");
        assert_eq!(next_corner("nonsense"), "top-right");
    }

    #[test]
    fn essentials_toggles() {
        let mut s = st();
        handle_command("/essentials hud", &mut s);
        assert!(!s.hud_on);
        handle_command("/essentials corner", &mut s);
        assert_eq!(s.corner, "top-right");
    }

    #[test]
    fn hud_has_position_and_five_friends_max() {
        let f: Vec<String> = (0..9).map(|i| format!("p{}", i)).collect();
        let l = hud_lines(&f, Some((10.7, 64.0, -3.2)));
        assert!(l.iter().any(|(t, _)| t == "XYZ 10 64 -4"));
        assert!(l.iter().any(|(t, _)| t == "Friends: 9"));
        assert_eq!(l.iter().filter(|(t, _)| t.starts_with("  p")).count(), 5);
        assert_eq!(hud_lines(&[], None).len(), 3);
    }
}
