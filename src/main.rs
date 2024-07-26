use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::metadata;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source path in the format pod-name:/path/to/remote/file
    src: String,

    /// Destination path in the format /path/to/local/file
    dest: String,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    let src = &args.src;
    let dest = &args.dest;

    // Extract the file name from the destination path
    let file_name = Path::new(dest).file_name().unwrap().to_string_lossy();

    // Get the size of the remote file
    let remote_size = get_remote_file_size(src).expect("Failed to get remote file size");

    // Create a progress bar
    let pb = ProgressBar::new(remote_size);
    pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "{{spinner:.green}} [{{elapsed_precise}}] {} [{{bar:40.cyan/blue}}] {{bytes}}/{{total_bytes}} ({{eta}})",
                    file_name
                ))
                .progress_chars("#>-"),
        );

    // Start the oc cp command
    let mut child = Command::new("oc")
        .arg("cp")
        .arg(src)
        .arg(dest)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start oc cp");

    // Spawn a thread to update the progress bar based on the file size
    let pb_clone = pb.clone();
    let dest_clone = dest.to_string();
    thread::spawn(move || {
        loop {
            // Get the current size of the destination file
            let size = metadata(&dest_clone).map(|m| m.len()).unwrap_or(0);

            // Update the progress bar
            pb_clone.set_position(size);

            // Break the loop if the download is complete
            if size >= remote_size {
                break;
            }

            // Sleep for a short duration before checking the size again
            thread::sleep(Duration::from_millis(500));
        }
        pb_clone.finish_with_message("Download complete");
    });

    // Wait for the command to complete
    let status = child.wait().expect("Failed to wait on child");
    if !status.success() {
        eprintln!("oc cp command failed");
    }
}

fn get_remote_file_size(remote_path: &str) -> Option<u64> {
    // Extract the pod name and the file path
    let parts: Vec<&str> = remote_path.splitn(2, ':').collect();
    if parts.len() != 2 {
        return None;
    }
    let pod = parts[0];
    let file_path = parts[1];

    // Run the oc exec command to get the file size
    let output = Command::new("oc")
        .arg("exec")
        .arg(pod)
        .arg("--")
        .arg("stat")
        .arg("--format=%s")
        .arg(file_path)
        .output()
        .expect("Failed to execute oc exec command");

    if !output.status.success() {
        return None;
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.trim().parse().ok()
}
