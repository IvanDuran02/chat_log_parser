use serde_json::Value;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read Json file
    let file_content = fs::read_to_string("src/chat_logs/alec_logs.json")?;

    // Parse the JSON into a generic Value
    let json: Value = serde_json::from_str(&file_content)?;

    let mut total_messages = 0;
    let mut total_call_duration = 0;
    let mut longest_call = 0;

    // HashMap for counting individual messages
    let mut individual_messages = HashMap::new();

    // Access specific fields in JSON
    if let Some(messages) = json.get("messages").and_then(|m| m.as_array()) {
        for message in messages {
            // Access the "author" field in the current message
            if let Some(author) = message.get("author") {
                // Access the "nickname" field inside "author"
                if let Some(nickname) = author.get("nickname").and_then(|n| n.as_str()) {
                    *individual_messages.entry(nickname).or_insert(0) += 1;
                }
            }

            // Read messages
            if let Some(content) = message.get("content").and_then(|c| c.as_str()) {
                total_messages += 1;
                if let Some(duration) = extract_call_duration(content) {
                    total_call_duration += duration;
                    if duration > longest_call {
                        longest_call = duration;
                    }
                }
            }
        }
    }

    println!("Total Messages: {}", total_messages);
    for (key, value) in &individual_messages {
        println!("{}: {}", key, value);
    }
    println!("Total Call Time: {}", format_time(total_call_duration));
    println!("Longest Call Was: {}", format_time(longest_call));

    Ok(())
}

fn format_time(duration: i32) -> String {
    let hours = duration / 60;
    let minutes = duration % 60;

    format!("{} hours and {} minutes", hours, minutes)
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
