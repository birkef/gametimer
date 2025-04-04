use std::{ thread::sleep, env::args, process::Command };
use humantime::parse_duration;
use hide_console::hide_console;
use winrt_notification::Toast;

fn main() {
    hide_console();

    let string_duration: String = args().nth(1).unwrap_or("0s".to_string());

    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("gametimer")
        .text1(format!("Компьютер будет заблокирован через {}", string_duration).as_str())
        .show()
        .unwrap();

    sleep(parse_duration(string_duration.as_str()).unwrap());

    Command::new("rundll32")
        .args(["user32.dll", "LockWorkStation"])
        .spawn()
        .expect("Не удалось заблокировать компьютер");
}
