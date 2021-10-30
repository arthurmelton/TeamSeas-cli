use termion::color;
use serde_json::Value;
use curl::easy::Easy;

fn get_cont(url:&str) -> Value {
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url(url).unwrap();
    let _output = easy.custom_request("GET");
    let mut transfer = easy.transfer();
    transfer
        .write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })
        .unwrap();
    transfer.perform().unwrap();
    drop(transfer);

    serde_json::from_str(&dst.iter().map(|&c| c as char).collect::<String>().as_str()).unwrap()
}

fn make_box(title:&str, left_title:&str, left_cont:&str, right_title:&str, right_cont:&str, longest_line:usize, total_length:usize, left_length:Vec<usize>, right_length:Vec<usize>) -> String {
    format!("╭{}╮\n| {}{}{} |\n| {}{}{}     {}{}{} |\n| {} |\n| {} |\n| {} |\n| {} |\n| {} |\n╰{}╯", "-".repeat(longest_line*2+5+2), " ".repeat((((longest_line*2+5-total_length-7)as f32)/2.0).floor() as usize) ,title, " ".repeat((((longest_line*2+5-total_length-7)as f32)/2.0).ceil() as usize), " ".repeat((((longest_line-6) as f32)/2.0).floor() as usize) ,left_title, " ".repeat((((longest_line-6) as f32)/2.0).ceil() as usize), " ".repeat((((longest_line-4) as f32)/2.0).floor() as usize), right_title, " ".repeat((((longest_line-4) as f32)/2.0).ceil() as usize), format!("{}{}{}     {}{}{}", " ".repeat((((longest_line-left_length[0]) as f32)/2.0).floor() as usize), left_cont.lines().nth(0).unwrap().trim(), " ".repeat((((longest_line-left_length[0]) as f32)/2.0).ceil() as usize), " ".repeat((((longest_line-right_length[0]) as f32)/2.0).floor() as usize), right_cont.lines().nth(0).unwrap(), " ".repeat((((longest_line-right_length[0]) as f32)/2.0).ceil() as usize)), format!("{}{}{}     {}{}{}", " ".repeat((((longest_line-left_length[1]) as f32)/2.0).floor() as usize), left_cont.lines().nth(1).unwrap().trim(), " ".repeat((((longest_line-left_length[1]) as f32)/2.0).ceil() as usize), " ".repeat((((longest_line-right_length[1]) as f32)/2.0).floor() as usize), right_cont.lines().nth(1).unwrap(), " ".repeat((((longest_line-right_length[1]) as f32)/2.0).ceil() as usize)), format!("{}{}{}     {}{}{}", " ".repeat((((longest_line-left_length[2]) as f32)/2.0).floor() as usize), left_cont.lines().nth(2).unwrap().trim(), " ".repeat((((longest_line-left_length[2]) as f32)/2.0).ceil() as usize), " ".repeat((((longest_line-right_length[2]) as f32)/2.0).floor() as usize), right_cont.lines().nth(2).unwrap(), " ".repeat((((longest_line-right_length[2]) as f32)/2.0).ceil() as usize)), format!("{}{}{}     {}{}{}", " ".repeat((((longest_line-left_length[3]) as f32)/2.0).floor() as usize), left_cont.lines().nth(3).unwrap().trim(), " ".repeat((((longest_line-left_length[3]) as f32)/2.0).ceil() as usize), " ".repeat((((longest_line-right_length[3]) as f32)/2.0).floor() as usize), right_cont.lines().nth(3).unwrap(), " ".repeat((((longest_line-right_length[3]) as f32)/2.0).ceil() as usize)), format!("{}{}{}     {}{}{}", " ".repeat((((longest_line-left_length[4]) as f32)/2.0).floor() as usize), left_cont.lines().nth(4).unwrap().trim(), " ".repeat((((longest_line-left_length[4]) as f32)/2.0).ceil() as usize), " ".repeat((((longest_line-right_length[4]) as f32)/2.0).floor() as usize), right_cont.lines().nth(4).unwrap(), " ".repeat((((longest_line-right_length[4]) as f32)/2.0).ceil() as usize)), "-".repeat(longest_line*2+5+2))
}
fn main() {
    let total = get_cont(&"https://tscache.com/donation_total.json")["count"].clone();
    let recent = get_cont(&"https://tscache.com/lb_recent.json");
    let mut longest_line = 0;
    let mut left_cont:String = "".to_string();
    let mut left_length = Vec::new();
    for i in 0..5 {
        left_cont.push_str(format!("{}{}{}: {}\n", color::Fg(color::LightCyan), recent["recent"][i]["name"].to_string().trim_matches('\"'), color::Fg(color::White), recent["recent"][i]["pounds"].to_string().trim_matches('\"')).as_str());
        if longest_line < format!("{}: {}", recent["recent"][i]["name"].to_string().trim_matches('\"'), recent["recent"][i]["pounds"].to_string().trim_matches('\"')).len() {
            longest_line = format!("{}: {}", recent["recent"][i]["name"].to_string().trim_matches('\"'), recent["recent"][i]["pounds"].to_string().trim_matches('\"')).len();
        }
        left_length.push(format!("{}: {}", recent["recent"][i]["name"].to_string().trim_matches('\"'), recent["recent"][i]["pounds"].to_string().trim_matches('\"')).len());
    }
    let mut right_length = Vec::new();
    let mut right_cont:String = "".to_string();
    for i in 0..5 {
        right_cont.push_str(format!("{}{}{}: {}\n", color::Fg(color::LightCyan), recent["most"][i]["name"].to_string().trim_matches('\"'), color::Fg(color::White), recent["most"][i]["pounds"].to_string().trim_matches('\"')).as_str());
        if longest_line < format!("{}: {}", recent["most"][i]["name"].to_string().trim_matches('\"'), recent["most"][i]["pounds"].to_string().trim_matches('\"')).len() {
            longest_line = format!("{}: {}", recent["most"][i]["name"].to_string().trim_matches('\"'), recent["most"][i]["pounds"].to_string().trim_matches('\"')).len();
        }
        right_length.push(format!("{}: {}", recent["most"][i]["name"].to_string().trim_matches('\"'), recent["most"][i]["pounds"].to_string().trim_matches('\"')).len());
    }
    println!("{}", make_box(format!("{}{}{}{}", color::Fg(color::Blue), "Total: " ,total.to_string().trim_matches('\"'), color::Fg(color::White)).as_str(), format!("{}{}{}", color::Fg(color::Blue), "Recent", color::Fg(color::White)).as_str(), left_cont.as_str(), format!("{}{}{}", color::Fg(color::Blue), "Most", color::Fg(color::White)).as_str(), right_cont.as_str(), longest_line as usize, total.to_string().trim_matches('\"').len(), left_length, right_length));
}