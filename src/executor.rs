use std::process::{Command as Cmd, Stdio};
use tokio::time::{timeout, Duration};

const COMMAND_TIMEOUT_SECS: u64 = 30;

pub async fn execute(cmd: &str, shell: &str) -> String {
    let trimmed = cmd.trim();

    // Validate based on shell type
    if is_unix_shell(shell) {
        if trimmed.starts_with("sudo ") {
            let has_n_flag = trimmed.contains(" -n ");
            let has_s_flag = trimmed.contains(" -S ");
            let has_echo_pipe = trimmed.contains("echo") && trimmed.contains("|");

            if !has_n_flag && !(has_s_flag && has_echo_pipe) {
                return "error: sudo only allowed with '-n' or '-S' (via 'echo password | sudo -S ...')".to_string();
            }
        }
    }

    let fut = async {
        let (shell_cmd, shell_arg) = get_shell_command(shell);

        match Cmd::new(shell_cmd)
            .arg(shell_arg)
            .arg(cmd)
            .stdin(Stdio::null())
            .output()
        {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let status = out.status.code().unwrap_or(-1);

                if out.status.success() {
                    if stdout.is_empty() && !stderr.is_empty() {
                        stderr
                    } else {
                        stdout
                    }
                } else {
                    let output = if stderr.is_empty() {
                        stdout
                    } else {
                        stderr
                    };
                    format!("error: [exit code: {}] {}", status, output.trim())
                }
            }
            Err(e) => format!("error: {e}"),
        }
    };

    match timeout(Duration::from_secs(COMMAND_TIMEOUT_SECS), fut).await {
        Ok(result) => result,
        Err(_) => format!("error: command timeout after {}s", COMMAND_TIMEOUT_SECS),
    }
}

fn is_unix_shell(shell: &str) -> bool {
    matches!(shell.to_lowercase().as_str(), "sh" | "bash" | "zsh" | "ksh")
}

fn get_shell_command(shell: &str) -> (&str, &str) {
    match shell.to_lowercase().as_str() {
        "powershell" | "pwsh" => ("powershell", "-Command"),
        "cmd" => ("cmd", "/c"),
        _ => ("sh", "-c"),
    }
}