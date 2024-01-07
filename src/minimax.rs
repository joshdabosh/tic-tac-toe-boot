#[path="./util.rs"]
mod util;

extern crate alloc;

use alloc::vec::Vec;

use log::info;

pub fn make_ai_move(state: &mut [[u8; 3]; 3]) -> (usize, usize) {
    let mut final_move = -1;

    minimax(state, 2, &mut final_move);

    util::decompress_move(final_move)
}

fn minimax(
    state: &mut [[u8; 3]; 3],
    player: i32,
    mut final_move: &mut i32
) -> i32 {
    let mut available_moves: Vec<i32> = Vec::new();

    // find available moves on the board
    for i in 0..3 {
        for j in 0..3 {
            if state[i][j] == 0 {
                available_moves.push((i * 3 + j) as i32);
            }
        }
    }

    if available_moves.len() == 0 {
        // no more available moves, the game has ended
        return score_game(*state, player);
    }

    let mut scores: Vec<i32> = Vec::new();
    let mut moves: Vec<i32> = Vec::new();

    for m in available_moves {
        // explore move
        let (row_idx, col_idx) = util::decompress_move(m);

        state[row_idx][col_idx] = player as u8;

        scores.push(minimax(state, (player % 2) + 1, &mut final_move));
        moves.push(m);

        state[row_idx][col_idx] = 0;
    }

    if player == 2 {
        // making moves for the AI
        let mut max_idx = 0;
        let mut max_score = scores[0];

        for i in 0..scores.len() {
            if max_score < scores[i] {
                max_idx = i;
                max_score = scores[i];
            }
        }

        *final_move = moves[max_idx];

        max_score
    } else {
        // making moves for the human
        let mut min_idx = 0;
        let mut min_score = scores[0];

        for i in 0..scores.len() {
            if min_score > scores[i] {
                min_idx = i;
                min_score = scores[i];
            }
        }

        *final_move = moves[min_idx];

        min_score
    }
}

fn score_game(
    state: [[u8; 3]; 3],
    player: i32
) -> i32 {
    let w = winner(state);

    if w == player {
        return 10;
    } else if w == (player % 2) + 1 {
        return -10;
    }

    return 0;
}

pub fn winner(
    state: [[u8; 3]; 3]
) -> i32 {
    let mut x_count = 0;
    let mut o_count = 0;

    // check rows
    for i in 0..3 {
        for j in 0..3 {
            if state[i][j] == 1 {
                x_count += 1;
            } else if state[i][j] == 2 {
                o_count += 1;
            }
        }

        if x_count == 3 {
            return 1;
        }

        if o_count == 3 {
            return 2;
        }

        x_count = 0;
        o_count = 0;
    }

    // check columns
    x_count = 0;
    o_count = 0;
    for i in 0..3 {
        for j in 0..3 {
            if state[j][i] == 1 {
                x_count += 1;
            } else if state[j][i] == 2 {
                o_count += 1;
            }
        }

        if x_count == 3 {
            return 1;
        }

        if o_count == 3 {
            return 2;
        }

        x_count = 0;
        o_count = 0;
    }

    // check L-R diagonal
    x_count = 0;
    o_count = 0;
    for i in 0..3 {
        if state[i][i] == 1 {
            x_count += 1;
        } else if state[i][i] == 2 {
            o_count += 1;
        }
    }

    if x_count == 3 {
        return 1;
    } else if o_count == 3 {
        return 2;
    }


    // check R-L diagonal
    x_count = 0;
    o_count = 0;
    for i in 0..3 {
        if state[i][2-i] == 1 {
            x_count += 1;
        } else if state[i][2-i] == 2 {
            o_count += 1;
        }
    }

    if x_count == 3 {
        return 1;
    } else if o_count == 3 {
        return 2;
    }

    return 0;
}
