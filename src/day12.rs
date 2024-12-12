use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Stats {
    area: u32,
    perimeter: u32,
}

fn neighbours(&(h, w): &(i32, i32)) -> [(i32, i32); 4] {
    [(h - 1, w), (h, w - 1), (h + 1, w), (h, w + 1)]
}
pub(crate) fn day12(lines: Vec<String>) -> (i64, i64) {
    let plots = parse(lines);
    let region_map: HashMap<(i32, i32), u32> = get_region_map(plots);
    let region_stats = calculate_stats(region_map);

    let partone = region_stats
        .values()
        .map(|stats| stats.area * stats.perimeter)
        .sum::<u32>() as i64;
    let parttwo = 0;
    (partone, parttwo)
}

fn calculate_stats(region_map: HashMap<(i32, i32), u32>) -> HashMap<u32, Stats> {
    let mut region_stats: HashMap<u32, Stats> = HashMap::new();
    for (plot, region) in region_map.iter() {
        let additional_perimeter = neighbours(plot)
            .iter()
            .filter(|neighbour| region_map.get(neighbour) != Some(region))
            .count() as u32;
        let mut stats = region_stats.entry(*region).or_insert(Stats {
            area: 0,
            perimeter: 0,
        });
        stats.area += 1;
        stats.perimeter += additional_perimeter;
    }
    region_stats
}

fn get_region_map(plots: HashMap<(i32, i32), char>) -> HashMap<(i32, i32), u32> {
    let mut region_num = 0;
    let mut region_map: HashMap<(i32, i32), u32> = HashMap::new();
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut to_visit: Vec<(i32, i32)> = vec![*plots.keys().next().unwrap()];
    while let Some(master_plot) = to_visit.pop() {
        if visited.contains(&master_plot) {
            continue;
        }
        let master_plant = plots.get(&master_plot).unwrap();
        let mut plots_in_region = vec![master_plot];
        while let Some(plot) = plots_in_region.pop() {
            if visited.contains(&plot) {
                continue;
            }
            visited.insert(plot);
            region_map.insert(plot, region_num);
            for neighbour in neighbours(&plot).iter() {
                if visited.contains(neighbour) {
                    continue;
                }
                match plots.get(neighbour) {
                    Some(plant) => {
                        if plant == master_plant {
                            plots_in_region.push(*neighbour);
                        } else {
                            to_visit.push(*neighbour);
                        }
                    }
                    None => {}
                }
            }
        }
        region_num += 1;
    }
    region_map
}

fn parse(lines: Vec<String>) -> HashMap<(i32, i32), char> {
    let plots: HashMap<_, _> = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i32, y as i32), ch))
        })
        .flatten()
        .collect();
    plots
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_12_small() {
        let lines = read_testfile("day12test.txt");
        assert_eq!(day12(lines), (1930, 0));
    }

    #[test]
    fn test_day_12() {
        let lines = read_testfile("day12.txt");
        assert_eq!(day12(lines), (1533644, 0));
    }
}
