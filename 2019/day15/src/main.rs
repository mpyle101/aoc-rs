use anyhow::Result;
use std::collections::HashMap;
use std::io;
use vm::Vm;

fn main() {
    let program = include_str!("./program.txt");
    part_one(program).unwrap();

    let board = include_str!("./board.txt");
    let steps = part_two(board).unwrap();
    println!("Oxygen staturation: {steps} minutes");
}

fn part_one(program: &str) -> Result<(), &str> {
    let mut droid = Droid::from(program).or(Err("load failed"))?;
    droid.start().or(Err("start failed"))?;

    let mut board = Board::new();
    board.draw(&droid);

    let mut moves = 0;
    loop {
        moves += 1;
        let mut input = String::new();
        io::stdin().read_line(&mut input).or(Err("readline failed"))?;
        let cmd = match input.trim() {
            "q" => break,
            "w" => Some(Command::North),
            "a" => Some(Command::West),
            "s" => Some(Command::South),
            "d" => Some(Command::East),
            cmd => {
                println!("Unknown command: {}", cmd);
                None
            }
        };
        if let Some(cmd) = cmd {
            let result = droid.step(cmd)?;
            match result {
                Status::Wall   => board.update(&droid, Tile::Wall, Some(cmd)),
                Status::Moved  => board.update(&droid, Tile::Open, None),
                Status::Oxygen => board.update(&droid, Tile::Oxygen, None),
            }
        }

        board.draw(&droid);
        println!("Moves: {}", moves);
    };

    Ok(())
}

type TileMap = HashMap<(usize, usize), Tile>;
type TilePos = ((usize, usize), Tile);

fn part_two(board: &str) -> Result<i32, &str> {
    let mut tiles: TileMap = board.lines()
        .enumerate()
        .flat_map(move |(y, s)| s.chars().enumerate()
            .map(move |(x, c)| to_tile((x, y), c))
        )
        .collect::<Result<Vec<TilePos>, &str>>()?
        .into_iter()
        .collect();

    let oxygen = tiles.iter()
        .find_map(|(k, &v)|(v == Tile::Oxygen).then(|| k))
        .ok_or("no oxygen")?;

    let mut steps  = 0;
    let mut spread = open_tiles(&oxygen, &tiles);
    while spread.len() > 0 {
        steps += 1;
        spread.iter().for_each(|&pos| { tiles.insert(pos, Tile::Oxygen); });
        spread = spread.iter().flat_map(|p| open_tiles(p, &tiles)).collect();
    };


    Ok(steps)
}

fn to_tile(pos: (usize, usize), c: char) -> Result<TilePos, &'static str> {
    match c {
        '#' => Ok((pos, Tile::Wall)),
        '.' => Ok((pos, Tile::Open)),
        'O' => Ok((pos, Tile::Oxygen)),
        _  => Err("invalid tile")
    }
}

fn open_tiles(pos: &(usize, usize), tiles: &TileMap) -> Vec<(usize, usize)> {
    [
        (pos.0, pos.1 - 1),   // north
        (pos.0, pos.1 + 1),   // south
        (pos.0 + 1, pos.1),   // east
        (pos.0 - 1, pos.1),   // west
    ].iter()
    .filter_map(|&p| tiles.get(&p).filter(|&t| *t == Tile::Open).map(|_| p))
    .collect()
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Wall,
    Start,
    Oxygen,
}

#[derive(Clone, Copy)]
enum Command {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

#[derive(PartialEq)]
enum Status {
    Wall = 0,
    Moved = 1,
    Oxygen = 2,
}

fn step(pos: (i32, i32), cmd: Option<Command>) -> (i32, i32) {
    match cmd {
        Some(Command::North) => (pos.0, pos.1 - 1),
        Some(Command::South) => (pos.0, pos.1 + 1),
        Some(Command::East)  => (pos.0 + 1, pos.1),
        Some(Command::West)  => (pos.0 - 1, pos.1),
        None => pos,
    }
}

struct Board {
    tiles: HashMap<(i32, i32), Tile>,
    tl: (i32, i32),
    br: (i32, i32),
}

impl Board {
    fn new() -> Self {
        let mut tiles = HashMap::new();
        tiles.insert((0, 0), Tile::Start);

        Board { tiles, tl: (0, 0), br: (0, 0) }
    }

    fn draw(&self, droid: &Droid) {
        let y0 = self.tl.1;
        let y1 = self.br.1;
        let x0 = self.tl.0;
        let x1 = self.br.0;

        for y in y0..=y1 {
            let row: String = (x0..=x1).map(|x| match self.tiles.get(&(x, y)) {
                Some(Tile::Open)   if (x, y) == droid.pos => 'D',
                Some(Tile::Start)  if (x, y) == droid.pos => 'D',
                Some(Tile::Oxygen) if (x, y) == droid.pos => 'D',
                Some(Tile::Open)   => '.',
                Some(Tile::Wall)   => '#',
                Some(Tile::Start)  => 'S',
                Some(Tile::Oxygen) => 'O',
                None => '?',
            }).collect();
            println!("{}", row);
        };
    }

    fn update(&mut self, droid: &Droid, tile: Tile, cmd: Option<Command>) {
        let pos = match tile {
            Tile::Wall => {
                let pos = step(droid.pos, cmd);
                self.tiles.insert(pos, tile);
                pos
            },
            Tile::Open => {
                self.tiles.insert(droid.pos, tile);
                droid.pos
            },
            Tile::Oxygen => {
                self.tiles.insert(droid.pos, tile);
                droid.pos
            },
            Tile::Start => droid.pos,
        };
        self.tl.0 = self.tl.0.min(pos.0);
        self.tl.1 = self.tl.1.min(pos.1);
        self.br.0 = self.br.0.max(pos.0);
        self.br.1 = self.br.1.max(pos.1);
    }
}

struct Droid {
    vm: Vm,
    pos: (i32, i32),
}

impl Droid {
    fn from(program: &str) -> Result<Self> {
        Ok(Droid { vm: Vm::new(program)?, pos: (0, 0) })
    }

    fn start(&mut self) -> Result<()> {
        self.vm.exec()?;
        Ok(())
    }

    fn step(&mut self, cmd: Command) -> Result<Status, &'static str> {
        self.vm.write(cmd as i64);
        self.vm.cont().or(Err("continue failed"))?;
        let status = match self.vm.read().ok_or("no status")? {
            0 => Status::Wall,
            1 => Status::Moved,
            2 => Status::Oxygen,
            _ => return Err("invalid status"),
        };

        if status != Status::Wall {
            self.pos = step(self.pos, Some(cmd))
        }
    
        Ok(status) 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let board = include_str!("./board.txt");
        let steps = part_two(board).unwrap();

        assert_eq!(steps, 334)
    }
}