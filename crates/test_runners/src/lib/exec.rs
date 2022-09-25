use std::process::Command;

pub fn execute(cmd: &mut Command) -> Result<std::process::Output, std::io::Error> {
    let args: Vec<String> = cmd
        .get_args()
        .map(|s| s.to_string_lossy().into_owned())
        .collect();
    println!(
        "> {} {}",
        cmd.get_program().to_string_lossy(),
        args.join(" ")
    );
    let result = cmd.output();
    if let Ok(ref output) = result {
        println!("> Exited with {}", output.status);
        if !output.stdout.is_empty() {
            println!("### START STDOUT ###");
            println!("{}", String::from_utf8(output.stdout.clone()).unwrap());
            println!("### END STDOUT ###");
        }
        if !output.stderr.is_empty() {
            println!("### START STDERR ###");
            println!("{}", String::from_utf8(output.stderr.clone()).unwrap());
            println!("### END STDERR ###");
        }
    }
    result
}

pub fn execute_or_die(cmd: &mut Command) {
    let result = execute(cmd);
    match result {
        Ok(output) => {
            if !output.status.success() {
                panic!("! Exited with {}", output.status)
            }
        }
        Err(err) => panic!("ERROR: {:?}", err),
    }
}
