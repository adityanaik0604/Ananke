use sysinfo::System;

pub fn get_process_data(system: &System) -> Vec<(String, f32)> {
    let mut processes: Vec<_> = system
        .processes()
        .values()
        .map(|p| (p.name().to_string(), p.cpu_usage()))
        .collect();

    processes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    processes.truncate(5);

    processes
}
