use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph},
};

use sysinfo::System;

// import your modules
use crate::{cpu, memory, process, network};

pub fn run_app() -> Result<(), Box<dyn Error>> {
    // 🔹 setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 🔹 system object
    let mut system = System::new_all();

    // 🔁 main loop
    loop {
        system.refresh_all();

        // ✅ collect data HERE (this was missing)
        let cpu_usage = cpu::get_cpu_usage(&system);
        let used_memory = memory::get_used_memory(&system);
        let total_memory = memory::get_total_memory(&system);
        let processes = process::get_process_data(&system);
        let network_data = network::get_network_data();

        terminal.draw(|f| {
            let size = f.size();

            let mut text = String::new();

            text.push_str("=== SYSTEM MONITOR ===\n\n");

            // 🔹 CPU
            text.push_str("CPU Usage\n");
            for (i, usage) in cpu_usage.iter().enumerate() {
                text.push_str(&format!("Core {}: {:.1}%\n", i, usage));
            }

            // 🔹 Memory
            text.push_str("\nMemory\n");
            text.push_str(&format!(
                "Used: {} MB / {} MB\n",
                used_memory,
                total_memory
            ));

            // 🔹 Processes
            text.push_str("\nTop Processes\n");
            for (name, cpu) in processes.iter().take(5) {
                text.push_str(&format!("{} - {:.1}%\n", name, cpu));
            }

            // 🔹 Network
            text.push_str("\nNetwork\n");
            for (iface, rx, tx) in network_data.iter() {
                text.push_str(&format!(
                    "{}: RX {} bytes | TX {} bytes\n",
                    iface, rx, tx
                ));
            }

            text.push_str("\nPress 'q' to quit");

            let block = Block::default()
                .title("System Monitor")
                .borders(Borders::ALL);

            let paragraph = Paragraph::new(text).block(block);

            f.render_widget(paragraph, size);
        })?;

        // ⌨️ input handling
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // 🔹 cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}