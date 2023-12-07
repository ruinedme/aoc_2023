use timer::profile;

pub fn run_day5(inputs: &String) {
    profile! {
        let day5_1 = day5_1(&inputs);
        println!("Day 5-1: {day5_1}");
    }

    profile! {
        let day5_2 = day5_2(&inputs);
        println!("Day 5-2: {day5_2}");
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Range {
    dest_start: usize,
    source_start: usize,
    range: usize,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Category {
    name: String,
    ranges: Vec<Range>,
}

struct SeedMap {
    seeds: Vec<usize>,
    categories: Vec<Category>,
}

impl SeedMap {
    fn new(inputs: &String) -> Self {
        let mut seeds: Vec<usize> = Vec::new();
        let mut map_name = String::new();
        let mut categories: Vec<Category> = Vec::new();
        let mut ranges: Vec<Range> = Vec::new();

        for (i, line) in inputs.lines().enumerate() {
            if i == 0 {
                let a: Vec<&str> = line.split(": ").collect();
                seeds = a[1]
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect();
                continue;
            }

            if line.contains("-to-") {
                let a: Vec<&str> = line.split_ascii_whitespace().collect();
                map_name = String::from(a[0]);
                continue;
            }

            if line.is_empty() && !&ranges.is_empty() {
                categories.push(Category {
                    name: map_name.clone(),
                    ranges: ranges.clone(),
                });
                ranges.clear();
                continue;
            }
            let current_range: Vec<usize> = line
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            if !current_range.is_empty() {
                ranges.push(Range {
                    dest_start: current_range[0],
                    source_start: current_range[1],
                    range: current_range[2],
                });
            }
        }
        categories.push(Category {
            name: map_name.clone(),
            ranges: ranges.clone(),
        });
        return SeedMap { seeds, categories };
    }

    fn seed_to_location(&self, seed: usize) -> usize {
        let mut location = seed;
        for category in &self.categories {
            for range in &category.ranges {
                if location >= range.source_start && location < range.source_start + range.range {
                    let offset = range.dest_start.abs_diff(range.source_start);
                    if range.dest_start < range.source_start {
                        location -= offset;
                    } else {
                        location += offset;
                    }
                    break;
                }
            }
        }
        return location;
    }

    // Very hacky can probably be done better
    fn location_to_seed(&self, location: usize) -> usize {
        let mut category = self.categories.len() - 1;
        let mut seed = location;

        while category > 0 {
            for range in &self.categories[category].ranges {
                if seed >= range.dest_start && seed < range.dest_start + range.range {
                    let offset = range.source_start.abs_diff(range.dest_start);
                    if range.dest_start < range.source_start {
                        seed += offset
                    } else {
                        seed -= offset;
                    }
                    break;
                }
            }
            category -= 1;
        }
        // have to do a final iteration on categories[0] due to the above causing a an out of bounds error checking for >= 0
        for range in &self.categories[0].ranges {
            if seed >= range.dest_start && seed < range.dest_start + range.range {
                let offset = range.source_start.abs_diff(range.dest_start);
                if range.dest_start < range.source_start {
                    seed += offset
                } else {
                    seed -= offset;
                }
                break;
            }
        }
        return seed;
    }
}

fn day5_1(inputs: &String) -> usize {
    let seed_map = SeedMap::new(inputs);
    let mut lowest = usize::MAX;
    for seed in &seed_map.seeds {
        let location = seed_map.seed_to_location(*seed);
        lowest = lowest.min(location);
    }

    return lowest;
}

fn day5_2(inputs: &String) -> usize {
    let seed_map = SeedMap::new(inputs);
    let higest_seed = &seed_map
        .seeds
        .iter()
        .enumerate()
        .step_by(2)
        .map(|(i, &x)| x + &seed_map.seeds[i + 1])
        .max()
        .unwrap();

    // The idea being start at seed location 1 and work backwards from the mappings to find the seed location.
    // Return the first seed that is in any given seed range.
    for i in 1..higest_seed + 1 {
        let location = seed_map.location_to_seed(i);
        let seed: Vec<usize> = seed_map
            .seeds
            .iter()
            .enumerate()
            .step_by(2)
            .filter(|(i, &x)| {
                let range = x + seed_map.seeds[i + 1];
                location >= x && location <= range
            })
            .map(|x| *x.1)
            .collect();
        if !seed.is_empty() {
            return i;
        }
    }

    // Original solution ~2 minutes
    // let mut lowest = usise::MAX;
    // for i in (0..seed_map.seeds.len() - 1).step_by(2) {
    //     let range = seed_map.seeds[i] + seed_map.seeds[i + 1];
    //     for seed in seed_map.seeds[i]..range {
    //         let location = seed_map.calculate_seed_location(seed);
    //         lowest = lowest.min(location);
    //     }
    // }
    // return max;

    panic!("Failed to Find Seed Mapping");
}
