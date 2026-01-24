
use crate::models::common::APIResponse;

pub async fn print_tracks(response: &APIResponse) -> Result<(), Box<dyn std::error::Error>>{
    
    if response.recordings.is_empty() {
        return Err("No recordings found".into());
    }

    for recording in response.recordings.iter() {
        println!("-----");
       let artist_name = recording
        .artist_credit
        .as_ref()
        .and_then(|credits| credits.first())
        .map(|ac| ac.name.as_str())
        .unwrap_or("Unknown");

    println!("Artist name: {}", artist_name);
        println!("Recording name: {}", recording.title);
        println!("-----");
    }
    Ok(())
}