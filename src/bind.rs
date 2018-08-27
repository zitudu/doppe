use clap::{App, Arg, ArgMatches, SubCommand};
use command::{Command, CommandResult};
use std;
use std::os::unix::process::CommandExt;
use std::process::Command as Exec;

pub struct Bind {}
impl Bind {
    fn publish_port(&self, port: &str) -> String {
        if let Ok(port) = port.parse::<u16>() {
            format!("-L {port}:localhost:{port}", port = port)
        } else {
            String::with_capacity(0)
        }
    }
}

impl Command for Bind {
    fn name(&self) -> &str {
        "bind"
    }
    fn sub_command(&self) -> App {
        SubCommand::with_name(self.name())
            .version("0.0.1")
            .author("zitudu <zitudu@pronmail.com>")
            .about("Bind remote resources on to local")
            .arg(
                Arg::with_name("port")
                    .help("Bind one port")
                    .short("p")
                    .long("port")
                    .value_name("PORT")
                    .multiple(true)
                    .number_of_values(1),
            )
            .arg(
                Arg::with_name("port-list")
                    .help("Bind one or more ports")
                    .short("P")
                    .long("port-list")
                    .takes_value(true)
                    .value_name("PORT_LIST")
                    .use_delimiter(true)
                    .value_delimiter(","),
            )
            .arg(
                Arg::with_name("shell")
                    .help("Run shell")
                    .short("s")
                    .long("shell")
                    .takes_value(true)
                    .value_name("SHELL")
                    .default_value("/bin/bash"),
            )
            .arg(Arg::with_name("host").value_name("HOST").required(true))
    }
    fn handle(&self, matches: &ArgMatches) -> CommandResult {
        if matches.occurrences_of("port") + matches.occurrences_of("port-list") > 0 {
            let mut ports = Vec::<String>::new();
            if let Some(mut ports_str) = matches.values_of("port") {
                while let Some(port) = ports_str.next() {
                    ports.push(self.publish_port(port));
                }
            }
            if let Some(mut ports_str) = matches.values_of("port-list") {
                while let Some(port) = ports_str.next() {
                    ports.push(self.publish_port(port));
                }
            }
            let echo = format!("echo 'Binding ports {:?}'", ports);
            let shell = if matches.occurrences_of("shell") != 0 {
                format!("{};{}", &echo, matches.value_of("shell").unwrap())
            } else {
                String::from(":")
            };
            Exec::new("ssh")
                .arg("-t")
                .args(&ports)
                .arg(matches.value_of("host").unwrap())
                // .arg("echo 1111")
                .arg(format!("{shell};{echo};read -p 'press any key to exit...' -n 1",echo=echo, shell=shell))
                .stdin(std::process::Stdio::inherit())
                // .spawn()
                // .unwrap();
                .exec();
        }
        Ok(())
    }
}
