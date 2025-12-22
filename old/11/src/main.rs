use std::collections::HashMap;
use std::fs;

struct Device {
    id: String,
    outputs: Vec<String>,
}

impl Device {
    fn path_count(&self, id: &str, device_map: &HashMap<String, Device>, memo_map: &mut HashMap<String, u64>) -> u64 {
        let memoised = memo_map.get(&self.id);
        if memoised.is_some() { return memoised.unwrap().clone(); }
        
        if self.id == id { return 1; }

        let count: u64 = self.outputs
            .iter()
            .map(|s| device_map.get(s).unwrap().path_count(id, device_map, memo_map))
            .sum();

        memo_map.insert(self.id.to_string(), count.clone());
        count
    }
}

fn main() -> Result<(), std::io::Error>  {
    let mut device_map: HashMap<String, Device> = HashMap::new();

    let devices: Vec<Device> = fs::read_to_string("./input.txt")?
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();

            let id = parts[0].trim().to_string();
            let outputs: Vec<_> = parts[1]
                .split(' ')
                .filter(|p| p.trim().len() > 0)
                .map(|p| p.to_string())
                .collect();

            Device { id, outputs }
        })
        .collect();

    for d in devices {
        device_map.insert(d.id.clone(), d);
    }

    device_map.insert("out".to_string(), Device { id: "out".to_string(), outputs: vec![] });

    let total_paths = device_map.get("you").unwrap().path_count("out", &device_map, &mut HashMap::new());
    
    println!("Total paths from you to out: {}", total_paths);

    let paths_svr_fft = device_map.get("svr").unwrap().path_count("fft", &device_map, &mut HashMap::new());
    let paths_fft_dac = device_map.get("fft").unwrap().path_count("dac", &device_map, &mut HashMap::new());
    let paths_dac_out = device_map.get("dac").unwrap().path_count("out", &device_map, &mut HashMap::new());

    let route_1_total = paths_svr_fft * paths_fft_dac * paths_dac_out;

    let paths_svr_dac = device_map.get("svr").unwrap().path_count("dac", &device_map, &mut HashMap::new());
    let paths_dac_fft = device_map.get("dac").unwrap().path_count("fft", &device_map, &mut HashMap::new());
    let paths_fft_out = device_map.get("fft").unwrap().path_count("out", &device_map, &mut HashMap::new());
    
    let route_2_total = paths_svr_dac * paths_dac_fft * paths_fft_out;

    let grand_total = route_1_total + route_2_total;
    println!("Total paths svr to out via dac and fft: {:?}", grand_total);
    Ok(())
}