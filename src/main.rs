use std::{ env::args };
use tokio::time::{ sleep };
use tokio::process::Command;
use humantime::parse_duration;
use winrt_notification::Toast;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let string_duration: String = args().nth(1).unwrap_or("0s".to_string());

    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("gametimer")
        .text1(format!("Компьютер будет заблокирован через {}", string_duration).as_str())
        .show()
        .unwrap();

    sleep(parse_duration(string_duration.as_str()).unwrap()).await;

    Command::new("rundll32")
        .args(["user32.dll", "LockWorkStation"])
        .spawn()?;

    Ok(())
}
