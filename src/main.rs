//The main conductor of the Orchestra.

use sysinfo::System as SysInfo;
use std::{thread, time::Duration};

mod cpu;
mod memory;
mod network;
mod process;
mod ui;

//uncomment the below lines pls...

pub struct System{
    pub cpu_usage : Vec<f32>,
    pub total_memory : u64,
    pub used_memory : u64,
    pub processes : Vec<(String, f32)>,
    pub network_data : Vec<(String, u64, u64)>  
}

fn main(){
    let mut system = SysInfo::new_all();

    loop{
        system.refresh_all();

        let data = System{
            cpu_usage : cpu::get_cpu_usage(&system),
            total_memory : memory::get_total_memory(&system),
            used_memory : memory::get_used_memory(&system),
            processes : process::get_process_data(&system),
            network_data : network::get_network_data(),
        };

        ui::render(&data);

        thread::sleep(Duration::from_secs(1));
    }
    
}