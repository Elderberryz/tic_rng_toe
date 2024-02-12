// tic_rng_toe is an implementation of the game tic-tac-toe (also known as noughts and crosses or Xs and Os) for one player,
// where the second player is represented by the computer, placing his marks in a pseudorandom fashion.
use rand::Rng;
use std::io;
use std::collections::HashMap;
    
fn main() {
    // Vector v is used to track the state of the game.
    // It contains the possible input values, which are the integers 1 up to and including 9.
    // After each input (from the player or computer), the input value is removed from the vector.
    // The game ends in a draw if the vector is empty.
    let mut v: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Array player is used to track the win condition of the player.
    // At the start of the game, it contains a zero value for each possible input value.
    // After player input, the corresponding zero value will be replaced by 1.
    let mut player: [u8; 9] = [0; 9];

    // Array computer is used to track the win condition of the computer.
    // It follows the same principle as the array player.
    let mut computer: [u8; 9] = [0; 9];

    // Variable turn counts the number of turns played.
    // A full turn ends if both player and computer have input a valid value and neither have won.
    // After three turns, either player or computer may have won so we start checking win conditions at that time.
    let mut turn: u8 = 0;

    // String board represents the game board.
    let mut board = String::from("   |   |  \n-----------\n   |   |  \n-----------\n   |   |  ");

    // This hash maps the possible input values to the indices in the string board.
    // E.g.: input value 2 corresponds to index 5 of string board.   
    let fields = HashMap::from([ 
	(1, 1),
	(2, 5),
	(3, 9),
	(4, 24),
	(5, 28),
	(6, 32),
	(7, 47),
	(8, 51),
	(9, 55)
    ]);

    // Adress of Welcome written to stdout.
    println!("################");
    println!("# Tic-rng-toe! #");
    println!("################\n");
    println!(" 1 | 2 | 3\n-----------\n 4 | 5 | 6\n-----------\n 7 | 8 | 9\n");
    println!("Choose a number (1..9) to place the first mark and start the game!");

    // Start of the game loop.
    // The loop is broken if either player or computer wins or if there is a draw.
    loop {

	// String number will hold the input value from player.
	println!("\nInput a valid number to continue.");
	let mut number = String::new();

	// Read from stdin into number.
	io::stdin()
	    .read_line(&mut number)
	    .expect("Failed to read line");

	// If number is not a number of type u8: restart loop.
	let number: u8 = match number.trim().parse() {
	    Ok(num) => num,
	    Err(_) => {
		println!("Invalid input: Not a valid input number (1..9)");
		continue; },
	};

	// If number is not contained in v: restart loop.
	if !v.contains(&number) {
	    println!("Invalid input: The chosen field does not exist (1..9) OR it was already marked in a previous turn.");
	    continue;
	}

	// At this point v contains number thus input is valid.
	// Increase turn count by 1.
	turn += 1;

	// Save index of input value in string board in variable field.
	let field = fields.get(&number).copied().unwrap_or(0);

	// Update the board state.
	// That is, replace the character in string board at index field by the character 'X'.
	board = replace_char_at(&board, field, 'X');

	// Update the game state.
	// That is, remove input value from game state vector v.
	v.retain(|x| x != &number);

	// Replace zero value in array player that corresponds to input value by 1.
	player[usize::from(number) - 1] = 1;

	// Write current board state to stdout.
	println!("\nYour turn:\n");
	println!("{}", board);

	// After turn 3, start to check if player has won.
	// If player has won, break the game loop.
	// Else, check if the game ends in a draw.
	if turn >= 3 {
	    if player[0] + player[1] + player[2] == 3 || player[0] + player[3] + player[6] == 3 || player[0] + player[4] + player[8] == 3 || player[3] + player[4] + player[5] == 3 || player[1] + player[4] + player[7] == 3 || player[2] + player[5] + player[8] == 3 || player[6] + player[7] + player[8] == 3 || player[2] + player[4] + player[6] == 3 {
		println!("\n#############################");
		println!("# Congratulations! You won! #");
		println!("#############################\n");
		break;
	    } else if v.len() == 0 {
		println!("\n###################");
		println!("# So close! Draw. #");
		println!("###################\n");
		break;
	    }
	}

	// Compute upper bound of the range of possible input values.
	let upper_bound: u8 = (v.len() - 1).try_into().unwrap();

	// Compute index of input value for computer.
	// That is, draw a pseudorandom number in the range 1 up to and including the upper_bound.
	let comp_index : u8 = rand::thread_rng().gen_range(1..=upper_bound);

	// Compute input value for computer.
	let comp_size = usize::from(comp_index);
	let comp_pick : u8 = v[comp_size];

	// Save index of input value in string board in variable comp_field.
	let comp_field = fields.get(&comp_pick).copied().unwrap_or(0);

	// Update the board state.
	// That is, replace the character in string board at index comp_field by the character 'O'.
	board = replace_char_at(&board, comp_field, 'O');

	// Update the game state.
	// That is, remove input value from game state vector v.
	v.retain(|x| x != &comp_pick);

	// Replace zero value in array computer that corresponds to input value by 1.
	computer[usize::from(comp_pick) - 1] = 1;

	// Write current board state to stdout.
	println!("\n\nComputer turn:\n");
	println!("{}", board);

	// After turn 3, start to check if computer has won.
	// If computer has won, break the game loop.
	if turn >= 3 {
	    if computer[0] + computer[1] + computer[2] == 3 || computer[0] + computer[3] + computer[6] == 3 || computer[0] + computer[4] + computer[8] == 3 || computer[3] + computer[4] + computer[5] == 3 || computer[1] + computer[4] + computer[7] == 3 || computer[2] + computer[5] + computer[8] == 3 || computer[6] + computer[7] + computer[8] == 3 || computer[2] + computer[4] + computer[6] == 3 {
		println!("\n############################");
		println!("# Computer won. Try again! #");
		println!("############################\n");
		break;
	    }
	}
    }
}
// Replace character at index idx in string s by character c
fn replace_char_at(s: &str, idx: u8, c: char) -> String {
    let mut r = String::with_capacity(s.len());
    for (i, d) in s.char_indices() {
        r.push(if i == idx.into() { c } else { d });
    }
    r
}


