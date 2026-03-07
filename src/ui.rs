use crate::System;

pub fn render(data: &System) {
    print!("\x1B[2J\x1B[1;1H"); // clear screen

    println!("=== SYSTEM MONITOR ===\n");

    println!("CPU Usage:");
    for (i, usage) in data.cpu_usage.iter().enumerate() {
        println!("Core {}: {:.2}%", i, usage);
    }

    println!("\nMemory:");
    println!("Used: {} MB / {} MB", data.used_memory, data.total_memory);

    println!("\nTop Processes:");
    for (name, cpu) in &data.processes {
        println!("{} - {:.2}%", name, cpu);
    }

    println!("\nNetwork:");
    for (name, rx, tx) in &data.network_data {
        println!("{}: RX {} bytes | TX {} bytes", name, rx, tx);
    }
}