use sysinfo::System;

pub fn get_cpu_usage(system:&System) -> Vec<f32> {
    system.cpus().iter().map(|cpu|cpu.cpu_usage()).collect()
}