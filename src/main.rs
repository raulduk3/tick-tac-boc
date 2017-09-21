/*
	
	. . .
	. . .
	. . .

	x . x
	. x o
    x o o 

    Tick Tack Toe
*/
extern crate colored;
extern crate rand;

use std::fmt;
use colored::*;
use rand::Rng;

// Constants
const O : char = 'O';
const X : char = 'X';
const EMPTY : char = '.';

const SIZE : i32 = 3;

struct Bot
{
	mark: char
}


struct Board
{
	data: [[char; 3]; 3],
	players: Vec<Bot>,
	current_player_index: usize,
	first: usize
}

// Board methods
impl Board
{
	fn new() -> Board
	{
		let turn = rand::thread_rng().gen::<bool>();
		Board 
		{
			data: [[EMPTY; SIZE as usize]; SIZE as usize],
			players: vec![Bot { mark: X}, Bot { mark: O}, Bot { mark : '.' }],
			current_player_index: turn as usize,
			first: turn as usize
		}
	}

	fn is_full(&self) -> bool
	{
		for column in 0..SIZE
		{
			for row in 0..SIZE 
			{
				if self.is_empty(&(column as usize, row as usize))  
					{ return false; }
			}
		}
		return true;
	}

	// These two methofs are confusing as balls, the code
	// is not elegant. I will rewrite them later...
	fn is_row_win(&self) -> bool
	{
		for row in 0..SIZE
		{
			let mut count = 0;
			for column in 0 ..SIZE
			{
				let point = &(row as usize, column as usize);
				if self.get_mark(&point) != self.current_mark() || self.is_empty(&point)
				{
					break;
				}
				count += 1;
			}
			if count == 3
			{
				return true;
			}
		}
		return false;
	}

	fn is_column_win(&self) -> bool
	{
		for column in 0..SIZE
		{
			let mut count = 0;
			for row in 0 ..SIZE
			{
				let point = &(row as usize, column as usize);
				if self.get_mark(&point) != self.current_mark() || self.is_empty(&point)
				{
					break;
				}
				count += 1;
			}
			if count == 3
			{
				return true;
			}
		}
		false
	}

	fn is_diag_win(&self) -> bool
	{
		let current_mark = self.current_mark();

		if self.get_mark(&(0, 0)) == current_mark 
		{
			if self.get_mark(&(1, 1)) == current_mark
			{
				if self.get_mark(&(2, 2)) == current_mark
				{
					return true;
				}
				return false;
			}
			return false;
		}
		else if self.get_mark(&(2, 0)) == current_mark
		{
			if self.get_mark(&(1, 1)) == current_mark
			{
				if self.get_mark(&(0, 2)) == current_mark
				{
					return true;
				}
				return false;
			}
			return false;
		}
		else
		{
			return false;
		}
	}

	fn is_over(&self) -> bool
	{

		if self.is_row_win()
		{
			return true;
		} else if self.is_column_win()
		{
			return true;
		}
		else if self.is_diag_win()
		{
			return true;
		}
		else
		{
			return false;
		}
	}

	// Not needed, maybe in future?
	fn is_marked(&self, point: &(usize, usize)) -> bool
	{
		let &(column, row) = point;
		self.data[column][row] != EMPTY
	}
 
	fn is_empty(&self, point: &(usize, usize)) -> bool
	{
		let &(column, row) = point;
		self.data[column][row] == EMPTY
	}

	fn current_mark(&self) -> char { self.players[self.current_player_index].mark } 

	fn get_mark(&self, point: &(usize, usize)) -> char
	{
		let &(column, row) = point;
		self.data[column][row] 
	}

	fn mark(&mut self, point:(usize, usize))
	{
		let (column, row) = point;
		//println!("{}, {}", column, row);
		let current_player_mark = self.current_mark();
		self.data[column][row] = current_player_mark; 
	}

	fn change_turn(&mut self)
	{
		let opposite_player_index = (self.current_player_index + 1) % 2;
		self.current_player_index = opposite_player_index;
	}

	fn random_point() -> (usize, usize) { (rand::thread_rng().gen_range::<usize>(0, SIZE as usize), 
											rand::thread_rng().gen_range::<usize>(0, SIZE as usize)) }
}


// Display implemented for board
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for column in 0..SIZE
        {
        	for row in 0..SIZE {
        		let value = (self.data[column as usize][row as usize]).to_string();
        		if self.data[column as usize][row as usize] == X
        		{
        			write!(f, "{} ", value.red().bold())?;
        		}else if self.data[column as usize][row as usize] == O
        		{
        			write!(f, "{} ", value.blue().bold())?;
        		}else
        		{
        			write!(f, "{} ", value)?;
        		}
        	}
        	write!(f,"\n")?;
        }
        let winner_text = if self.current_mark() == X { X.to_string().normal() } else if self.current_mark() == O { O.to_string().normal() } else { "\n\n     /\\_/\\
    ( o o )
  x-==_Y_==-O
      `-'".to_string().yellow().dimmed()};
        write!(f, "{} went first and {} won\n", self.players[self.first].mark.to_string(), winner_text)
    }
}

fn run_game() -> (Board, i32)
{
	let mut board = Board::new();
	let mut counter = 0;
	// Main loop
	loop
	{
		let point = Board::random_point();

		if board.is_empty(&point)
		{
			board.mark(point);

			if board.is_over()
			{
				break;
			}

			board.change_turn();
		}
		else
		{
			if board.is_full()
			{
				board.current_player_index = 2;
				break;
			}
			continue;
		}
		counter += 1;
	}
	(board, counter)
}

// main
fn main()
{

	/*let mut three_games = Vec::new();

	while three_games.len() < 10
	{
		let b = run_game();
		if b.1 == 5
		{
			three_games.push(b)
		}
	}

	for board in three_games.iter()
	{
		println!("{}", board.0);
	}*/

	let b = run_game();
	println!("{}", b.0);

}