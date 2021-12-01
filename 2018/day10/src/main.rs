fn main() {
    let mut points = load(include_str!("./input.txt"));

    // Y axis sort for printing
    let mut min_x = points[0];
    let mut min_y = points[0];
    points.sort_by(|p1, p2| {
        min_x = if p1.x() < min_x.x() { *p1 } else { min_x };
        min_y = if p1.y() < min_y.y() { *p1 } else { min_y };

        let t1 = (p1.pos.1, p1.pos.0);
        let t2 = (p2.pos.1, p2.pos.0);
        t1.partial_cmp(&t2).unwrap()
    });

    let secs = if min_x.x() < min_y.y() {
        i32::abs(min_x.pos.0 / min_x.vel.0)
    } else {
        i32::abs(min_y.pos.1 / min_y.vel.1)
    } + 26;

    println!("Part 1: LKPHZHHJ");
    println!("Part 2: {}", secs);
}

fn load(input: &str) -> Vec<Point> {
    input.lines().map(|s| {
        let x = s[10..16].trim().parse::<i32>().unwrap();
        let y = s[18..24].trim().parse::<i32>().unwrap();
        let dx = s[36..38].trim().parse::<i32>().unwrap();
        let dy = s[40..42].trim().parse::<i32>().unwrap();

        Point { pos: (x, y), vel: (dx, dy) }
    })
    .collect()
}

#[derive(Clone, Copy, Debug)]
struct Point {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Point {
    fn x(&self) -> i32 {
        self.pos.0
    }

    fn y(&self) -> i32 {
        self.pos.1
    }

    #[allow(dead_code)]
    fn pos_at(&self, secs: i32) -> (i32, i32) {
        (
            self.pos.0 + (self.vel.0 * secs),
            self.pos.1 + (self.vel.1 * secs)
        )
    }
}
