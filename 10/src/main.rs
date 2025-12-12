use regex::Regex;
use std::fs;
use rand::Rng;

#[derive(Debug)]
struct Machine {
    joltage_current: Vec<u32>,
    joltage_configured: Vec<u32>,
    indicators_current: Vec<bool>,
    indicators_configured: Vec<bool>,
    switches: Vec<Vec<usize>>,
}

impl Machine {

    fn pull_switch_config(&mut self, i: usize) {
        let switch = &self.switches[i];
        for n in switch {
            self.indicators_current[*n] = !self.indicators_current[*n];
        }
    }

    fn safe_pull_switch_joltage(&mut self, i: usize) -> bool {
        let switch = &self.switches[i];

        let mut safe = true;

        for n in switch {
            if self.joltage_current[*n] + 1 > self.joltage_configured[*n] {
                safe = false;
            }
        }
        if safe {
            for n in switch {
                self.joltage_current[*n] += 1;
            }
        }
        safe
    }

    fn new(joltage_configured: Vec<u32>, indicators_configured: Vec<bool>, switches: Vec<Vec<usize>>) -> Machine {
        let mut machine = Self {
            joltage_configured,
            indicators_configured,
            switches,
            joltage_current: vec![],
            indicators_current: vec![],
        };
        machine.reset();
        machine
    }

    fn reset(&mut self) {
        self.indicators_current = self.indicators_configured.iter().map(|_| false).collect();
        self.joltage_current = self.joltage_configured.iter().map(|_| 0).collect();
    }

}

fn main() -> Result<(), std::io::Error> {
    let mut machines: Vec<Machine> = fs::read_to_string("./input.txt")?
        .lines()
        .map(|line| {
            let indicators_re = Regex::new(r"\[(.*?)\]").unwrap();
            let indicators_configured: Vec<bool> = indicators_re
                .captures_iter(line)
                .next()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .split("")
                .filter(|x| x.len() > 0)
                .map(|x| if x == "#" { true } else { false })
                .collect();

            let switches_re = Regex::new(r"\((.*?)\)").unwrap();

            let switches: Vec<Vec<usize>> = switches_re
                .captures_iter(line)
                .filter_map(|cap| cap.get(1).map(|m| m.as_str()))
                .map(|s| s.split(",").map(|x| x.parse().unwrap()).collect())
                .collect();

            let joltage_re = Regex::new(r"\{(.*?)\}").unwrap();
            let joltage_configured: Vec<u32> = joltage_re
                .captures_iter(line)
                .next()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect();

            Machine::new(
                joltage_configured,
                indicators_configured,
                switches,
            )
        })
        .collect();

    // let total = configure_machines(&mut machines);
    // println!("Total: {}", total);

    let total_joltage_pulls = configure_joltage(&mut machines);
    println!("Total joltage: {}", total_joltage_pulls);

    Ok(())
}


fn configure_joltage(machines: &mut Vec<Machine>) -> i32 {
    let mut rng = rand::thread_rng();

    let total = machines
        .iter_mut()
        .fold(0, |sum, machine| {
            let mut min_pull_count = 10000;

            println!("Running {}", sum);

            let mut found_correct = false;
            for _ in 0..10000 {
                machine.reset();

                let mut pull_count = 0;
                for _ in 0..1000 {
                    if machine.joltage_current == machine.joltage_configured {
                        found_correct = true;
                        break;
                    }

                    let rand_switch_index = rng.gen_range(0..machine.switches.len());
                    let did_pull = machine.safe_pull_switch_joltage(rand_switch_index);

                    if !did_pull {
                        continue;
                    }

                    // println!("joltage {:?} {:?}", machine.joltage_current, machine.joltage_configured);
                    pull_count += 1
                }

                min_pull_count = min_pull_count.min(pull_count);
            }

            if min_pull_count == 10000 || !found_correct {
                panic!("Joltage could not be configured!");
            }

            sum + min_pull_count
        });

    total
}

fn configure_indicators(machines: &mut Vec<Machine>) -> i32 {
    let mut rng = rand::thread_rng();

    let total = machines
        .iter_mut()
        .fold(0, |sum, machine| {
            let mut min_pull_count = 10000;

            for _ in 0..10000 {
                machine.reset();

                let mut pull_count = 0;
                loop {
                    if machine.indicators_current == machine.indicators_configured {
                        break;
                    }

                    let rand_switch_index = rng.gen_range(0..machine.switches.len());
                    machine.pull_switch_config(rand_switch_index);
                    pull_count += 1
                }

                min_pull_count = min_pull_count.min(pull_count);
            }

            sum + min_pull_count
        });

    total
}