use crate::define_solver;
use std::collections::HashMap;

define_solver!(
    Day11Solver,
    "2025",
    "11",
    HashMap<String, Device>,
    preprocess,
    part_one,
    part_two
);

fn preprocess(input: &str) -> HashMap<String, Device> {
    let mut device_map: HashMap<String, Device> = HashMap::new();

    let devices: Vec<Device> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();

            let id = parts[0].to_string();
            let outputs: Vec<_> = parts[1]
                .split(' ')
                .filter(|&p| p.len() > 0)
                .map(|p| p.to_string())
                .collect();

            Device { id, outputs }
        })
        .collect();

    for d in devices {
        device_map.insert(d.id.clone(), d);
    }

    device_map.insert("out".to_string(), Device { id: "out".to_string(), outputs: vec![] });
    device_map
}

fn part_one(device_map: &HashMap<String, Device>) -> String {
    get_paths_to_out(device_map).to_string()
}

fn part_two(device_map: &HashMap<String, Device>) -> String {
    get_paths_part_two(device_map).to_string()
}

fn get_paths_to_out(device_map: &HashMap<String, Device>) -> u64 {
    device_map["you"].path_count("out", &device_map, &mut HashMap::new())
}

fn get_paths_part_two(device_map: &HashMap<String, Device>) -> u64 {
    let paths_svr_fft = device_map["svr"].path_count("fft", &device_map, &mut HashMap::new());
    let paths_fft_dac = device_map["fft"].path_count("dac", &device_map, &mut HashMap::new());
    let paths_dac_out = device_map["dac"].path_count("out", &device_map, &mut HashMap::new());

    let route_1_total = paths_svr_fft * paths_fft_dac * paths_dac_out;

    let paths_svr_dac = device_map["svr"].path_count("dac", &device_map, &mut HashMap::new());
    let paths_dac_fft = device_map["dac"].path_count("fft", &device_map, &mut HashMap::new());
    let paths_fft_out = device_map["fft"].path_count("out", &device_map, &mut HashMap::new());
    
    let route_2_total = paths_svr_dac * paths_dac_fft * paths_fft_out;

    route_1_total + route_2_total
}


struct Device {
    id: String,
    outputs: Vec<String>,
}

impl Device {
    fn path_count(&self, id: &str, device_map: &HashMap<String, Device>, memo_map: &mut HashMap<String, u64>) -> u64 {
        let memoised = memo_map.get(&self.id);
        if memoised.is_some() { return *memoised.expect("expect memoised is some"); }
        
        if self.id == id { return 1; }

        let count: u64 = self.outputs
            .iter()
            .map(|s| device_map[s].path_count(id, device_map, memo_map))
            .sum();

        memo_map.insert(self.id.to_string(), count);
        count
    }
}