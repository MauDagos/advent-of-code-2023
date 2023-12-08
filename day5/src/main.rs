use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
    ops::Range,
    path::Path,
};

fn main() {
    let (lowest_location_1, lowest_location_2) = check_almanac("./data/input.txt");
    println!("[Part 1] The lowest location is: {lowest_location_1}");
    println!("[Part 2] The lowest location is: {lowest_location_2}");
}

fn check_almanac<P>(filename: P) -> (u64, u64)
where
    P: AsRef<Path>,
{
    let almanac = Almanac::from_file(filename);
    if let Some(lowest_location_from_seeds) = almanac.find_lowest_location_from_seeds() {
        if let Some(lowest_location_from_seed_ranges) =
            almanac.find_lowest_location_from_seed_ranges()
        {
            (lowest_location_from_seeds, lowest_location_from_seed_ranges)
        } else {
            panic!("Unable to find location from seed ranges");
        }
    } else {
        panic!("Unable to find location from seeds");
    }
}

struct CategoryMapEntry {
    destination_range: Range<u64>,
    source_range: Range<u64>,
}

impl CategoryMapEntry {
    fn is_number_in_range(&self, num: u64) -> bool {
        self.source_range.contains(&num)
    }

    fn get_number_destination(&self, num: u64) -> u64 {
        self.destination_range.start + num - self.source_range.start
    }

    fn is_before_entry(&self, entry: &CategoryMapEntry) -> Ordering {
        if self.source_range.start < entry.source_range.start {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    fn is_destination_in_range(&self, range: &Range<u64>) -> bool {
        self.destination_range.start < range.end && range.start < self.destination_range.end
    }

    fn source_range_overlap(&self, range: &Range<u64>) -> Range<u64> {
        let mut start_offset = 0;
        let mut end_offset = 0;
        if range.start > self.destination_range.start {
            start_offset = range.start - self.destination_range.start;
        }
        if range.end < self.destination_range.end {
            end_offset = self.destination_range.end - range.end;
        }
        Range {
            start: self.source_range.start + start_offset,
            end: self.source_range.end - end_offset,
        }
    }
}

struct CategoryMap {
    entries: Vec<CategoryMapEntry>,
}

impl CategoryMap {
    fn find_number_destination(&self, num: u64) -> u64 {
        if let Some(entry) = self.entries.iter().find(|e| e.is_number_in_range(num)) {
            entry.get_number_destination(num)
        } else {
            num
        }
    }

    fn get_source_ranges(&self, destination_range: &Range<u64>) -> Vec<Range<u64>> {
        self.entries
            .iter()
            .filter(|e| e.is_destination_in_range(destination_range))
            .map(|e| e.source_range_overlap(destination_range))
            .collect()
    }

    fn fill_category_map(&mut self) {
        // Sort reversed, because pop() will grab from the end
        self.entries.sort_unstable_by(|a, b| b.is_before_entry(a));
        let mut final_entries = vec![];
        let mut start = 0;
        while let Some(entry) = self.entries.pop() {
            let entry_start = entry.source_range.start;
            if start < entry_start {
                final_entries.push(CategoryMapEntry {
                    destination_range: start..entry_start,
                    source_range: start..entry_start,
                });
            }
            start = entry.source_range.end;
            final_entries.push(entry);
        }
        final_entries.push(CategoryMapEntry {
            destination_range: start..u64::MAX,
            source_range: start..u64::MAX,
        });
        self.entries = final_entries;
    }
}

struct SeedToLocationRange {
    seed_ranges: Vec<Range<u64>>,
    location_range: Range<u64>,
}

struct Almanac {
    seeds: Vec<u64>,
    category_maps: Vec<CategoryMap>,
    seed_ranges: Vec<Range<u64>>,
    seed_to_location_ranges: Vec<SeedToLocationRange>,
}

impl Almanac {
    fn from_file<P>(filename: P) -> Self
    where
        P: AsRef<Path>,
    {
        let mut file_iter = BufReader::new(File::open(filename).unwrap())
            .lines()
            .flatten();
        let seeds = Self::parse_seeds(file_iter.next().unwrap());
        file_iter.next(); // consume new line
        let mut category_maps = vec![];
        while let Some(_) = file_iter.next() {
            // consume map title
            let mut entries = vec![];
            loop {
                let entry = file_iter.next();
                if entry.is_none() || entry == Some("".to_string()) {
                    break;
                }
                entries.push(entry.unwrap());
            }
            let map = Self::parse_category_map(entries);
            category_maps.push(map);
        }
        let seed_ranges = Self::make_seed_ranges(&seeds);
        Self::fill_category_maps(&mut category_maps);
        let seed_to_location_ranges = Self::make_seed_to_location_ranges(&category_maps);
        Almanac {
            seeds,
            category_maps,
            seed_ranges,
            seed_to_location_ranges,
        }
    }

    fn parse_numbers(nums: &str) -> Vec<u64> {
        nums.split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect()
    }

    fn parse_seeds(line: String) -> Vec<u64> {
        let (_, nums) = line.split_once(':').unwrap();
        Self::parse_numbers(nums)
    }

    fn parse_category_map(entries: Vec<String>) -> CategoryMap {
        let map_entries = entries
            .iter()
            .map(|e| {
                let parsed = Self::parse_numbers(e);
                let destination = parsed[0];
                let source = parsed[1];
                let range = parsed[2];
                CategoryMapEntry {
                    destination_range: destination..(destination + range),
                    source_range: source..(source + range),
                }
            })
            .collect();
        CategoryMap {
            entries: map_entries,
        }
    }

    fn fill_category_maps(maps: &mut [CategoryMap]) {
        maps.iter_mut().for_each(|map| map.fill_category_map());
    }

    fn make_seed_ranges(seeds: &[u64]) -> Vec<Range<u64>> {
        seeds
            .chunks(2)
            .map(|s| Range {
                start: s[0],
                end: s[0] + s[1],
            })
            .collect()
    }

    fn make_seed_to_location_ranges(maps: &[CategoryMap]) -> Vec<SeedToLocationRange> {
        // Get hold of all of the destination ranges in the last map. These are
        // the location ranges.
        let mut location_ranges: Vec<Range<u64>> = maps
            .last()
            .unwrap()
            .entries
            .iter()
            .map(|e| e.destination_range.clone())
            .collect();
        // Sort them by the start of the range.
        location_ranges.sort_unstable_by(|a, b| {
            if a.start < b.start {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        // For each location range, find it's corresponding seed ranges
        location_ranges
            .into_iter()
            .map(|l| {
                // Traverse the maps backwards to get the final source ranges.
                // Start off by looking in the last map for the source ranges of
                // the location range, and continue onwards.
                let mut destination_ranges = vec![l.clone()];
                maps.iter().rev().for_each(|m| {
                    destination_ranges = destination_ranges
                        .iter()
                        .flat_map(|dest_range| m.get_source_ranges(dest_range))
                        .collect()
                });
                SeedToLocationRange {
                    seed_ranges: destination_ranges,
                    location_range: l,
                }
            })
            .collect()
    }

    fn find_location(&self, seed: u64) -> u64 {
        let mut num_to_find = seed;
        for map in &self.category_maps {
            num_to_find = map.find_number_destination(num_to_find);
        }
        num_to_find
    }

    fn find_lowest_location_from_seeds(&self) -> Option<u64> {
        for stl_range in &self.seed_to_location_ranges {
            let mut candidate_seeds = vec![];
            for seed_range in &stl_range.seed_ranges {
                self.seeds
                    .iter()
                    .filter(|seed| seed_range.contains(seed))
                    .for_each(|seed| candidate_seeds.push(*seed));
            }
            if !candidate_seeds.is_empty() {
                return Some(
                    candidate_seeds
                        .into_iter()
                        .map(|seed| self.find_location(seed))
                        .min()
                        .unwrap(),
                );
            }
        }
        None
    }

    fn intersect_ranges(range1: &Range<u64>, range2: &Range<u64>) -> Range<u64> {
        range1.start.max(range2.start)..range1.end.min(range2.end)
    }

    fn find_lowest_location_from_seed_ranges(&self) -> Option<u64> {
        for stl_range in &self.seed_to_location_ranges {
            let mut candidate_seeds = vec![];
            for stl_seed_range in &stl_range.seed_ranges {
                self.seed_ranges
                    .iter()
                    .map(|seed_range| Self::intersect_ranges(seed_range, stl_seed_range))
                    .filter(|new_range| new_range.start < new_range.end)
                    .for_each(|new_range| {
                        for seed in new_range {
                            candidate_seeds.push(seed)
                        }
                    })
            }
            if !candidate_seeds.is_empty() {
                return Some(
                    candidate_seeds
                        .into_iter()
                        .map(|seed| self.find_location(seed))
                        .min()
                        .unwrap(),
                );
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(check_almanac("./data/example.txt"), (35, 46));
    }

    #[test]
    fn test_input() {
        assert_eq!(check_almanac("./data/input.txt"), (403695602, 219529182));
    }
}
