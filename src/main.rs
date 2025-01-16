use rocket::serde::json::Json; 
use rocket::serde::Deserialize;

#[macro_use] extern crate rocket;


#[get("/",rank = 2)]
fn default() -> String {
    format!("Hello, \n world!")
}


#[post("/", data = "<turn_status_jason>")]
fn post(turn_status_jason: Json<TurnStatus>) -> String {
    let turn_status:TurnStatus = turn_status_jason.into_inner();
    let mut turn_count = 10;
    for n in 0..9 {
        if turn_status.board_old[n] == 9{
            turn_count -= 1;
        }
    }
    let players_win_status: [u8;8];
    let current_player: char;
    let next_player: char;
    if turn_count%2 == 1 {
        current_player = 'X';
        next_player = 'O';
        players_win_status = add_to_win_status(turn_status.turn, turn_status.win_status_x);
    } else {
        current_player = 'O';
        next_player = 'X';
        players_win_status = add_to_win_status(turn_status.turn, turn_status.win_status_o);
    }

    let message = if check_for_win(players_win_status) {
                            format!("Player {} won!", current_player)
                    } else if turn_count == 9 {
                        format!("It's a tie!")
                    } else {
                        format!("The game goes on! It's your turn, {}", next_player)
                    };
    
    let mut  board_new = [9;9]; 
    let mut nine_counter = 0;
    for n in 0..9 {
        board_new[n] = if turn_status.board_old[n] == 9 {
                        if nine_counter > 0 {
                            break;
                        }
                        nine_counter += 1;
                        turn_status.turn
                    } else {
                        turn_status.board_old[n]
                    };
    }
    let board_display = into_board_display(board_new);
    
    format!("{}{}{}/n{}{}{}/n{}{}{}/n{}",
        board_display[0],
        board_display[1],
        board_display[2],
        board_display[3],
        board_display[4],
        board_display[5],
        board_display[6],
        board_display[7],
        board_display[8],
        message)

}


#[launch]
fn rocket() -> _ {
    rocket::build()
                .mount("/", routes![default,post])
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct TurnStatus {
    board_old: [usize;9],
    turn: usize,
    win_status_x: [u8;8],
    win_status_o: [u8;8]
}

//turns the board into the traditional displayable format with x,o and □ as empty positions
fn into_board_display(board:[usize;9]) -> [char;9]{
    let mut board_display: [char;9] = ['□';9];
    for n in 0..9 {
        if board[n] == 9 {
            continue;
        } else if n%2 == 0 {
            board_display[board[n]] = 'x';
        } else {
            board_display[board[n]] = 'o';
        }
    }
    board_display
}

//add a turn to a given win status
fn add_to_win_status(turn: usize, win_status: [u8;8]) -> [u8;8] { 
    let mut win_status_output: [u8; 8] =[0;8];
    let win_status_positions:[[u8;8];9] = [ 
        [1,0,0,1,0,0,1,0], //tl pos
        [1,0,0,0,1,0,0,0], //tm pos
        [1,0,0,0,0,1,0,1], //tr pos
        [0,1,0,1,0,1,0,0], //ml pos
        [0,1,0,1,0,1,1,1], //mm pos
        [0,1,0,0,0,1,0,0], //mr pos
        [0,0,1,1,0,0,0,1], //bl pos
        [0,0,1,0,1,0,0,0], //bm pos
        [0,0,1,0,0,1,1,0], //br pos
    ]; //it is [t_row,m_row,b_row,l_col,m_col,r_col,tl_diag,tr_diag] (t=top, m=mid, b=bottom, l=left, r=right)
    for n in 0..8 {
        win_status_output[n] = win_status[n] + win_status_positions[turn][n];
    }
    win_status_output
}

fn check_for_win (win_status: [u8;8]) -> bool {
    let mut max = 0;
    for n in 0..8 {
        if max <= win_status[n] {
            max = win_status[n]
        }
    }
    max == 3
}

//curl -X POST -H "Content-Type: application/json" -d @C:\Users\Schramm\source\repos\Rocket_2\src\game_state.json http://127.0.0.1:8000



