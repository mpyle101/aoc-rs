use pathfinding::matrix::Matrix;

fn main() {
    use std::fs;
    use std::time::Instant;
    
    let input = fs::read_to_string("./input.txt").unwrap();
    let cucumbers = load(&input);

    let t1 = Instant::now();
    let steps = part_one(&cucumbers);
    let t2 = Instant::now();
    println!("Part 1: {steps} {:?}", t2 - t1);
}

fn load(input: &str) -> Matrix<char> {
    Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap()
}

fn part_one(cucumbers: &Matrix<char>) -> i32 {
    let mut m = cucumbers.clone();

    let mut steps = 0;
    let mut moved = true;

    while moved { 
        steps += 1;
        moved = step_east(&m).map_or(false, |m1| { m = m1; true });
        moved = step_south(&m).map_or(moved, |m1| { m = m1; true });
    }
    
    steps
}

fn step_east(mat: &Matrix<char>) -> Option<Matrix<char>> {
    let mut m = mat.clone();

    let mut moved = false;
    mat.indices()
        .filter(|&rc| mat.get(rc).map_or(false, |v| *v == '>'))
        .for_each(|rc| {
            let rc1 = (rc.0, (rc.1 + 1) % m.columns);
            if mat.get(rc1).map_or(false, |v| *v == '.') { 
                moved = true;
                if let Some(v) = m.get_mut(rc)  { *v = '.' };
                if let Some(v) = m.get_mut(rc1) { *v = '>' };
            }
        });

    moved.then(|| m)
}

fn step_south(mat: &Matrix<char>) -> Option<Matrix<char>> {
    let mut m = mat.clone();

    let mut moved = false;
    mat.indices()
        .filter(|&rc| mat.get(rc).map_or(false, |v| *v == 'v'))
        .for_each(|rc| {
            let rc1 = ((rc.0 + 1) % m.rows, rc.1);
            if mat.get(rc1).map_or(false, |v| *v == '.') { 
                moved = true;
                if let Some(v) = m.get_mut(rc)  { *v = '.' };
                if let Some(v) = m.get_mut(rc1) { *v = 'v' };
            }
        });

    moved.then(|| m)
}

#[allow(dead_code)]
fn print(m: &Matrix<char>) {
    (0..m.rows).for_each(|r| {
        (0..m.columns).for_each(|c| {
            print!("{}", *m.get((r, c)).unwrap())
        });
        println!();
    });
    println!();
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn it_works() {
        let input = fs::read_to_string("./input.txt").unwrap();
        let cucumbers = load(&input);
    
        let steps = part_one(&cucumbers);
        assert_eq!(steps, 568);
    }

    #[test]
    fn small() {
        let input = fs::read_to_string("./test.txt").unwrap();
        let cucumbers = load(&input);
    
        let steps = part_one(&cucumbers);
        assert_eq!(steps, 58);
    }
}