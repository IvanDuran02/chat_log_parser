use serde_json::Value;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read Json file
    let file_content = fs::read_to_string("src/chat_logs/discord_messages.json")?;

    // Parse the JSON into a generic Value
    let json: Value = serde_json::from_str(&file_content)?;

    let mut total_call_duration = 0;

    // Access specific fields in JSON
    if let Some(messages) = json.get("messages") {
        for message in messages.as_array().unwrap_or(&vec![]) {
            /* if let Some(author) = message.get("author") {
                println!("Author: {}", author);
            } */

            if let Some(content) = message.get("content").and_then(|c| c.as_str()) {
                if let Some(duration) = extract_call_duration(content) {
                    total_call_duration += duration;
                }
            }
        }
    }

    let hours = total_call_duration / 60;
    let minutes = total_call_duration - (hours * 60);

    println!("Total Call Time: {} hours and {} minutes.", hours, minutes);

    Ok(())
}

fn extract_call_duration(content: &str) -> Option<i32> {
    // Check if content is a call
    // Example: "Started a call that lasted 45 minutes."
    let call_pattern = "Started a call that lasted ";
    if let Some(start_index) = content.find(call_pattern) {
        let duration_start = start_index + call_pattern.len();
        if let Some(end_index) = content[duration_start..].find(" minutes.") {
            // Parse the number
            return content[duration_start..duration_start + end_index]
                .parse::<i32>()
                .ok();
        }
    }
    None
}

/* Example Discord Message
{
  "id": "979584595563532320",
  "type": "Default",
  "timestamp": "2022-05-26T23:19:27.384-04:00",
  "timestampEdited": null,
  "callEndedTimestamp": null,
  "isPinned": false,
  "content": "when you come back we can play league",
  "author": {
    "id": "197460641655685132",
    "name": ".popo.",
    "discriminator": "0000",
    "nickname": "popo",
    "color": null,
    "isBot": false,
    "roles": [],
    "avatarUrl": "https://cdn.discordapp.com/avatars/197460641655685132/487fc8b3b97efe2a02f9e141fc015a8a.png?size=512"
  },
  "attachments": [],
  "embeds": [],
  "stickers": [],
  "reactions": [],
  "mentions": [],
  "inlineEmojis": []
}, */
