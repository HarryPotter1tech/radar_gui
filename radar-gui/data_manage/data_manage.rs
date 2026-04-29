use chrono;
use std::fs::File;
use std::io::{Error, Read, Write};
fn message_log_file_create(server_name: &str, log_file_name: &mut String) -> Result<(), Error> {
    std::fs::create_dir_all("./radar-gui/logs").unwrap();
    *log_file_name = format!(
        "{}_{}_{}",
        server_name,
        chrono::Local::now().format("%Y-%m-%d %H-%M-%S"),
        "log.txt"
    );
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(&log_file_name)?;
    writeln!(file, "Log file created: {}", log_file_name).unwrap();
    println!("[DataManage]Log file created: {}", log_file_name);
    Ok(())
}
fn message_log_write(
    message: &str,
    server_name: &str,
    log_file_name: &mut String,
) -> Result<(), Error> {
    let mut file = File::options().append(true).open(&log_file_name)?;
    writeln!(file, "{}", message).unwrap();
    Ok(())
}
