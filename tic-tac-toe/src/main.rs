#[derive(Debug)]
enum GameState {
    Ongoing,
    Win(Player),
    Tie,
}

#[derive(Debug, PartialEq)]
enum Cell {
    Empty,
    X,
    O,
}

#[derive(Debug, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Debug)]
struct Score {
    player_x : u32,
    player_o : u32,
}

impl Player {
    fn other(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Debug)]
struct Board {
    cells: [[Cell; 3]; 3],
    current_player: Player,
    score : Score,
}

use std::io;

impl Board {
    fn render(&self) {
        println!("\\\\---------------- Tic - Tac - Toe -------------//\n\n");
        println!("XOXOOXOX------- Score Card -------XOXOXOX");

        println!("X : {}",self.score.player_x);
        println!("O : {}",self.score.player_o);

        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Empty => print!("   "),
                    Cell::X => print!(" X "),
                    Cell::O => print!(" O "),
                }
                if j < self.cells.len() - 1 {
                    print!("|");
                }
            }
            if i < self.cells.len() - 1 {
                println!("\n-----------");
            } else {
                println!("");
            }
        }
    }

    fn play(&mut self, row: usize, column: usize) {
        println!(
            "User {0:?} played : {row} row and {column} column",
            self.current_player
        );
        match self.cells[row][column] {
            Cell::Empty => {
                println!("Chance has been played successfully!");
                self.cells[row][column] = match self.current_player {
                    Player::X => Cell::X,
                    Player::O => Cell::O,
                };
                self.current_player = self.current_player.other();
            }
            _ => {
                println!(
                    "This position has already been played by {0:?}",
                    self.cells[row][column]
                );
            }
        }
    }

    fn reset_board(&mut self){
        self.cells = [
            [Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty],
        ];
        self.current_player = Player::X;
    }

    fn check_state(&self) -> GameState {
        //check win
        if Self::check_win(&self.cells, &self.current_player) {
            return GameState::Win(self.current_player.other());
        }

        // checks if the game is still ongoing
        let mut empty_count = 0;
        for row in &self.cells {
            for cell in row {
                 match cell {
                    Cell::Empty => empty_count+=1,
                    _ => ()
                }
            }
        }

        if empty_count > 0 {
            return GameState::Ongoing;
        }


        GameState::Tie
    }

    fn check_win(game_cells: &[[Cell; 3]; 3], current: &Player) -> bool{
        let target_cell = match current.other() {
            Player::X => Cell::X,
            Player::O => Cell::O,
        };
        // rows ---------
        for i in 0..3 {
            if game_cells[i][0] == target_cell
                && game_cells[i][1] == target_cell
                && game_cells[i][2] == target_cell
            {
                return true;
            }
        }
        //columns --------
        for i in 0..3 {
            if game_cells[0][i] == target_cell
                && game_cells[1][i] == target_cell
                && game_cells[2][i] == target_cell
            {
                return true;
            }
        }
        //diagonals -------
        if game_cells[0][0] == target_cell
            && game_cells[1][1] == target_cell
            && game_cells[2][2] == target_cell
        {
            return true;
        }

        if game_cells[2][0] == target_cell
            && game_cells[1][1] == target_cell
            && game_cells[0][2] == target_cell
        {
            return true;
        }

        false
    }
}

fn main() {
    let mut board = Board {
        cells: [
            [Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty],
        ],
        current_player: Player::X,
        score : Score {
            player_x : 0,
            player_o : 0,
        }
    };

    'game: loop {
        board.render();
        match board.check_state() {
            GameState::Ongoing => {
                println!(
                    "\n\n<<------ Its {0:?}'s turn ------>>\n\n",
                    board.current_player
                );
                println!("Enter Row: ");
                let mut row = String::new();
                let mut column = String::new();

                io::stdin()
                    .read_line(&mut row)
                    .expect("Failed to get input for row");

                let row = match row.trim().parse::<usize>() {
                    Ok(num) if (1..=3).contains(&num) => num,
                    Ok(_) | Err(_) => {
                        println!("Row number isnt valid");
                        continue;
                    }
                };
                let row = row - 1;

                println!("Enter Column: ");
                io::stdin()
                    .read_line(&mut column)
                    .expect("Failed to get input for column");

                let column = match column.trim().parse::<usize>() {
                    Ok(num) if (1..=3).contains(&num) => num,
                    Ok(_) | Err(_) => {
                        println!("Column number isnt valid");
                        continue;
                    }
                };
                let column = column - 1;

                board.play(row, column);
            }
            GameState::Win(winner) => {
                println!("Game is won by {0:?}",winner);
                match winner {
                Player::X => board.score.player_x += 1,
                Player::O => board.score.player_o += 1,
            }
                board.reset_board();
                continue;
            }
            GameState::Tie => {
                println!("Game is tied between {0:?} and {1:?}",board.current_player,board.current_player.other());
                board.reset_board();
                continue;
            }
        }
    }
}
