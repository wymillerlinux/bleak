// enum used when a certain TV app is being used
#[warn(dead_code)]
pub enum ActiveApp {
    Roku,
    Netflix,
    Hulu,
    AmazonPrime,
    Plex,
    Pandora,
    Spotify,
    Crunchyroll,
    Funimation,
    VRV,
    Nothing
}

// enum used to select what kind of TV you are using
// not used atm
#[warn(dead_code)]
pub enum TV {
    Roku,
    Android,
    Samsung,
    Amazon,
}

pub enum TVPower {
    On,
    Off,
}

pub fn match_to_app(text: String) -> ActiveApp {
    let mut result = text.split_whitespace();

    match result.next() {
        Some("Roku") => ActiveApp::Roku,
        Some("Netflix") => ActiveApp::Netflix,
        Some("Hulu") => ActiveApp::Hulu,
        Some("Prime") => ActiveApp::AmazonPrime,
        Some("Plex") => ActiveApp::Plex,
        Some("Pandora") => ActiveApp::Pandora,
        Some("Spotify") => ActiveApp::Spotify,
        Some("Plex") => ActiveApp::Plex,
        _ => ActiveApp::Nothing,
    }
}

pub fn match_to_power_status(text: String) -> TVPower {
    match text.trim() {
        "PowerOn" => TVPower::On,
        _ => TVPower::Off,
    }
}