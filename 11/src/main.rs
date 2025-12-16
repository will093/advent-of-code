use std::collections::HashMap;
use std::fs;

struct Device {
    id: String,
    outputs: Vec<String>,
}

impl Device {
    fn path_count(&self, id: &str, device_map: &HashMap<String, Device>) -> u64 {
        if self.id == id {
            return 1;
        }
        self.outputs
            .iter()
            .map(|s| device_map.get(s).unwrap().path_count(id, device_map))
            .sum()
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

    let paths = device_map.get("you").unwrap().path_count("out", &device_map);
    
    println!("Total paths from you to out: {}", paths);
    Ok(())
}