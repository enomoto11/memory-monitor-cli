use clap::{Arg, Command};
use colored::*;
use std::collections::HashMap;
use sysinfo::{System, SystemExt, ProcessExt};

#[derive(Debug)]
struct MemoryInfo {
    total_memory: u64,
    used_memory: u64,
    free_memory: u64,
    available_memory: u64,
}

#[derive(Debug)]
struct ProcessInfo {
    name: String,
    memory_usage: u64,
    memory_percent: f64,
}

fn main() {
    let matches = Command::new("memory-monitor-cli")
        .version("0.1.0")
        .about("A Rust CLI tool for monitoring system memory usage with visual display")
        .arg(
            Arg::new("apps")
                .short('a')
                .long("apps")
                .help("Show memory usage by application")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("top")
                .short('t')
                .long("top")
                .help("Number of top processes to show")
                .value_name("NUMBER")
                .default_value("15"),
        )
        .get_matches();

    let mut system = System::new_all();
    system.refresh_all();

    let memory_info = get_memory_info(&system);
    display_system_memory(&memory_info);

    if matches.get_flag("apps") {
        let top_count: usize = matches
            .get_one::<String>("top")
            .unwrap()
            .parse()
            .unwrap_or(15);
        display_app_memory(&system, top_count);
    }
}

fn get_memory_info(system: &System) -> MemoryInfo {
    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let free_memory = system.free_memory();
    let available_memory = system.available_memory();

    MemoryInfo {
        total_memory,
        used_memory,
        free_memory,
        available_memory,
    }
}

fn display_system_memory(memory_info: &MemoryInfo) {
    let total_gb = memory_info.total_memory as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_gb = memory_info.used_memory as f64 / 1024.0 / 1024.0 / 1024.0;
    let free_gb = memory_info.free_memory as f64 / 1024.0 / 1024.0 / 1024.0;
    let available_gb = memory_info.available_memory as f64 / 1024.0 / 1024.0 / 1024.0;

    let used_percent = (memory_info.used_memory as f64 / memory_info.total_memory as f64) * 100.0;

    println!("{}", "=".repeat(60).blue());
    println!("{}", "                    メモリ使用状況".white().bold());
    println!("{}", "=".repeat(60).blue());
    println!();
    println!("総メモリ: {:.1} GB", total_gb.to_string().green().bold());
    println!();

    println!("メモリ内訳:");
    println!("  使用中:      {:>8.1} GB ({:>5.1}%)", used_gb, used_percent);
    println!("  空き:        {:>8.1} GB ({:>5.1}%)", free_gb, (free_gb / total_gb) * 100.0);
    println!("  利用可能:    {:>8.1} GB ({:>5.1}%)", available_gb, (available_gb / total_gb) * 100.0);

    println!();
    println!("使用中: {:.1} GB ({:.1}%)", used_gb, used_percent);
    println!("空き:   {:.1} GB ({:.1}%)", free_gb, (free_gb / total_gb) * 100.0);
    println!();

    // Visual bar
    let bar_length = 50;
    let used_blocks = (used_percent / 100.0 * bar_length as f64) as usize;
    let free_blocks = bar_length - used_blocks;

    println!("メモリ使用率:");
    let bar = format!("[{}{}]", 
        "█".repeat(used_blocks).red(),
        "░".repeat(free_blocks).white()
    );
    println!("{}", bar);
    println!("{}{:.1}%", " ".repeat(used_blocks), used_percent);

    // Memory pressure indicator
    let status = if used_percent < 70.0 {
        "正常 ✓".green()
    } else if used_percent < 85.0 {
        "中程度".yellow()
    } else {
        "高負荷 ⚠".red()
    };

    println!();
    println!("メモリ圧迫度: {}", status);
    println!("{}", "=".repeat(60).blue());
}

fn display_app_memory(system: &System, top_count: usize) {
    let mut app_memory: HashMap<String, u64> = HashMap::new();
    
    for (_, process) in system.processes() {
        let app_name = extract_app_name(process.name());
        let memory = process.memory();
        
        *app_memory.entry(app_name).or_insert(0) += memory;
    }

    let mut sorted_apps: Vec<_> = app_memory.iter().collect();
    sorted_apps.sort_by(|a, b| b.1.cmp(a.1));

    let total_memory = system.total_memory();

    println!();
    println!("{}", "=".repeat(70).blue());
    println!("{}", "                  アプリケーション別メモリ使用状況".white().bold());
    println!("{}", "=".repeat(70).blue());
    println!();
    println!("{:<30} {:>15} {:>10} グラフ", "アプリケーション", "メモリ使用量", "使用率");
    println!("{}", "-".repeat(70));

    for (app, &memory) in sorted_apps.iter().take(top_count) {
        let memory_mb = memory as f64 / 1024.0 / 1024.0;
        let memory_percent = (memory as f64 / total_memory as f64) * 100.0;
        let bar_length = (memory_percent * 2.0) as usize;
        let bar = "█".repeat(bar_length);
        
        let colored_bar = if memory_percent > 2.0 {
            bar.red()
        } else if memory_percent > 1.0 {
            bar.yellow()
        } else {
            bar.green()
        };
        
        let truncated_app = if app.len() > 30 {
            format!("{}...", &app[..27])
        } else {
            app.to_string()
        };
        
        println!("{:<30} {:>10.0} MB {:>8.1}% {}", 
            truncated_app, 
            memory_mb, 
            memory_percent, 
            colored_bar
        );
    }

    let total_used: u64 = sorted_apps.iter().map(|(_, &memory)| memory).sum();
    let total_used_mb = total_used as f64 / 1024.0 / 1024.0;
    let total_used_percent = (total_used as f64 / total_memory as f64) * 100.0;

    println!("{}", "-".repeat(70));
    println!("{:<30} {:>10.0} MB {:>8.1}%", "合計使用量", total_used_mb, total_used_percent);
    println!("{}", "=".repeat(70).blue());
}

fn extract_app_name(process_name: &str) -> String {
    // Extract meaningful app names from process names
    let name = process_name.to_string();
    
    // Remove common suffixes and clean up names
    let cleaned = name
        .replace(".app", "")
        .replace(" Helper", "")
        .replace(" (Renderer)", "")
        .replace(" (GPU)", "")
        .replace("com.docker.", "Docker ")
        .replace("com.apple.", "");
    
    // Take first part if it's a long path-like name
    if cleaned.contains('/') {
        cleaned.split('/').last().unwrap_or(&cleaned).to_string()
    } else {
        cleaned
    }
}

trait StringTruncate {
    fn truncate(&self, max_len: usize) -> String;
}

impl StringTruncate for str {
    fn truncate(&self, max_len: usize) -> String {
        if self.len() <= max_len {
            self.to_string()
        } else {
            format!("{}...", &self[..max_len.saturating_sub(3)])
        }
    }
}