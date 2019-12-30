// enum used when a certain TV app is being used
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
}

// enum used to select what kind of TV you are using
pub enum TV {
    Roku,
    Android,
    Samsung,
    Amazon,
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
        _ => ActiveApp::Roku,
    }
}
