use anyhow::Error;
use serde::Deserialize;
use std::fs;
// use std::io;
use std::io::Read;
use std::io::{BufRead, BufReader};
use subprocess::Exec;
use bat::PrettyPrinter;

#[derive(Debug, PartialEq, Deserialize)]
struct CommandSpec {
    command: String,
    args: Option<Vec<String>>
}

fn read_yaml_spec(path: &str) -> Result<CommandSpec,Error> {
    let input = fs::read_to_string(path)?;

    let command_spec: CommandSpec = serde_yaml::from_str(&input)?;
    Ok(command_spec)
}

fn output_stream(stream: impl Read) -> Result<(), Error>{
    
    let buffer_reader = BufReader::new(stream);
    // for (i, line) in buffer_reader.lines().enumerate() {
    //     println!("{}: {}", i, line?);
    // }

    PrettyPrinter::new()
        .input_from_reader(buffer_reader)
        //.language("shell")
        .grid(true)
        .line_numbers(true)
        .paging_mode(bat::PagingMode::Always)
        .print()?;
    Ok(())
}

fn run_command(command_spec: CommandSpec) -> Result<(), Error> {
    if let Some(args) = command_spec.args {
        let stream = Exec::cmd(&command_spec.command)
        .args(&args)
        .stream_stdout()?;
        output_stream(stream)?;
    } else{
        let stream = Exec::cmd(&command_spec.command)
        .stream_stdout()?;
        output_stream(stream)?;
    }

    Ok(())
}

fn main() -> Result<(), Error>{
    let command_spec = read_yaml_spec("command.yaml")?;
    println!("{:#?}", command_spec);
    run_command(command_spec)?;
    Ok(())
}
