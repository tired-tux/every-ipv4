use std::fs::File;
use std::io::{self, Write};

fn main() {
    let mut ip_list = Vec::new();

    for q in 1..=253 {
        for w in 1..=253 {
            for e in 1..=253 {
                for r in 1..=253 {
                    let ip = format!("{}.{}.{}.{}", q, w, e, r);

                    let output = std::process::Command::new("ping")
                        .args(&["-c", "1", "-i", "0.2", &ip])
                        .output();

                    match output {
                        Ok(output) if output.status.success() => {
                            ip_list.push(ip);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let mut file = File::create("ip.list").expect("Failed to create ip.list file");

    for ip in ip_list {
        writeln!(file, "{}", ip).expect("Failed to write to ip.list file");
    }

    println!("Execution completed successfully.");
}
