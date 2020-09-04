use std::fs;
use std::path::*;
use std::process::{Command, Stdio};

#[derive(Debug)]
enum SatoriError {
    Ans,
    Rte,
}

fn main() {
    let program_path = compile_program(Path::new("test.cpp"));
    if program_path.is_err() {
        println!("lol u code be not compile");
        return;
    }

    let test_result = run_test(Path::new("0.in"), program_path.unwrap().as_path());
    if test_result.is_err() {
        println!("{:?}", test_result.unwrap_err());
    }
}

fn run_test(test_path: &Path, program_path: &Path) -> Result<(), SatoriError> {
    let test_file = fs::File::open(test_path).unwrap();
    let mut buf_reader = std::io::BufReader::new(test_file);

    let mut program_process = Command::new(program_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to create command");

    if let Some(mut stdin) = program_process.stdin.as_mut() {
        std::io::copy(&mut buf_reader, &mut stdin).expect("Failed to copy stdin to buf_reader");
    }

    let output_path = Path::new("output");

    let output_file = fs::File::create(output_path).unwrap();
    let mut output_writer = std::io::BufWriter::new(output_file);

    if let Some(mut stdout) = program_process.stdout.as_mut() {
        std::io::copy(&mut stdout, &mut output_writer)
            .expect("Failed to copy stdout to buf_writer");
    }

    let exit_status = program_process.wait().expect("Failed to wait for a child");

    println!("Process exited with code {:?}", exit_status.code().unwrap());

    if !exit_status.success() {
        return Err(SatoriError::Rte);
    }
    return Ok(());

    return verify_test(test_path, output_path);
}

fn verify_test(test: &Path, output: &Path) -> Result<(), SatoriError> {
    unimplemented!()
}

fn compile_program(source_path: &Path) -> Result<PathBuf, ()> {
    let output_path = PathBuf::from("./program");
    let _ = fs::remove_file(&output_path);
    let compilation_status = Command::new("g++")
        .args(&[
            source_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .output()
        .map_err(|_| ())?;

    println!("{:?}", compilation_status.status);
    println!(
        "{:?}",
        String::from_utf8(compilation_status.stderr).unwrap()
    );

    if output_path.exists() {
        return Ok(output_path);
    }
    Err(())
}
