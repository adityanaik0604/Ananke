use sysinfo::Networks;

pub fn get_network_data() -> Vec<(String, u64, u64)> {
    let networks = Networks::new_with_refreshed_list();
    networks
        .iter()
        .map(|(name, data)| {
            (
                name.to_string(),
                data.received(),
                data.transmitted(),
            )
        })
        .collect()
}