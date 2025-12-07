use std::error::Error;
use std::ops::Deref;
use std::path::Path;
use std::{env, fs};

pub struct DailyInput(String);

impl DailyInput {
    pub fn get(day: u32) -> Self {
        cache_or_get_daily_input(day).unwrap_or_else(|err| panic!("Failed to get daily input: {}", err))
    }
}

impl Deref for DailyInput {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for DailyInput {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn cache_or_get_daily_input(day: u32) -> Result<DailyInput, Box<dyn Error>> {
    let cache_path = format!("./.input/day{}.txt", day);

    let input = if Path::new(&cache_path).exists() {
        fs::read_to_string(&cache_path)?
    } else {
        let input = get_daily_input(day)?;
        fs::create_dir_all("./.input")?;
        fs::write(&cache_path, &input)?;
        input
    };

    Ok(DailyInput(input))
}

fn get_daily_input(day: u32) -> Result<String, Box<dyn Error>> {
    let session = env::var("AOC_SESSION")?;

    let mut response = ureq::get(&format!("https://adventofcode.com/2025/day/{}/input", day))
        .header("Cookie", &format!("session={}", session))
        .call()?;

    let mut content = response.body_mut().read_to_string()?;
    content.pop();
    Ok(content)
}
