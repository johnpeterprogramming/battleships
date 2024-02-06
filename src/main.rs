use std::io;
use rand::Rng;

const ROWS: usize = 6;
const COLS: usize = 9;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Ship {
    cords: Vec<[usize; 2]>, // y, x
    id: u32, // make a char maby?
    hp: u32
}
impl Ship {
    fn new(cord: [usize; 2], direction: Direction, length: usize, id: u32) -> Ship { // Do with error handling later to check collisions
        let cords = match direction {
            Direction::Down => (cord[0]..cord[0]+length).map(|y| [y, cord[1]]).collect(),
            Direction::Up => (cord[0]+1-length..=cord[0]).map(|y| [y, cord[1]]).collect(),
            Direction::Right => (cord[1]..cord[1]+length).map(|x| [cord[0], x] ).collect(),
            Direction::Left => (cord[1]+1-length..=cord[1]).map(|x| [cord[0], x]).collect(),
        };
        Ship {
            cords,
            id,
            hp : length as u32
        }
    }
    fn check_collision(&self, cord: &[usize; 2]) -> bool {
        self.cords.contains(cord) 
    }
}

struct Board {
    grid : [[char; COLS]; ROWS],
    placed_ships: Vec<Ship>
}
impl Board {
    fn new() -> Board {
        Board {
            grid : [[' '; COLS]; ROWS],
            placed_ships : Vec::new()
        }
    }
    fn print_board(&self, hide: bool) {
        println!("      1 2 3 4 5 6 7 8 9 ");
        println!("    ---------------------");
        for i in 0..ROWS {
            print!("  {} | ", i+1);
            for j in 0..COLS {
                if hide {
                    match self.grid[i][j] {
                        'H'|'M' => print!("{} ", self.grid[i][j]),
                        _ => print!("  ")
                    }
                }
                else {
                    print!("{} ", self.grid[i][j]);
                } 
            }
            println!("| ");
        }
        println!("    ---------------------");
    }
    fn place_ship(&mut self, ship: Ship) -> bool {
        // Check for collisions
        for [i, j] in &ship.cords {
            if self.grid[*i][*j] != ' ' {
                return false;
            }
        }

        // Now place ship onto board
        for [i, j] in &ship.cords {
            self.grid[*i][*j] = char::from_u32(ship.id + 48 as u32).expect("couldn't convert ship id to char");
        }

        println!("Added {:?} sucessfuly", ship);
        self.placed_ships.push(ship);

        true
    }
    fn shoot(&mut self, cord: [usize; 2]) {
        // There must be a check before this funcion can be called so same coordinate can't be selected twice
        for ship in self.placed_ships.iter_mut() {
            if ship.check_collision(&cord) {
                println!("HIT a ship at {:?}", cord);
                ship.hp -= 1;
                if ship.hp == 0 {
                    println!("Ship has been SUNK!");
                }
                self.grid[cord[0]][cord[1]] = 'H';

                return;
            }
        }
        self.grid[cord[0]][cord[1]] = 'M';
    }
    fn check_win(&self) -> bool {
        for ship in self.placed_ships.iter() {
            if ship.hp != 0 {
                return false;
            }
        }
        true
    }
}


fn alternate_turn(mut turn: u8) -> u8{
    turn = (turn + 1) % 2;
    turn
}

fn main() {
    let mut player_1_board = Board::new();
    let mut player_2_board = Board::new();

    let mut turn = 0;
    let mut ship_id = 1;
    
    // AI ship selection, preset - make random later
    // let ship1 = Ship::new([0, 5], Direction::Left, 2, 1);
    let ship2 = Ship::new([2, 2], Direction::Down, 4, 2);
    // let ship3 = Ship::new([3, 3], Direction::Right, 5, 3);

    // player_2_board.place_ship(ship1);
    player_2_board.place_ship(ship2);
    // player_2_board.place_ship(ship3);
    // player_2_board.print_board(false);

    println!("Player {} is placing ships!", turn + 1);

    // Player 1 ship selection
    let mut user_input = String::new();
    for j in 0..1 {
        println!("Type the coordinate of the ship(Ex. 3,1): ");

        user_input.clear();
        io::stdin().read_line(&mut user_input).expect("couldn't read line");
        let mut cord:[usize; 2] = [0, 0];
        user_input.trim()
                .split(",")
                .filter_map(|x: &str| x.parse::<usize>().ok())
                .enumerate()
                .for_each(|(i, x)| cord[i] = x-1);

        println!("Type the Direction(r,l,u,d): ");
        user_input.clear();
        io::stdin().read_line(&mut user_input).expect("couldn't read line");

        let input_char = user_input.trim().chars().next().unwrap();
        let direction = match input_char {
            'r' => Direction::Right, 
            'l' => Direction::Left, 
            'u' => Direction::Up, 
            'd' => Direction::Down, 
            _ => panic!("invalid direction input.")
        };
        
        println!("Type the length: ");
        user_input.clear();
        io::stdin().read_line(&mut user_input).expect("couldn't read line");
        let length: usize = user_input.trim().parse::<usize>().expect("couldn't parse length");

        let user_ship = Ship::new(cord, direction, length, ship_id);
        ship_id += 1;

        player_1_board.place_ship(user_ship);
    }
    player_1_board.print_board(false);

    // Game loop can begin
    loop {
        println!("It is PLAYER {}'s turn!", turn + 1);

        if turn == 0 {
            println!("Player 2 BOARD");
            player_2_board.print_board(true);

            // Player 1 turn (human)
            println!("Make a guess: ");
            
            user_input.clear();
            io::stdin().read_line(&mut user_input).expect("couldn't read line");
            let mut cord:[usize; 2] = [0, 0];
            user_input.trim()
                .split(",")
                .filter_map(|x: &str| x.parse::<usize>().ok())
                .enumerate()
                .for_each(|(i, x)| cord[i] = x-1);

            // Do validation

            // Make shot
            player_2_board.shoot(cord);

            if player_2_board.check_win() {
                println!("Player 1 Has SUNK all the ships! GG!");
                player_2_board.print_board(true);
                break;
            }

        } 
        else {
            // Player 2 turn (AI)
            // println!("Player 1 BOARD");


            let rand_y = rand::thread_rng().gen_range(0..ROWS);
            let rand_x = rand::thread_rng().gen_range(0..COLS);
            let ai_cord = [rand_y, rand_x];

            player_1_board.shoot(ai_cord);
            println!("AI Made a move and this is how it looks:");
            player_1_board.print_board(false);

            if player_1_board.check_win() {
                println!("Player 2 Has SUNK all the ships! GG!");
                player_1_board.print_board(false);
                break;
            }
        }


        println!("\n\n");
        turn = alternate_turn(turn);
    }
    // do win checks!

}

