use crate::AOCDay;

use nom::{
    IResult,
    Parser,
    bytes::complete::{tag, take_till},
    branch::alt,
    sequence::{separated_pair, preceded, tuple},
    character::{complete, is_digit},

};

/*
 * Day 22: Reactor Reboot
 *
 * The reactor needs to be rebooted. This is done by turning on cubes specified by a cuboid region.
 */

pub struct Day22();

impl AOCDay for Day22 {
    fn part1(&self, _input: &str) -> Option<String> { Some(part1(_input)) }
    fn part2(&self, _input: &str) -> Option<String> { Some(part2(_input)) }
    fn get_num(&self) -> u32 { 22 }
}

pub fn get() -> Day22 {Day22()}

#[derive(Debug)]
struct Cuboid{
    xlim: (i32, i32), // (min, max)
    ylim: (i32, i32),
    zlim: (i32, i32),
}

impl Cuboid {
    /// Volume of cuboid in number of cubes
    fn volume(&self) -> u64 {
        let zlim = self.zlim;
        let ylim = self.ylim;
        let xlim = self.xlim;
        let z_width = (zlim.1 - zlim.0 + 1) as i64;
        let y_width = (ylim.1 - ylim.0 + 1) as i64;
        let x_width = (xlim.1 - xlim.0 + 1) as i64;
        (z_width * y_width * x_width) as u64
    }

    /// Intersection between two cuboids
    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.zlim.0 > other.zlim.1 || other.zlim.0 > self.zlim.1 {return None}; // no intersection
        if self.ylim.0 > other.ylim.1 || other.ylim.0 > self.ylim.1 {return None};
        if self.xlim.0 > other.xlim.1 || other.xlim.0 > self.xlim.1 {return None};
        // Cube must intersect
        let zlim_int = (i32::max(self.zlim.0, other.zlim.0), i32::min(self.zlim.1, other.zlim.1));
        let ylim_int = (i32::max(self.ylim.0, other.ylim.0), i32::min(self.ylim.1, other.ylim.1));
        let xlim_int = (i32::max(self.xlim.0, other.xlim.0), i32::min(self.xlim.1, other.xlim.1));
        Some(Cuboid {
            xlim: xlim_int,
            ylim: ylim_int,
            zlim: zlim_int,
        })
    }
}

#[derive(Debug)]
struct Instr(Cuboid, bool); // (cuboid, turn off/on)

/// Naive solution, quite ugly
fn part1(input: &str) -> String {
    let instructions = parse(&input);
    let mut space = Vec::new();
    for _ in -50..51 {
        let mut grid = Vec::new();
        for _ in -50..51 {
            let row = vec![false; 101];
            grid.push(row);
        }
        space.push(grid);
    }
    // follow instructions
    for instr in instructions {
        let (cuboid, on) = (instr.0, instr.1);
        let within_limit = within_limit(cuboid.zlim.0, -50, 50) && within_limit(cuboid.zlim.1, -50, 50) && within_limit(cuboid.ylim.0, -50, 50) && within_limit(cuboid.ylim.1, -50, 50) && within_limit(cuboid.xlim.0, -50, 50) && within_limit(cuboid.xlim.1, -50, 50);
        if within_limit {
            let (zmin, zmax) = (clamp(cuboid.zlim.0, -50, 50), clamp(cuboid.zlim.1, -50, 50));
            let (ymin, ymax) = (clamp(cuboid.ylim.0, -50, 50), clamp(cuboid.ylim.1, -50, 50));
            let (xmin, xmax) = (clamp(cuboid.xlim.0, -50, 50), clamp(cuboid.xlim.1, -50, 50));
            for z in (zmin+50)..(zmax+51) {
                for y in (ymin+50)..(ymax+51) {
                    for x in (xmin+50)..(xmax+51) {
                        space[z as usize][y as usize][x as usize] = on;
                    }
                }
            }
        }
    }
    let count: u64 = space.into_iter().flatten().flatten().map(|on| if on {1} else {0}).sum();
    format!("{}", count)
}

fn part2(input: &str) -> String {
    /*
     * Based on inclusion-exclusion principle. https://en.wikipedia.org/wiki/Inclusion%E2%80%93exclusion_principle
     */
    struct CuboidState {
        cuboid: Cuboid,
        add: bool,
    }

    let instructions = parse(&input);
    // Based
    let mut reactor: Vec<CuboidState> = Vec::new();
    for instr in instructions {
        // compute all intersections with the new cuboid
        let mut extra_cuboids = Vec::new();
        for c in reactor.iter() {
            let intersect = instr.0.intersection(&c.cuboid);
            if intersect.is_none() {continue;}
            let s = CuboidState{
                cuboid: intersect.unwrap(),
                add: !c.add};
            extra_cuboids.push(s);
        }
        // if instruction is on, add the cuboid itself as well
        if instr.1 {
            extra_cuboids.push(CuboidState{
                cuboid: instr.0,
                add: true
            });
        }
        reactor.append(&mut extra_cuboids);
    }
    // Count the number of on cubes
    let mut count: i64 = 0;
    for s in reactor {
        let sign = if s.add {1} else {-1};
        count += sign * (s.cuboid.volume() as i64);
    }
    count.to_string()
}

fn within_limit(i: i32, min: i32, max: i32) -> bool {
    i >= min && i <= max
}

fn clamp(i: i32, min: i32, max: i32) -> i32 {
    if i < min {min} else if i > max {max} else {i}
}

fn in_range(elem: i32, min: i32, max: i32) -> bool {
    if elem >= min && elem <= max {true} else {false}
}


// --- Parsing ---
fn parse_on_off(input: &str) -> IResult<&str, bool> {
    let on = tag("on").map(|_| true);
    let off = tag("off").map(|_| false);
    alt((on, off))(input)
}

fn parse_range(input: &str) -> IResult<&str, (i32, i32)> {
    let coord_bs = take_till(|c| (c == '-') || (is_digit(c as u8)));
    preceded(coord_bs, separated_pair(complete::i32, tag(".."), complete::i32))(input)
}

fn parse_instr(input: &str) -> IResult<&str, Instr> {
    let trimmed = input.trim();
    let mut parser = tuple((parse_on_off, parse_range, parse_range, parse_range)).map(|(on, (x1,x2), (y1,y2), (z1,z2))| {
        let cuboid = Cuboid {
            xlim: (x1, x2),
            ylim: (y1, y2),
            zlim: (z1, z2),
        };
        Instr(cuboid, on)
    });
    parser.parse(trimmed)
}

fn parse(input: &str) -> Vec<Instr> {
    let mut instrs = Vec::new();
    for line in input.lines() {
        instrs.push(parse_instr(line).unwrap().1);
    }
    instrs
}

fn test_input() -> String {
    String::from("on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682")
}

fn test_input2() -> String {
    String::from("on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cuboid() {
        let c1 = Cuboid {
            zlim: (-2,2),
            ylim: (-2,2),
            xlim: (-2,2),
        };
        let c2 = Cuboid {
            zlim: (2,5),
            ylim: (-4,-4),
            xlim: (-4,-2),
        };
        assert_eq!(c1.volume(), 125);
        assert_eq!(c2.volume(), 12);
        let c3 = Cuboid {
            xlim: (-2,2),
            ylim: (-1,3),
            zlim: (-1,3),
        };
        let intersection = c1.intersection(&c3);
        assert_eq!(intersection.unwrap().volume(), 80);

    }
}
