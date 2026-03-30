use std::env;
use std::process::Command;
use colorize::*;

fn compile(optimization_mode: &str, cfile: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let output_filename = format!("gcc{}", optimization_mode);
    let gcc_status = Command::new("gcc")
        .arg(optimization_mode)
        .arg("-o")
        .arg(&output_filename)
        .arg(cfile)
        .status()
        .expect("error of exec command");

    if gcc_status.success() {
        let output = Command::new("stat")
            .arg("-c")
            .arg("%s")
            .arg(&output_filename)
            .output()
            .expect("error of exec command");

        if output.status.success() {
            let size_str = String::from_utf8(output.stdout)
                .expect("invalid UTF-8 in stat output")
                .trim()
                .to_string();

            let size: usize = size_str.parse()
                .expect("failed to parse size as usize");

            println!("{optimization_mode} optimization: {} (size: {size} bytes)", 
                     output_filename.green());

            Ok(size)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("stat failed: {}", error);
            Err(Box::from("stat command failed"))
        }
    } else {
        Err(Box::from("gcc compilation failed"))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <source_file.c>", args[0]);
        std::process::exit(1);
    }

    let optimizations = ["-O0", "-O1", "-O2", "-O3", "-Os", "-Oz"];
    let mut results: Vec<(String, usize)> = Vec::new();

    println!("{}", "compilation results:".cyan().bold());
    println!("{}", "-".repeat(50));

    for &opt in &optimizations {
        match compile(opt, &args[1]) {
            Ok(size) => results.push((opt.to_string(), size)),
            Err(e) => eprintln!("  {}: {}", opt.red(), e),
        }
    }

    if results.is_empty() {
        eprintln!("{}", "No successful compilations!".red().bold());
        return Ok(());
    }

    let min_size = results.iter().map(|(_, size)| size).min().unwrap();

    let all_same = results.iter().all(|(_, size)| size == min_size);

    println!("\n{}", "=".repeat(50));
    println!("{}", "Summary:".cyan().bold());

    if all_same {
        println!("  Best optimization: {}", "any".yellow().bold());
        println!("  Size: {} bytes", min_size);
    } else {
       let best_opts: Vec<String> = results.iter()
           .filter(|(_, size)| size == min_size)
            .map(|(opt, _)| opt.clone())
            .collect();
        
        if best_opts.len() > 1 {
            println!("  Best optimizations: {}", 
                     best_opts.join(", ").green().bold());
            println!("  Size: {} bytes", min_size);
        } else {
            println!("  Best optimization: {} ({} bytes)",
                     <String as Clone>::clone(&best_opts[0]).green().bold(),
                     min_size);
        }
    }

    println!("\n{}", "Size comparison:".cyan().bold());
    for (opt, size) in &results {
        let diff = if size > min_size {
            format!("+{} bytes", size - min_size).red()
        } else if size < min_size {
            format!("-{} bytes", min_size - size).green()
        } else {
            "best".green().to_string()
        };
    
        println!("  {:4}: {:8} bytes {}", opt, size, diff);
    }
    println!("{}", "=".repeat(50));

    Ok(())
}
