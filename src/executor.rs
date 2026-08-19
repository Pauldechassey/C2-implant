use std::process::Command as Cmd;
use tokio::time::{timeout, Duration};

const COMMAND_TIMEOUT_SECS: u64 = 30;

pub async fn execute(cmd: &str) -> String {
    let fut = async {
        match Cmd::new("sh").arg("-c").arg(cmd).output() { //execute a command in a shell --> switch to "cmd" or "powershell" for Windows
            Ok(out) => String::from_utf8_lossy(&out.stdout).to_string(),
            Err(e) => format!("error: {e}"),
        }
    };

    match timeout(Duration::from_secs(COMMAND_TIMEOUT_SECS), fut).await {
        Ok(result) => result,
        Err(_) => format!("error: command timeout after {}s", COMMAND_TIMEOUT_SECS),
    }
}