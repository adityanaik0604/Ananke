use sysinfo::System;

pub fn get_total_memory(system: &System) -> u64 {
    system.total_memory() / 1024
}

pub fn get_used_memory(system: &System) -> u64 {
    system.used_memory() / 1024
}