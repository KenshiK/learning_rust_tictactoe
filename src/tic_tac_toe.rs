pub mod tic_tac_toe {
    use std::io;
    static SPACE: char = ' ';
    static PLAYER1: char = 'X';
    static PLAYER2: char = 'O';

    pub fn game() {
        let mut game_state: [char; 9] = [SPACE;9];
        let mut turn = true;

        loop {
            print_game_state(&game_state);

            let p = set_character(turn);
            let pos = match select_case(p, &game_state) {
                Ok(n) => n,
                Err(message) => {
                    println!("{}", message);
                    wait_for_enter();
                    clear_console();
                    continue;
                }
            };

            game_state[pos] = p;

            if is_won(game_state) {
                print_game_state(&game_state);
                println!("Player {} won", p);
                break;
            }

            turn = !turn;
        }
    }

    fn is_won(game_state: [char; 9]) -> bool {
        return check_win_straight(game_state, 0, 7, 3, 1) //line
        || check_win_straight(game_state, 1, 3, 1, 3) //column
        || check_win(game_state, 0, 9, 4, 0) //diagonal from top left
        || check_win(game_state, 2, 5, 2, 2) //diagonal from top right
    }

    fn check_win(game_state: [char; 9], from: usize, to: usize, step: usize, base: usize) -> bool {
        let base_pos = base;
        let checked_character = game_state[base_pos];
        if checked_character == SPACE {
            return false;
        }
        let mut same_character = true;
        for i in ((base_pos + from)..(base_pos + to)).step_by(step) {
            if game_state[i] != checked_character {
                same_character = false;
                break;
            }
        }
        if same_character {
            return true;
        }
        false
    }

    fn check_win_straight(game_state: [char; 9], from: usize, to: usize, step: usize, base: usize) -> bool {
        for line in 0..3 {
            if check_win(game_state, from, to, step, line*base) {
                return true;
            }
        }
        false
    }

    fn wait_for_enter() {
        io::stdin().lines().next();
    }

    fn select_case(p: char, game_state: &[char; 9]) -> Result<usize, &'static str>{
        println!("Choose where to {} [0-{}] : ", p, game_state.len());
        let selected = read_usize();

        match selected < game_state.len() {
            true => {
                if game_state[selected] == SPACE {
                    return Ok(selected);
                }
                return Err("Already selected");
            },
            false => Err("Out of bond"),
        }
    }

    fn set_character(turn: bool) -> char {
        match turn {
            true => {
                PLAYER1
            }
            false => {
                PLAYER2
            }
        }
    }

    fn clear_console() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    fn print_game_state(game_state: &[char; 9]) {
        clear_console();
        print!("Tic Tac Toe \n");
        for n in 0..9 {
            if n % 3 == 2 {
                print!("{}\n", game_state[n]);
                if n != 8 {
                    print!("---------\n");
                }
        }
            else {
                print!("{} |", game_state[n]);
            }
        }
    }

    fn read_usize() -> usize {
        let line = io::stdin().lines().next().unwrap().unwrap();
        return line.parse().unwrap();
    }

    #[test]
    fn test_straight_line() {
        for line in 1..=3 {
            let mut game_state:[char; 9] = [SPACE;9];
            for i in line*0..line*3 { game_state[i] = PLAYER1;}
            print_game_state(&game_state);
            assert!(is_won(game_state));
        }
    }

    #[test]
    fn test_straight_column() {
        for column in 0..3 {
            let mut game_state:[char; 9] = [SPACE;9];
            for i in (column+0..column+7).step_by(3) { game_state[i] = PLAYER1;}
            print_game_state(&game_state);
            assert!(is_won(game_state));
        }
    }

    #[test]
    fn test_diagonal_from_top_left() {
        let mut game_state:[char; 9] = [SPACE;9];
        for i in (0..9).step_by(4) { game_state[i] = PLAYER1;}
        print_game_state(&game_state);
        assert!(is_won(game_state));
    }

    #[test]
    fn test_diagonal_from_top_right() {
        let mut game_state:[char; 9] = [SPACE;9];
        for i in (2..7).step_by(2) { game_state[i] = PLAYER1;}
        print_game_state(&game_state);
        assert!(is_won(game_state));
    }

}