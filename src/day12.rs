use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;
use std::collections::{HashMap, HashSet};

type Plot = (i32, i32);

#[derive(Eq, Hash, PartialEq, Debug)]
struct Border {
    inside: Plot,
    outside: Plot,
}

impl Border {
    fn next_straight(&self) -> Self {
        let x_diff = self.outside.0 - self.inside.0;
        let y_diff = self.outside.1 - self.inside.1;

        let (x_delta, y_delta) = match (x_diff, y_diff) {
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            _ => panic!(),
        };
        Border {
            inside: (self.inside.0 + x_delta, self.inside.1 + y_delta),
            outside: (self.outside.0 + x_delta, self.outside.1 + y_delta),
        }
    }
}

#[derive(Debug)]
struct Stats {
    area: u32,
    perimeter: u32,
}

fn neighbours(&(h, w): &Plot) -> [Plot; 4] {
    [(h - 1, w), (h, w - 1), (h + 1, w), (h, w + 1)]
}
pub(crate) fn day12(lines: Vec<String>) -> FunctionOutput {
    let plots = parse(lines);
    let region_map: HashMap<Plot, u32> = get_region_map(&plots);
    let region_stats = calculate_stats(&region_map);
    let region_sides = get_sides_per_region(&region_map);

    let partone = region_stats
        .values()
        .map(|stats| stats.area * stats.perimeter)
        .sum::<u32>() as i64;
    let parttwo = region_stats
        .iter()
        .map(|(region, stats)| stats.area * region_sides.get(region).unwrap())
        .sum::<u32>() as i64;
    IntPair(partone, parttwo)
}

fn get_sides_per_region(region_map: &HashMap<Plot, u32>) -> HashMap<u32, u32> {
    let mut region_borders: HashMap<u32, HashSet<Border>> = HashMap::new();
    for (plot, region) in (&region_map).iter() {
        for &neighbour in neighbours(plot)
            .iter()
            .filter(|neighbour| region_map.get(neighbour) != Some(region))
        {
            region_borders
                .entry(*region)
                .or_insert(HashSet::new())
                .insert(Border {
                    inside: *plot,
                    outside: neighbour,
                });
        }
    }

    let region_sides: HashMap<u32, u32> = region_borders
        .iter()
        .map(|(&region, borders)| {
            (
                region,
                borders
                    .iter()
                    .filter(|border| !borders.contains(&border.next_straight()))
                    .count() as u32,
            )
        })
        .collect();
    region_sides
}

fn calculate_stats(region_map: &HashMap<Plot, u32>) -> HashMap<u32, Stats> {
    let mut region_stats: HashMap<u32, Stats> = HashMap::new();
    for (plot, region) in region_map.iter() {
        let additional_perimeter = neighbours(plot)
            .iter()
            .filter(|neighbour| region_map.get(neighbour) != Some(region))
            .count() as u32;
        let stats = region_stats.entry(*region).or_insert(Stats {
            area: 0,
            perimeter: 0,
        });
        stats.area += 1;
        stats.perimeter += additional_perimeter;
    }
    region_stats
}

fn get_region_map(plots: &HashMap<Plot, char>) -> HashMap<Plot, u32> {
    let mut region_num = 0;
    let mut region_map: HashMap<Plot, u32> = HashMap::new();
    let mut visited = HashSet::<Plot>::new();
    let mut to_visit: Vec<Plot> = vec![*plots.keys().next().unwrap()];
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

fn parse(lines: Vec<String>) -> HashMap<Plot, char> {
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
        assert_eq!(day12(lines), IntPair(1930, 1206));
    }

    #[test]
    fn test_day_12() {
        let lines = read_testfile("day12.txt");
        assert_eq!(day12(lines), IntPair(1533644, 936718));
    }

    #[test]
    fn test_next() {
        let border = Border {
            inside: (0, 1),
            outside: (0, 0),
        };
        let expected = Border {
            inside: (-1, 1),
            outside: (-1, 0),
        };
        assert_eq!(border.next_straight(), expected);
    }
}
