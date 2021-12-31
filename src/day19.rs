use crate::error;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Game {
    scanners: Vec<Vec<Vec3D>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct Vec3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Debug)]
struct DistanceAndPos {
    distance: Vec3D,
    pos: Vec3D,
}

impl std::fmt::Display for Vec3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}

impl std::fmt::Display for DistanceAndPos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=>{}", self.distance, self.pos)
    }
}

impl Vec3D {
    pub fn distance(&self) -> i64 {
        let a = self.x.pow(2);
        let b = self.y.pow(2);
        let c = self.z.pow(2);
        let distance = ((a + b + c) as f64).sqrt();
        distance.round() as i64
    }

    pub fn subtract(&self, rhs: &Vec3D) -> Vec3D {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Vec3D { x, y, z }
    }

    pub fn any_above(&self, value: i64) -> bool {
        self.x.abs() > value || self.y.abs() > value || self.z.abs() > value
    }

    pub fn move_to_scanner(&self, scanner_position: &Vec3D) -> Vec3D {
        Vec3D {
            x: self.x + scanner_position.x,
            y: self.y + scanner_position.y,
            z: self.z + scanner_position.z,
        }
    }

    pub fn transform_and_flip(&self, transformation: &ScannerTransformation) -> Vec3D {
        let mut new_pos = *self;

        new_pos = match transformation.rotation {
            ScannerRotation::XYZ => Vec3D {
                x: new_pos.x,
                y: new_pos.y,
                z: new_pos.z,
            },
            ScannerRotation::XZY => Vec3D {
                x: new_pos.x,
                y: new_pos.z,
                z: new_pos.y,
            },
            ScannerRotation::YXZ => Vec3D {
                x: new_pos.y,
                y: new_pos.x,
                z: new_pos.z,
            },
            ScannerRotation::YZX => Vec3D {
                x: new_pos.y,
                y: new_pos.z,
                z: new_pos.x,
            },
            ScannerRotation::ZXY => Vec3D {
                x: new_pos.z,
                y: new_pos.x,
                z: new_pos.y,
            },
            ScannerRotation::ZYX => Vec3D {
                x: new_pos.z,
                y: new_pos.y,
                z: new_pos.x,
            },
        };

        if transformation.flip_x {
            new_pos.x = -new_pos.x;
        }

        if transformation.flip_y {
            new_pos.y = -new_pos.y;
        }

        if transformation.flip_z {
            new_pos.z = -new_pos.z;
        }

        new_pos
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScannerRotation {
    XYZ,
    XZY,
    YXZ,
    YZX,
    ZXY,
    ZYX,
}

#[derive(Debug)]
pub struct ScannerTransformation {
    rotation: ScannerRotation,
    flip_x: bool,
    flip_y: bool,
    flip_z: bool,
}

impl std::fmt::Display for ScannerTransformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}|{}|{}|{}]", self.rotation, self.flip_x, self.flip_y, self.flip_z)
    }
}

impl std::str::FromStr for Game {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game { scanners: Vec::new() };

        let mut probes = Vec::new();

        for line in s.trim_start().trim_end().lines().filter(|l| !l.is_empty()) {
            if line.starts_with("---") {
                if !probes.is_empty() {
                    game.scanners.push(probes);
                }
                probes = Vec::new();
                continue;
            }
            let (x, y, z) = scan_fmt::scan_fmt!(line, "{d},{d},{d}", i64, i64, i64)?;
            probes.push(Vec3D { x, y, z })
        }

        game.scanners.push(probes);

        Ok(game)
    }
}

fn get_probe_distances_for_probe_index(probes: &Vec<Vec3D>, index: usize) -> Vec<DistanceAndPos> {
    let mut distances = Vec::with_capacity(probes.len());

    let base_probe = &probes[index];
    for probe in probes.iter() {
        distances.push(DistanceAndPos {
            distance: base_probe.subtract(probe),
            pos: *probe,
        });
    }

    distances
}

pub fn find_probe_indexes_with_enough_overlapping_probes(lhs: &Vec<Vec3D>, rhs: &Vec<Vec3D>) -> Option<(usize, usize)> {
    let mut counts = Vec::new();
    let range = 0..usize::max(lhs.len(), rhs.len());
    let mut done: HashSet<(usize, usize)> = HashSet::new();
    for indices in range.permutations(2) {
        let lhs_index = indices[0];
        let rhs_index = indices[1];
        if lhs_index >= lhs.len() || rhs_index >= rhs.len() {
            continue;
        }
        if done.contains(&(rhs_index, lhs_index)) {
            continue;
        }
        let lhs_distances = get_probe_distances_for_probe_index(lhs, lhs_index);
        let rhs_distances = get_probe_distances_for_probe_index(rhs, rhs_index);
        let lhs_set: HashSet<i64> = HashSet::from_iter(lhs_distances.iter().map(|x| x.distance.distance()));
        let mut count = 0;
        for distance in rhs_distances {
            if lhs_set.contains(&distance.distance.distance()) {
                count += 1;
            }
        }

        counts.push((count, lhs_index, rhs_index));
        done.insert((lhs_index, rhs_index));
    }

    counts.sort_by(|a, b| b.cmp(a));

    if counts.is_empty() {
        None
    } else {
        let (count, lhs_index, rhs_index) = counts[0];
        if count >= 12 {
            Some((lhs_index, rhs_index))
        } else {
            None
        }
    }
}

pub struct ConvertResult {
    probes: Vec<Vec3D>,
    scanner_position: Vec3D,
    scanner_transformation: ScannerTransformation,
}

pub fn convert_probes(scanner_from: &Vec<Vec3D>, scanner_to: &Vec<Vec3D>) -> Option<ConvertResult> {
    if let Some((probe_index_from, probe_index_to)) = find_probe_indexes_with_enough_overlapping_probes(scanner_from, scanner_to) {
        let distances_from = get_probe_distances_for_probe_index(scanner_from, probe_index_from);
        let distances_to = get_probe_distances_for_probe_index(scanner_to, probe_index_to);

        let map: HashMap<i64, Vec3D> = HashMap::from_iter(distances_to.iter().map(|x| (x.distance.distance(), x.pos)));
        let mut same_probes = Vec::new();
        for dist_and_pos in distances_from.iter() {
            if map.contains_key(&dist_and_pos.distance.distance()) {
                let to = map[&dist_and_pos.distance.distance()];
                let from = dist_and_pos.pos;
                same_probes.push(VecPair { to, from });
            }
        }

        if let Some((scanner_position, scanner_transformation)) = find_rhs_scanner_position_and_transformation(&same_probes) {
            let probes = scanner_from.iter().map(|&p| p.transform_and_flip(&scanner_transformation).move_to_scanner(&scanner_position)).collect();

            Some(ConvertResult {
                probes,
                scanner_position,
                scanner_transformation,
            })
        } else {
            panic!("failed to find scanner position and transformation for\nfrom: {:?}\nto: {:?}", &scanner_from, &scanner_to,);
        }
    } else {
        None
    }
}

struct VecPair {
    from: Vec3D,
    to: Vec3D,
}

fn find_rhs_scanner_position_and_transformation(positions: &Vec<VecPair>) -> Option<(Vec3D, ScannerTransformation)> {
    let rotations = vec![
        ScannerRotation::XYZ,
        ScannerRotation::XZY,
        ScannerRotation::YXZ,
        ScannerRotation::YZX,
        ScannerRotation::ZXY,
        ScannerRotation::ZYX,
    ];

    let flips = vec![
        (false, false, false),
        (false, false, true),
        (false, true, false),
        (false, true, true),
        (true, false, false),
        (true, false, true),
        (true, true, false),
        (true, true, true),
    ];

    for rotation in rotations.iter() {
        for &flip in flips.iter() {
            let transformation = ScannerTransformation {
                rotation: *rotation,
                flip_x: flip.0,
                flip_y: flip.1,
                flip_z: flip.2,
            };
            let mut sample = None;
            let num_matching = positions
                .windows(2)
                .filter(|window| {
                    let from_1 = window[0].from.transform_and_flip(&transformation);
                    let from_2 = window[1].from.transform_and_flip(&transformation);
                    let to_1 = window[0].to;
                    let to_2 = window[1].to;
                    let from_diff = from_1.subtract(&from_2);
                    let to_diff = to_1.subtract(&to_2);
                    if from_diff == to_diff && sample.is_none() {
                        sample = Some(VecPair { from: from_1, to: to_1 });
                    }
                    from_diff == to_diff
                })
                .count();
            if num_matching >= 7 {
                let sample = sample.unwrap();
                let scanner_position = sample.to.subtract(&sample.from);
                return Some((scanner_position, transformation));
            }
        }
    }

    None
}

fn build_graph(scanners: &Vec<Vec<Vec3D>>) -> petgraph::graph::UnGraph<u32, ()> {
    let mut edges = Vec::new();
    for indices in (0..scanners.len()).combinations(2) {
        let index_lhs = indices[0];
        let index_rhs = indices[1];
        let scanner_lhs = &scanners[index_lhs];
        let scanner_rhs = &scanners[index_rhs];
        if let Some((_, _)) = find_probe_indexes_with_enough_overlapping_probes(scanner_lhs, scanner_rhs) {
            edges.push((index_lhs as u32, index_rhs as u32));
        }
    }
    petgraph::graph::UnGraph::<u32, ()>::from_edges(edges)
}

pub fn count_same_probes(lhs: &Vec<Vec3D>, rhs: &Vec<Vec3D>) -> usize {
    let lhs_set: HashSet<&Vec3D> = HashSet::from_iter(lhs.iter());
    let mut count = 0;
    for vec in rhs.iter() {
        if lhs_set.contains(vec) {
            count += 1
        }
    }
    count
}

pub fn find_probes_and_scanners(scanners: &Vec<Vec<Vec3D>>) -> (Vec<Vec3D>, Vec<Vec3D>) {
    let graph = build_graph(scanners);

    let mut all_probes = Vec::new();
    let mut all_scanners = Vec::new();

    all_probes.append(&mut scanners[0].clone());

    for index in 1..scanners.len() {
        if let Some((_cost, path)) = petgraph::algo::astar(
            &graph,
            petgraph::visit::NodeIndexable::from_index(&graph, index),
            |finish| finish == petgraph::visit::NodeIndexable::from_index(&graph, 0),
            |_| 1,
            |_| 0,
        ) {
            let mut work_probes = None;
            let mut scanner = None;
            for index in path.windows(2) {
                let from = index[0].index() as usize;
                let to = index[1].index() as usize;
                let mut new_probes = scanners[from].clone();
                if work_probes.is_none() {
                    work_probes = Some(new_probes);
                } else {
                    let mut existing = work_probes.unwrap();
                    let count = count_same_probes(&existing, &new_probes);
                    if count != 12 {
                        panic!("count != 12");
                    }
                    existing.append(&mut new_probes);
                    work_probes = Some(existing);
                }
                if let Some(result) = convert_probes(work_probes.as_ref().unwrap(), &scanners[to]) {
                    work_probes = Some(result.probes);
                    if scanner.is_none() {
                        scanner = Some(result.scanner_position);
                    } else {
                        let old_scanner = scanner.unwrap();
                        scanner = Some(old_scanner.transform_and_flip(&result.scanner_transformation).move_to_scanner(&result.scanner_position));
                    }
                } else {
                    panic!("failed to convert probes");
                }
            }
            all_probes.append(&mut work_probes.unwrap());
            all_scanners.push(scanner.unwrap());
        } else {
            panic!("can't reach scanner {} from scanner {}", 0, index);
        }
    }

    all_probes.sort();
    all_probes.dedup();

    (all_probes, all_scanners)
}

fn manhattan_distance(lhs: &Vec3D, rhs: &Vec3D) -> i64 {
    let x = (lhs.x - rhs.x).abs();
    let y = (lhs.y - rhs.y).abs();
    let z = (lhs.z - rhs.z).abs();
    x + y + z
}

pub fn max_manhattan_distance(points: &Vec<Vec3D>) -> i64 {
    let mut max = 0i64;
    for indices in (0..points.len()).combinations(2) {
        let lhs = points[indices[0]];
        let rhs = points[indices[1]];
        let distance = manhattan_distance(&lhs, &rhs);
        if max < distance {
            max = distance;
        }
    }
    max
}

#[test]
fn test_scan_fmt() -> Result<(), error::Error> {
    let (x, y, z) = scan_fmt::scan_fmt!("404,-588,-901", "{d},{d},{d}", i64, i64, i64)?;
    assert_eq!(x, 404);
    assert_eq!(y, -588);
    assert_eq!(z, -901);
    Ok(())
}

#[test]
fn test_pos_transform() -> Result<(), error::Error> {
    assert_eq!(
        Vec3D { x: 5, y: 6, z: 2 }.transform_and_flip(&ScannerTransformation {
            rotation: ScannerRotation::ZYX,
            flip_x: true,
            flip_y: true,
            flip_z: true
        }),
        Vec3D { x: -2, y: -6, z: -5 }
    );
    assert_eq!(
        Vec3D { x: 1, y: 2, z: 3 }.transform_and_flip(&ScannerTransformation {
            rotation: ScannerRotation::XZY,
            flip_x: true,
            flip_y: false,
            flip_z: true
        }),
        Vec3D { x: -1, y: 3, z: -2 }
    );
    Ok(())
}

#[test]
fn test_pos_distance_to() -> Result<(), error::Error> {
    let pos1 = Vec3D { x: 5, y: 6, z: 2 };
    let pos2 = Vec3D { x: -7, y: 11, z: -13 };

    assert_eq!(pos1.subtract(&pos2).distance(), 20);

    Ok(())
}

#[test]
fn test_manhattan_distance() -> Result<(), error::Error> {
    let p1 = Vec3D { x: 1105, y: -1205, z: 1229 };

    let p2 = Vec3D { x: -92, y: -2380, z: -20 };

    assert_eq!(manhattan_distance(&p1, &p2), 3621);

    let cameras = vec![
        Vec3D { x: 68, y: -1246, z: -43 },
        Vec3D { x: 1105, y: -1205, z: 1229 },
        Vec3D { x: -92, y: -2380, z: -20 },
        Vec3D { x: -20, y: -1133, z: 1061 },
    ];

    assert_eq!(max_manhattan_distance(&cameras), 3621);

    Ok(())
}

#[test]
fn test_day19() -> Result<(), error::Error> {
    let input = r#"
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"#;

    let game: Game = input.parse()?;

    let result = convert_probes(&game.scanners[1], &game.scanners[0]).unwrap();

    let count = count_same_probes(&result.probes, &game.scanners[0]);
    assert_eq!(count, 12);

    assert_eq!(game.scanners.len(), 5);
    assert_eq!(game.scanners.iter().map(|s| s.len()).sum::<usize>(), 127);

    let (probes, scanners) = find_probes_and_scanners(&game.scanners);

    assert_eq!(probes.len(), 79);
    assert_eq!(max_manhattan_distance(&scanners), 3621);

    let game: Game = std::fs::read_to_string("input_day19")?.parse()?;

    assert_eq!(game.scanners.len(), 31);
    assert_eq!(game.scanners[30].len(), 26);

    let (probes, scanners) = find_probes_and_scanners(&game.scanners);
    assert_eq!(probes.len(), 376);
    assert_eq!(max_manhattan_distance(&scanners), 10772);

    Ok(())
}
