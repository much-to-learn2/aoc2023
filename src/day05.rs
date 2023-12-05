use std::fs;

// a sortof proxy `HashMap`
#[derive(Debug)]
struct ProxyMap {
    key_ranges: Vec<(i64, i64)>,
    val_ranges: Vec<i64>,
}

impl ProxyMap {
    fn new() -> Self {
        Self {
            key_ranges: vec![],
            val_ranges: vec![],
        }
    }

    fn get(&self, key: i64) -> i64 {
        let mut i = 0;
        for &(start, stop) in self.key_ranges.iter() {
            if key >= start && key <= stop {
                let diff = key - start;
                let res = self.val_ranges[i] + diff;
                // println!("{key} mapped to {res}");
                return self.val_ranges[i] + diff;
            }
            i += 1;
        }
        key
    }

    // TODO: inserting w binary sort would keep ranges sorted, make 
    // lookups faster 
    fn add_map(&mut self, dest: i64, src: i64, len: i64) {
        self.key_ranges.push((src, src + len - 1));
        self.val_ranges.push(dest);
    }

    fn get_key(&self, v: i64) -> i64 {
        for i in 0..self.key_ranges.len() {
            if v >= self.val_ranges[i] && v <= self.val_ranges[i] + (self.key_ranges[i].1 - self.key_ranges[i].0) {
                let diff = v - self.val_ranges[i];
                return self.key_ranges[i].0 + diff;
            };
        };
        v
    }
}

// ordered vec of mappings for propagating a value through
struct MapSet {
    maps: Vec<ProxyMap>,
}

impl MapSet {
    fn propagate(&self, i: i64) -> i64 {
        let mut res = -1;
        for map in self.maps.iter() {
            if res == -1 {
                res = map.get(i);
            } else {
                res = map.get(res);
            };
        }
        res
    }

    fn back_propagate(&self, v: i64) -> i64 {
        let mut res = -1;
        let mut i = 0;
        for map in self.maps.iter().rev() {
            if i == 0 {
                res = map.get_key(v);
            } else {
                res = map.get_key(res);
            };
            i += 1;
        };
        res
    }
}


pub fn main() {
    println!("main from day05!");
    let contents = fs::read_to_string("inputs/day05.txt").expect("unable to read file contents");
    let mut it = contents.lines();
    let seeds: &str = it.next().unwrap();
    let seeds: Vec<i64> = seeds[1+seeds.find(" ").unwrap()..].split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
    it.next(); // newline

    let mut i = 7;
    let mut maps: Vec<ProxyMap> = vec![];
    loop {
        let _map_title = it.next();
        println!("map title: {:?}", _map_title);
        let mut pm: ProxyMap = ProxyMap::new();
        while let Some(line) = it.next() {
            if line.is_empty() { break };
            println!("line: {line}");
            let coords: Vec<i64> = line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
            pm.add_map(coords[0], coords[1], coords[2]);
        }

        maps.push(pm);        
        i -= 1;
        if i == 0 { break };
    }
    
    println!("seeds: {:?}", seeds);
    println!("maps: {:?}", maps);

    let mapset: MapSet = MapSet { maps: maps };
    let mut part1 = i64::MAX;
    for seed in &seeds {
        part1 = part1.min(mapset.propagate(*seed));
    }


    let mut part2 = i64::MAX;
    let mut seed_ranges: Vec<(i64, i64)> = vec![];
    for i in 0..seeds.len()-1 {
        if i % 2 != 0 { continue };
        println!("{:?} {:?}", seeds[i], seeds[i+1]);
        let start = seeds[i];
        let end = seeds[i] + seeds[i+1] - 1;
        seed_ranges.push((start, end));
    }
    println!("seed ranges: {:?}", seed_ranges);
    for i in 0..1000000000 {
        let seed = mapset.back_propagate(i);
        if i % 1000000 == 0 {
            println!("testing value {i}, produced by seed {seed}");
        };
        if seed_ranges.iter().any(|&(start, stop)| seed >= start && seed <= stop) {
            part2 = i;
            break;
        };
    }
    println!("part1: {part1}");
    println!("part2: {part2}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxymap() {
        let seeds: Vec<i64> = vec![79, 14, 55, 13];
        let mut seed_to_soil: ProxyMap = ProxyMap::new();
        seed_to_soil.add_map(50, 98, 2);
        seed_to_soil.add_map(52, 50, 48);
        assert_eq!(seed_to_soil.get(79), 81);
        assert_eq!(seed_to_soil.get(14), 14);
        assert_eq!(seed_to_soil.get(55), 57);
        assert_eq!(seed_to_soil.get(13), 13);
    }
}
