use crate::AOCDay;

use std::collections::HashMap;
use std::cmp::Ordering;

use nalgebra::base::{Vector3, Matrix3};

use nom::{
    bytes::complete::tag,
    sequence::{preceded, terminated},
    character::complete,
    error::Error,
};

use itertools::Itertools;

/*
 * Day 19: Beacon Scanner
 *
 * Beacons in various rotations measure several probes at a certain distance.
 * Find which beacons have overlapping measurements.
 */

pub struct Day19();

impl AOCDay for Day19 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 19 }
}

pub fn get() -> Day19 {Day19()}

#[derive(Debug)]
struct Scanner {
    id: u32,
    beacons: Vec<Vector3<i32>>,
}

#[derive(Debug)]
// Wrapper with additional info about the state of the scanner
struct ScannerInfo {
    scanner: Scanner,
    inner_distances: Vec<Distance>, // needs to be sorted by dist for optimal performance
    position: Option<Vector3<i32>>,
    orientation: Option<Matrix3<i32>>,
}

#[derive(Debug, Clone)]
struct Distance {
    src: usize,
    target: usize,
    dist: u32,
}

// The threshold for number of overlapping probes was 12, this constitutes to n*(n-1)/2 egdes.
const ALIGNMENT_THRESHOLD: u32 = 12;
const EDGE_THRESHOLD: u32 = ALIGNMENT_THRESHOLD * (ALIGNMENT_THRESHOLD - 1) / 2;

fn part1(input: &str) -> String {
    let scanners: Vec<ScannerInfo> = parse(&input).into_iter().map(analyze_scanner).collect();
    let aligned = align_scanners(scanners);
    // Check Number of Beacons
    let mut beacons = Vec::new();
    for scanner_info in aligned.values() {
        let mut bs: Vec<Vector3<i32>> = scanner_info.scanner.beacons.iter().map(|v| scanner_info.orientation.unwrap() * v + scanner_info.position.unwrap()).collect();
        beacons.append(&mut bs);
    }
    beacons.sort_by(|x,y|compare_vector(&x,&y));
    beacons.dedup_by(|x,y|compare_vector(&x,&y) == Ordering::Equal);
    beacons.len().to_string()
}

fn part2(input: &str) -> String {
    let scanners: Vec<ScannerInfo> = parse(&input).into_iter().map(analyze_scanner).collect();
    let aligned = align_scanners(scanners);

    let mut scanner_positions = Vec::new();
    for scanner_info in aligned.values() {
        scanner_positions.push(scanner_info.position.unwrap());
    }

    let mut max_dist = 0;
    while let Some(pos) = scanner_positions.pop() {
        for p in scanner_positions.iter() {
            max_dist = i32::max(max_dist, manhattan_distance(&pos, &p));
        }
    }
    max_dist.to_string()
}

/// Aligns `s1` with neighbour `s2`, requires that `s2` has a known position and orientation.
fn align_scanner(s1: &mut ScannerInfo, s2: &ScannerInfo) -> bool {
    assert!(s2.position.is_some() && s2.orientation.is_some(), "'s2' must have a known orientation and position");

    let mut s2_diffs: Vec<Vector3<i32>> = position_differences(&s2.scanner.beacons);
    s2_diffs.sort_by(|v1,v2| compare_vector(&v1,v2));

    // Step 1. Find correct configuration
    let mut rotational_alignment = false;
    let mut orientation = None;

    for rotation in possible_orientations() {
        // Apply rotation to beacon locations
        let mut beacons: Vec<Vector3<i32>> = s1.scanner.beacons.clone().into_iter().map(|v| rotation * v).collect();
        beacons.sort_by(|v1,v2| compare_vector(&v1,v2));
        // Compute new differences between beacons
        let mut diffs = position_differences(&beacons);
        diffs.sort_by(|v1,v2| compare_vector(&v1,v2));

        let eq_diffs = equal_vector_count(&diffs, &s2_diffs);

        if eq_diffs >= EDGE_THRESHOLD {
            rotational_alignment = true;
            orientation = Some(s2.orientation.unwrap() * rotation);
            break;
        }
    }
    if !rotational_alignment {
        return false;
    }

    // Step 2. Find offset, which causes probes to overlap
    let mut positional_alignment = false;
    let mut position: Option<Vector3<i32>> = None;

    let mut s2_beacons: Vec<Vector3<i32>> = s2.scanner.beacons.clone().into_iter().map(|v| s2.orientation.unwrap() * v).collect();
    s2_beacons.sort_by(|v1,v2|compare_vector(&v1,v2));

    let mut s1_beacons: Vec<Vector3<i32>> = s1.scanner.beacons.clone().into_iter().map(|v| orientation.unwrap() * v).collect();
    s1_beacons.sort_by(|v1,v2|compare_vector(&v1,v2));

    let mut stack = s2_beacons.clone();
    'outer: while let Some(s2_beacon) = stack.pop() {
        // Use s1_beacon as reference point
        for s1_beacon in s1_beacons.iter() {
            // align s2_beacon with s1_beacon and check whether alignment is correct
            let offset = s2_beacon - s1_beacon; // offset + x1 = x2 (if correct) 
            let mut aligned_beacons: Vec<Vector3<i32>> = s1_beacons.clone().into_iter().map(|v| offset + v).collect();
            aligned_beacons.sort_by(|x,y|compare_vector(&x,&y));

            let eq = equal_vector_count(&aligned_beacons, &s2_beacons);
            if eq >= ALIGNMENT_THRESHOLD {
                positional_alignment = true;
                position = Some(s2.position.unwrap() + offset);
                break 'outer;
            }
        }
    }
    if positional_alignment {
        s1.position = position;
        s1.orientation = orientation;
    }
    return positional_alignment;
}

/// Aligns all scanners
fn align_scanners(scanners: Vec<ScannerInfo>) -> HashMap<u32, ScannerInfo> {
    let mut unaligned = HashMap::new();
    let mut visited = HashMap::new();

    for s in scanners.into_iter() {
        unaligned.insert(s.scanner.id, s);
    }

    // Remove first scanner and make it the base reference frame
    let mut s0 = unaligned.remove(&0).unwrap();
    s0.position = Some(Vector3::from_element(0));
    s0.orientation = Some(Matrix3::identity());

    let mut queue = Vec::new();
    queue.push(s0);
    
    // Align scanners through graph traversal
    while let Some(scanner_info) = queue.pop() {
        // Visit node and try to align neighbouring nodes
        let ids = potential_neighbouring_scanners(&scanner_info, unaligned.values().collect());
        // Remove potential candidates
        for id in ids {
            let mut s = unaligned.remove(&id).unwrap(); // temporariliy take ownership of scanner
            let success = align_scanner(&mut s, &scanner_info);

            // Check whether scanner was aligned successfully
            if success {
                queue.push(s); // scanner can be visited next
            }else {
                unaligned.insert(id, s);
            }
        }
        visited.insert(scanner_info.scanner.id, scanner_info);
    }
    assert_eq!(unaligned.len(), 0, "There are still unaligned scanners left over");
    visited
} 

/// Uses the distances between beacons to find potential other scanners, which overlap their regions with
/// the scanner
///
/// Note: This function does not guarantee that two scanners overlap, but there is strong evidence
/// that two scanners might overlap. If a scanner is not in the result then it does definitely not
/// overlap with `s1`.
fn potential_neighbouring_scanners(s1: &ScannerInfo, scanners: Vec<&ScannerInfo>) -> Vec<u32> {
    let minimal_eq_dist = (12 * 11) / 2;
    let mut potential = Vec::new();
    for scanner_info in scanners {
        let eq_dist = equal_distance_count(s1, scanner_info);
        if eq_dist > minimal_eq_dist {
            potential.push(scanner_info.scanner.id);
        }
    }
    potential 
}

/// A prior analysis of a scanner and its probes
fn analyze_scanner(scanner: Scanner) -> ScannerInfo {
    let inner_distances = manhattan_distances(&scanner);
    return ScannerInfo {
        scanner,
        inner_distances,
        position: None, // unknown
        orientation: None, // unknown
    }
}

/// Manhattan distance between two vectors
fn manhattan_distance(v1: &Vector3<i32>, v2: &Vector3<i32>) -> i32 {
    v1.zip_map(v2, |a,b| i32::abs(a-b)).fold(0,|a,e| a+e)
}

/// Euclidean distance between two vectors
fn euclidean_distance(v1: &Vector3<i32>, v2: &Vector3<i32>) -> f32 {
    let f = v1.zip_map(v2, |a,b| i32::pow(a-b, 2)).fold(0,|a,e| a+e) as f32;
    f32::sqrt(f)
}

/// Computes a list of all distances between the beacons
fn manhattan_distances(scanner: &Scanner) -> Vec<Distance> {
    let mut distances = Vec::with_capacity(((scanner.beacons.len()-1) * scanner.beacons.len()) / 2);
    let mut stack: Vec<(usize, &Vector3<i32>)> = scanner.beacons.iter().enumerate().collect();
    while stack.len() > 1 {
        let (i1, beacon1) = stack.pop().unwrap();
        // compare element with elements left in stack
        for (i2, beacon2) in stack.iter() {
            let dist = manhattan_distance(beacon1, beacon2) as u32;
            distances.push(Distance{src: i1, target: *i2, dist});
        }
    }
    distances.sort_by(|d1,d2| d1.dist.cmp(&d2.dist));
    distances
}

/// Calculates the differences between the beacon locations
fn position_differences(beacons: &Vec<Vector3<i32>>) -> Vec<Vector3<i32>> {
    let mut differences = Vec::with_capacity(((beacons.len()-1) * beacons.len()) / 2);
    let mut stack: Vec<&Vector3<i32>> = beacons.iter().collect();
    while stack.len() > 1 {
        let beacon1 = stack.pop().unwrap();
        // compare element with elements left in stack
        for beacon2 in stack.iter() {
            differences.push(beacon1 - *beacon2);
        }
    }
    differences.sort_by(|v1,v2| compare_vector(&v1, &v2));
    differences
}

/// Computes the number of equal distances between beacons.
/// Useful heuristic for checking whether two scanners might have overlapping regions
fn equal_distance_count(s1: &ScannerInfo, s2: &ScannerInfo) -> u32 {
    let d1 = &s1.inner_distances; // inner_distances are sorted
    let d2 = &s2.inner_distances;
    let mut count = 0;
    let (mut i1, mut i2) = (0, 0);
    loop {
        if i1 >= d1.len() || i2 >= d2.len() {break;} // loop guard
        if d1[i1].dist == d2[i2].dist { // found an equal distance
            count+=1;
            i1 += 1;
            i2 += 1;
        }
        else if d1[i1].dist > d2[i2].dist {
            i2 += 1;
        }else {
            i1 += 1;
        }
    }
    count
}

/// Requires `s1` and `s2` are sorted
fn equal_vector_count(s1: &Vec<Vector3<i32>>, s2: &Vec<Vector3<i32>>) -> u32 {
    let d1 = s1; // inner_distances are sorted
    let d2 = s2;
    let mut count = 0;
    let (mut i1, mut i2) = (0, 0);
    loop {
        if i1 >= d1.len() || i2 >= d2.len() {break;} // loop guard
        if compare_vector(&d1[i1], &d2[i2]) == Ordering::Equal { // found an equal distance
            count+=1;
            i1 += 1;
            i2 += 1;
        }
        else if compare_vector(&d1[i1], &d2[i2]) == Ordering::Greater {
            i2 += 1;
        }else {
            i1 += 1;
        }
    }
    count
}

/// Returns all possible orientations that the scanner could be in the form of rotation matrices
fn possible_orientations() -> Vec<Matrix3<i32>> {
    let matrices: Vec<Matrix3<i32>> = vec![vec![1,0,0],vec![0,1,0],vec![0,0,1]].into_iter().permutations(3).map(|e| {
        Matrix3::from_iterator(e.concat().into_iter())
    }).map(|m| {
        let mut m2 = m.clone();
        multiply_row(&mut m2, 0, -1);
        vec![m, m2]
    }).flatten().map(|m| {
        let mut m2 = m.clone();
        multiply_row(&mut m2, 1, -1);
        vec![m, m2]
    }).flatten().map(|m| {
        let mut m2 = m.clone();
        multiply_row(&mut m2, 2, -1);
        vec![m, m2]
    }).flatten().filter(|m| det(&m) == 1).collect();
    matrices
}

fn multiply_row(matrix: &mut Matrix3<i32>, index: usize, scalar: i32) {
    for i in 0..3 {matrix[i*3+index] = matrix[i*3+index] * scalar;} // column major matrix
}

/// Determinant of 3x3 Matrix 
fn det(m: &Matrix3<i32>) -> i32 {
    let mut d = m[0] * (m[3*1+1]*m[3*2+2]-m[3*2+1]*m[3*1+2]);
    d = d - m[3]*(m[1]*m[8]-m[7]*m[2]);
    d = d + m[6]*(m[1]*m[5]-m[4]*m[2]);
    d
}

/// Utility function for comparing vectors (useful for sorting)
fn compare_vector(v1: &Vector3<i32>, v2: &Vector3<i32>) -> Ordering {
    if v1[0] == v2[0] {
        if v1[1] == v2[1] {
            if v1[2] == v2[2] { Ordering::Equal
            }else if v1[2] > v2[2] { Ordering::Greater
            }else { Ordering::Less}
        }else if v1[1] > v2[1] {Ordering::Greater
        }else {Ordering::Less}
    }else if v1[0] > v2[0] {Ordering::Greater
    }else {Ordering::Less}
}


// --- PARSING ---
fn parse_scanner_id(input: &str) -> u32 {
    let mut parser = preceded::<&str, _, _, Error<&str>, _, _>(tag("--- scanner "), terminated(complete::u32, tag(" ---")));
    parser(input).unwrap().1
}

fn parse_beacon(input: &str) -> Vector3<i32> {
    let mut coords = input.trim().split(',');
    let x = i32::from_str_radix(coords.next().unwrap(), 10).unwrap();
    let y = i32::from_str_radix(coords.next().unwrap(), 10).unwrap();
    let z = i32::from_str_radix(coords.next().unwrap(), 10).unwrap();
    Vector3::from_iterator(vec![x,y,z].into_iter())
}


fn parse(input: &str) -> Vec<Scanner> {
    let scans = input.split("\n\n");
    let mut scanners = Vec::new();

    for scan in scans {
        let mut lines = scan.lines();
        let heading = lines.next().unwrap();
        let id = parse_scanner_id(heading);
        let mut beacons = Vec::new();
        for beacon in lines {
            beacons.push(parse_beacon(beacon));
        }
        beacons.sort_by(|v1,v2| compare_vector(&v1, &v2));
        scanners.push(Scanner{id,beacons});
    }
    scanners
}

// --- Test Inputs ---
fn test_input_parsing() -> String {
    String::from("--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0")
}

fn test_input_example() -> String {
    String::from("--- scanner 0 ---
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
30,-46,-14")
}

fn test_input_simple() -> String {
    String::from("--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 1 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 2 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8")
}

// --- Tests ---
#[cfg(test)]
mod tests {

    use super::*;
   
    #[test] 
    fn dist() {
        let v1 = Vector3::from_iterator(vec![0,2,-2].into_iter());
        let v2 = Vector3::from_iterator(vec![0,-2,1].into_iter());
        assert_eq!(manhattan_distance(&v1, &v2), 7);
        assert_eq!(euclidean_distance(&v1, &v2), 5.0);
    }

    #[test] 
    fn matrix_ops() {
        let mut m1 = Matrix3::identity();
        multiply_row(&mut m1, 1, -1);
        println!("{:?}", m1);
        assert_eq!(det(&m1), -1, "Invalid Determinant");
        assert_eq!(possible_orientations().len(), 24, "Invalid number of orientations");
    }

    #[test] 
    fn utilities() {
        let v1 = Vector3::from_iterator(vec![0,2,-2].into_iter());
        let v2 = Vector3::from_iterator(vec![0,2,-2].into_iter());
        let v3 = Vector3::from_iterator(vec![-1,2,-2].into_iter());
        let vec1 = vec![v3.clone(), v2.clone()];
        let vec2 = vec![v1];
        assert_eq!(equal_vector_count(&vec1, &vec2), 1);
    }

}

