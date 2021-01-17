extern crate osascript;
#[macro_use]
extern crate serde_derive;

use osascript::JavaScript;
use std::env;
use std::fs;

#[derive(Deserialize)]
struct NowPlayingResults {
    artist: String,
    name: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let script = JavaScript::new(
        "
        var spotify = Application(\"Spotify\");
        var track = spotify.currentTrack();
        
        return {
            artist: track.artist(),
            album: track.album(),
            discNumber: track.discNumber(),
            duration: track.duration(),
            played_count: track.playedCount(),
            trackNumber: track.trackNumber(),
            popularity: track.popularity(),
            id: track.id(),
            name: track.name(),
            artworkUrl: track.artworkUrl(),
            albumArtist: track.albumArtist(),
            spotifyUrl: track.spotifyUrl()
        }
    ",
    );

    let rv: NowPlayingResults = script.execute().unwrap();

    let output = format!("{} - {}", rv.name, rv.artist);
    println!("Writing {} to {}", output, path);

    fs::write(path, output).expect("Unable to write file");
}
