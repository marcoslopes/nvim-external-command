use std::io::Write;
use std::os::unix::net::UnixStream;
use std::fs;
use clap::{Parser, Subcommand};
use rmpv::Value;

#[derive(Parser, Debug)]
struct Task {
    /// where to save the sockets
    #[arg(short, long, default_value = "/tmp")]
    sockets_path: String,

    /// socket name prefix, try to avoid colisions
    #[arg(short, long, default_value = "nvim-ec.")]
    tmp_prefix: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Exec {
        #[arg(short, long)]
        command: String,
    },
    Theme {
        #[arg(short, long)]
        name: String,
    },
}

/// https://github.com/msgpack-rpc/msgpack-rpc/blob/master/spec.md#request-message
///
/// [type, msgid, method, params]
/// type: The message type, must be the integer zero (0) for "Request" messages.
/// msgid: A 32-bit unsigned integer number. This number is used as a sequence number. The server's response to the "Request" will have the same msgid.
/// method: A string which represents the method name.
/// params: An array of the function arguments. The elements of this array are arbitrary objects.
fn request(command: &str) -> Vec<u8> {
    let request = Value::Array(vec![
        Value::from(0),
        Value::from(1),
        Value::from("nvim_command"),
        Value::Array(vec![
            Value::from(command)
        ])
    ]);
    let mut buf = Vec::new();
    rmpv::encode::write_value(&mut buf, &request).expect("could not encode msgpack request");
    buf
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let task = Task::parse();
    let filename_prefix = format!("{}/{}", task.sockets_path, task.tmp_prefix);

    let command = match task.command {
        Commands::Theme { ref name } => {
            format!("colorscheme {name}")
        },
        Commands::Exec { command } => {
            command
        },
    };

    // create msgpack rpc request
    let buf = request(&command);

    // find nvim sockets
    for entry in fs::read_dir(task.sockets_path)? {
        let path = entry?.path();
        if path.to_str().filter(|s| s.starts_with(&filename_prefix)).is_some() {
            let mut stream = UnixStream::connect(path)?;
            stream.write_all(&buf)?;
        }
    }
    Ok(())
}
