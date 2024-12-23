use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct DiscordStats {
    total_messages: usize,
    individual_messages: HashMap<String, i32>,
    total_call_duration: i32,
    longest_call: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let discord_stats: DiscordStats = discord_parser("src/chat_logs/ariel_logs.json")?;
    println!("Total Messages: {}", discord_stats.total_messages);
    for (key, value) in &discord_stats.individual_messages {
        println!("{}: {}", key, value);
    }
    calculate_percentage_difference(discord_stats.individual_messages);
    println!(
        "Total Call Time: {}",
        format_time(discord_stats.total_call_duration)
    );
    println!(
        "Longest Call Was: {}",
        format_time(discord_stats.longest_call)
    );

    Ok(())
}

fn discord_parser(json_file: &str) -> Result<DiscordStats, Box<dyn Error>> {
    // Read Json file
    let file_content = fs::read_to_string(json_file)?;

    // Parse the JSON into a generic Value
    let json: Value = serde_json::from_str(&file_content)?;

    let mut total_messages = 0;
    let mut total_call_duration = 0;
    let mut longest_call = 0;

    // HashMap for counting individual messages
    let mut individual_messages: HashMap<String, i32> = HashMap::new();

    // Access specific fields in JSON
    if let Some(messages) = json.get("messages").and_then(|m| m.as_array()) {
        for message in messages {
            // Access the "author" field in the current message
            if let Some(author) = message.get("author") {
                // Access the "nickname" field inside "author"
                if let Some(nickname) = author.get("nickname").and_then(|n| n.as_str()) {
                    *individual_messages.entry(nickname.to_string()).or_insert(0) += 1;
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

    Ok(DiscordStats {
        total_messages,
        individual_messages,
        total_call_duration,
        longest_call,
    })
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

fn calculate_percentage_difference(individual_messages: HashMap<String, i32>) {
    let entries: Vec<_> = individual_messages.iter().collect();

    if entries.len() == 2 {
        let (person1, count1) = entries[0];
        let (person2, count2) = entries[1];

        // Calculate the percentage difference
        let total_messages = *count1 + *count2;
        let percentage1 = (*count1 as f64 / total_messages as f64) * 100.0;
        let percentage2 = (*count2 as f64 / total_messages as f64) * 100.0;

        let diff = (percentage1 - percentage2).abs();

        // Now find who sent more messages
        if count1 > count2 {
            println!(
                "{} sent {:.2}% more messages than {}!",
                person1, diff, person2
            );
        } else if count2 > count1 {
            println!(
                "{} sent {:.2}% more messages than {}!",
                person2, diff, person1
            );
        }
    }
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
