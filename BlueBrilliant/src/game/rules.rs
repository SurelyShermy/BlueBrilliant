/*
This file will contain all of the move generation logic
As of 12/27 we are using the hex board implementation instead of the bitboard implementation


*/
use crate::game::initialization::*;
use crate::game::utilities::*;
pub fn can_castle(board_state: &BoardState, position: usize, direction: String, color: u8) -> bool {
    let squares = Vec::new();
    let attacker = if (color == WHITE) { BLACK } else { WHITE };
    if (board_state.check == true) {
        return false;
    }
    // if(color == WHITE && board_state.castling_white.king_side == false){
    //     return false;
    // }else if(color == BLACK && board_state.castling_black.king_side == false){
    //     return false;
    // }
    // if(color)
    if (direction == "King") {
        squares.push(position + 1);
        squares.push(position + 2);
        if (board_state.board[position + 3] != (color | ROOK)) {
            return false;
        }
    } else {
        squares.push(position - 1);
        squares.push(position - 2);
        squares.push(position - 3);
        if (board_state.board[position - 4] != (color | ROOK)) {
            return false;
        }
    }
    if (color == WHITE) {
        if (direction == "King" && !board_state.castling_white.king_side) {
            return false;
        }
        if (direction == "Queen" && !board_state.castling_white.queen_side) {
            return false;
        }
    } else {
        if (direction == "King" && !board_state.castling_black.king_side) {
            return false;
        }
        if (direction == "Queen" && !board_state.castling_black.queen_side) {
            return false;
        }
    }
    for i in 0..squares.len() {
        if (board_state.board[squares[i]] != 0) {
            return false;
        }
    }
    if (direction == "Queen") {
        squares.pop();
    }
    for i in 0..squares.len() {
        if (is_square_under_attack(board_state, squares[i], attacker)) {
            return false;
        }
    }
    return true;
}
pub fn knight_move_generator(
    board_state: &BoardState,
    position: usize,
    defender: bool,
) -> Vec<usize> {
    let mut valid_moves = Vec::new();
    let piece = board_state.board[position];
    let color = piece_color(piece);
    //Only the king can move in a double check
    if (board_state.check && board_state.checking_squares.length() > 1 && defender == false) {
        return valid_moves;
    }
    if (defender == false) {
        //If a knight is pinned, it cannot move under any circumstance
        //Might need to be optimized?
        for i in 0..board_state.pinned_pieces.len() {
            if (board_state.pinned_pieces[i] == position as u8) {
                return valid_moves;
            }
        }
    }
    //path is the vector that is attacking the king, thus any move that either blocks the path or captures the piece attacking the king is a valid move

    for i in 0..KNIGHT_MOVES.len() {
        let new_position = position + KNIGHT_MOVES[i];
        if (on_board(new_position as i16)) {
            let dest_piece = board_state.board[new_position];
            //when defender mode is on, we add the destination index no matter what
            if (piece_color(dest_piece) == color && defender == true) {
                valid_moves.push(new_position);
                continue;
            }
            //if the desintation is empty or the destination is an enemy piece, we add the destination index
            if (dest_piece == 0 || piece_color(dest_piece) != color) {
                if (board_state.check == true) {
                    valid_moves.push(new_position);
                }
            }
        }
    }
    if (board_state.check == true) {
        let check_moves = Vec::new();
        let path = calculate_path(board_state);
        for i in 0..valid_moves.len() {
            if (path.includes(valid_moves[i])) {
                check_moves.push(valid_moves[i]);
            }
        }
        return check_moves;
    }
    valid_moves
}

pub fn pawn_move_generator(
    board_state: &BoardState,
    position: usize,
    defender: bool,
) -> Vec<usize> {
    let mut valid_moves = Vec::new();
    let piece = board_state.board[position];
    let color = piece_color(piece);
    let player_king_pos = if (color == WHITE) {
        board_state.white_king_pos
    } else {
        board_state.black_king_pos
    };
    let attacker = if (color == WHITE) { BLACK } else { WHITE };
    //Only the king can move in a double check
    if (board_state.check && board_state.checking_squares.length() > 1 && defender == false) {
        return valid_moves;
    }
    if (board_state.check) {
        let path = calculate_path(board_state);
    }
    //Init moves depending on color
    if (color == WHITE) {
        let forward = 16;
        let double_forward = 32;
        let left_attack = 15;
        let right_attack = 17;
    } else {
        let forward = -16;
        let double_forward = -32;
        let left_attack = -17;
        let right_attack = -15;
    }
    let skip_captures = false;
    let skip_forward = false;
    let skip_left = false;
    let skip_right = false;
    if (defender == false) {
        for i in 0..board_state.pinned_pieces.len() {
            if (board_state.pinned_pieces[i][0] == position as u8) {
                let dir = board_state.pinned_pieces[i][1];
                if (dir == N || dir == S) {
                    skip_captures = true;
                }
                if (dir == SE || dir == SW || dir == NW || dir == NE) {
                    skip_forward = true;
                    if (dir == SE || dir == NW) {
                        if (color == WHITE) {
                            skip_right = true;
                        } else {
                            skip_left = true;
                        }
                    }
                    if (dir == SW || dir == NE) {
                        if (color == WHITE) {
                            skip_left = true;
                        } else {
                            skip_right = true;
                        }
                    }
                }
                if (dir == E || dir == W) {
                    skip_forward = true;
                    skip_captures = true;
                }
            }
            break;
        }
    }
    //Forwards
    if (!skip_forward && !defender) {
        let mut new_position = position + forward;
        let mut dest_piece = board_state.board[new_position];
        if (on_board(new_position as i16) && dest_piece == 0) {
            //Makes sure that it doesnt include a capture
            if (board_state.check == true) {
                if (path.includes(new_position)) {
                    valid_moves.push(new_position);
                }
            } else {
                valid_moves.push(new_position);
            }
            //Starting Square check
            if ((floor(position / 16) == 1 && color == WHITE)
                || (floor(position / 16) == 6 && color == BLACK))
            {
                new_position = position + double_forward;
                if (on_board(new_position as i16)) {
                    dest_piece = board_state.board[new_position];
                    if (dest_piece == 0) {
                        //Makes sure that it doesnt include a capture
                        if (board_state.check == true) {
                            //means the move will block the check
                            if (path.includes(new_position)) {
                                valid_moves.push(new_position);
                            }
                        } else {
                            valid_moves.push(new_position);
                        }
                    }
                }
            }
        }
    }
    if (!skip_captures || defender) {
        if (!skip_right || defender) {
            let mut new_position = position + right_attack;
            let mut dest_piece = board_state.board[new_position];
            //if we are on the board and the destination is an enemy piece or an enpassant square or defender mode is on
            if (on_board(new_position as i16)
                && ((piece_color(dest_piece) != color || board_state.enpassant == new_position)
                    || (defender)))
            {
                //Makes sure that it doesnt include a capture
                if (board_state.check == true) {
                    //means we are capturing the checking piece
                    if (path.includes(new_position)) {
                        valid_moves.push(new_position);
                    }
                }
                if (board_state.enpassant == new_position) {
                    let mut temp_boardstate = board_state.copy_board_state();
                    temp_boardstate.board[position] = 0;
                    temp_boardstate.board[new_position] = piece;
                    temp_boardstate.board[position + 1] = 0;
                    for i in 0..128 {
                        if (onboard(i) == false) {
                            continue;
                        }
                        if (isSlider(temp_boardstate.board[i])) {
                            temp_boardstate.check = false;
                            //TODO: STUB FUNCTION DOESNT EXIST YET
                            update_attack_table(&temp_boardstate, i);
                        }
                    }
                    temp_boardstate.check = false;
                    if (!is_square_under_attack(&temp_boardstate, player_king_pos, attacker)) {
                        valid_moves.push(new_position);
                    }
                }
                if (defender) {
                    valid_moves.push(new_position);
                } else if (piece_color(dest_piece) == attacker) {
                    valid_moves.push(new_position);
                }
            }
        }
        if (!skip_left || defender) {
            let mut new_position = position + left_attack;
            let mut dest_piece = board_state.board[new_position];
            //if we are on the board and the destination is an enemy piece or an enpassant square or defender mode is on
            if (on_board(new_position as i16)
                && ((piece_color(dest_piece) != color || board_state.enpassant == new_position)
                    || (defender)))
            {
                //Makes sure that it doesnt include a capture
                if (board_state.check == true) {
                    //means we are capturing the checking piece
                    if (path.includes(new_position)) {
                        valid_moves.push(new_position);
                    }
                }
                if (board_state.enpassant == new_position) {
                    let mut temp_boardstate = board_state.copy_board_state();
                    temp_boardstate.board[position] = 0;
                    temp_boardstate.board[new_position] = piece;
                    temp_boardstate.board[position + 1] = 0;
                    for i in 0..128 {
                        if (onboard(i) == false) {
                            continue;
                        }
                        if (isSlider(temp_boardstate.board[i])) {
                            temp_boardstate.check = false;
                            //TODO: STUB FUNCTION DOESNT EXIST YET
                            update_attack_table(&temp_boardstate, i);
                        }
                    }
                    temp_boardstate.check = false;
                    if (!is_square_under_attack(&temp_boardstate, player_king_pos, attacker)) {
                        valid_moves.push(new_position);
                    }
                }
                if (defender) {
                    valid_moves.push(new_position);
                } else if (piece_color(dest_piece) == attacker) {
                    valid_moves.push(new_position);
                }
            }
        }
    }
    valid_moves
}
pub fn bishop_move_generator(
    board_state: &BoardState,
    position: usize,
    defender: bool,
) -> Vec<usize> {
    let mut valid_moves = Vec::new();
    let piece = board_state.board[position];
    let color = piece_color(piece);
    let player_king_pos = if (color == WHITE) {
        board_state.white_king_pos
    } else {
        board_state.black_king_pos
    };
    let attacker = if (color == WHITE) { BLACK } else { WHITE };
    //Only the king can move in a double check
    if (board_state.check && board_state.checking_squares.length() > 1 && defender == false) {
        return valid_moves;
    }
    if (board_state.check) {
        let path = calculate_path(board_state);
    }
    let mut directions = Vec::new();
    if (!defender) {
        for i in 0..board_state.pinned_pieces.len() {
            if (board_state.pinned_pieces[i][0] == position as u8) {
                let dir = board_state.pinned_pieces[i][1];
                if (dir == N || dir == S || dir == E || dir == W) {
                    return valid_moves;
                }
                if (dir == SE || dir == NW) {
                    directions.push(SE);
                    directions.push(NW);
                } else if (dir == SW || dir == NE) {
                    directions.push(SW);
                    directions.push(NE);
                }
            }
            break;
        }
    }

    for i in 0..directions.len() {
        let mut current_position = position;
        while (true) {
            let new_position = current_position + directions[i];
            if (!on_board(new_position)) {
                break;
            }
            let dest_piece = board_state.board[new_position];
            if (piece_color(dest_piece) == color) {
                if (defender) {
                    valid_moves.push(new_position);
                    break;
                } else {
                    break;
                }
            } else if (piece_color == attacker) {
                valid_moves.push(new_position);
                break;
            } else {
                valid_moves.push(new_position);
            }
            current_position = new_position;
        }
    }
    if (board_state.check == true) {
        let check_moves = Vec::new();
        let path = calculate_path(board_state);
        for i in 0..valid_moves.len() {
            if (path.includes(valid_moves[i])) {
                check_moves.push(valid_moves[i]);
            }
        }
        return check_moves;
    }
    valid_moves
}
pub fn queen_move_generator(
    board_state: &BoardState,
    position: usize,
    defender: bool,
) -> Vec<usize> {
    let mut valid_moves = Vec::new();
    let piece = board_state.board[position];
    let color = piece_color(piece);
    let player_king_pos = if (color == WHITE) {
        board_state.white_king_pos
    } else {
        board_state.black_king_pos
    };
    let attacker = if (color == WHITE) { BLACK } else { WHITE };
    //Only the king can move in a double check
    if (board_state.check && board_state.checking_squares.length() > 1 && defender == false) {
        return valid_moves;
    }
    if (board_state.check) {
        let path = calculate_path(board_state);
    }
    let mut directions = Vec::new();
    if (!defender) {
        for i in 0..board_state.pinned_pieces.len() {
            if (board_state.pinned_pieces[i][0] == position as u8) {
                let dir = board_state.pinned_pieces[i][1];
                if (dir == N || dir == S) {
                    directions.push(N);
                    directions.push(S);
                } else if (dir == E || dir == W) {
                    directions.push(E);
                    directions.push(W);
                } else if (dir == SE || dir == NW) {
                    directions.push(SE);
                    directions.push(NW);
                } else if (dir == SW || dir == NE) {
                    directions.push(SW);
                    directions.push(NE);
                }
                break;
            }
        }
    }
    if (board_state.check) {
        let path = calculate_path(board_state);
    }
    for i in 0..directions.len() {
        let mut current_position = position;
        while (true) {
            let new_position = current_position + directions[i];
            if (!on_board(new_position)) {
                break;
            }
            let dest_piece = board_state.board[new_position];
            if (piece_color(dest_piece) == color) {
                if (defender) {
                    valid_moves.push(new_position);
                    break;
                } else {
                    break;
                }
            } else if (piece_color == attacker) {
                valid_moves.push(new_position);
                break;
            } else {
                valid_moves.push(new_position);
            }
            current_position = new_position;
        }
    }
    if (board_state.check == true) {
        let check_moves = Vec::new();
        let path = calculate_path(board_state);
        for i in 0..valid_moves.len() {
            if (path.includes(valid_moves[i])) {
                check_moves.push(valid_moves[i]);
            }
        }
        return check_moves;
    }
    valid_moves
}
pub fn rook_move_generator(
    board_state: &BoardState,
    position: usize,
    defender: bool,
) -> Vec<usize> {
    let mut valid_moves = Vec::new();
    let piece = board_state.board[position];
    let color = piece_color(piece);
    let player_king_pos = if (color == WHITE) {
        board_state.white_king_pos
    } else {
        board_state.black_king_pos
    };
    let attacker = if (color == WHITE) { BLACK } else { WHITE };
    //Only the king can move in a double check
    if (board_state.check && board_state.checking_squares.length() > 1 && defender == false) {
        return valid_moves;
    }
    if (board_state.check) {
        let path = calculate_path(board_state);
    }
    let mut directions = Vec::new();
    if (!defender) {
        for i in 0..board_state.pinned_pieces.len() {
            if (board_state.pinned_pieces[i][0] == position as u8) {
                let dir = board_state.pinned_pieces[i][1];
                if (dir == N || dir == S) {
                    directions.push(N);
                    directions.push(S);
                } else if (dir == E || dir == W) {
                    directions.push(E);
                    directions.push(W);
                } else {
                    return valid_moves;
                }
            }
            break;
        }
    }
    if (board_state.check) {
        let path = calculate_path(board_state);
    }
    for i in 0..directions.len() {
        let mut current_position = position;
        while (true) {
            let new_position = current_position + directions[i];
            if (!on_board(new_position)) {
                break;
            }
            let dest_piece = board_state.board[new_position];
            if (piece_color(dest_piece) == color) {
                if (defender) {
                    valid_moves.push(new_position);
                    break;
                } else {
                    break;
                }
            } else if (piece_color == attacker) {
                valid_moves.push(new_position);
                break;
            } else {
                valid_moves.push(new_position);
            }
            current_position = new_position;
        }
    }
    if (board_state.check == true) {
        let check_moves = Vec::new();
        let path = calculate_path(board_state);
        for i in 0..valid_moves.len() {
            if (path.includes(valid_moves[i])) {
                check_moves.push(valid_moves[i]);
            }
        }
        return check_moves;
    }
    valid_moves
}
pub fn king_move_generator(
    board_state: &BoardState,
    position: usize,
    defender: bool,
) -> Vec<usize> {
    let mut valid_moves = Vec::new();
    let piece = board_state.board[position];
    let color = piece_color(piece);
    let directions = [NW, N, NE, E, SE, S, SW, W];
    for i in 0..directions.len() {
        let new_position = position + directions[i];
        if (on_board(new_position as i16)) {
            let dest_piece = board_state.board[new_position];
            if (defender) {
                valid_moves.push(new_position);
                continue;
            } else if (piece_color(dest_piece) == color) {
                continue;
            }
            //THIS IS PROBABLY BROKEN
            if (board_state.check == true) {
                let mut temp_boardstate = board_state.copy_board_state();
                temp_boardstate.board[position] = 0;
                temp_boardstate.board[new_position] = piece;
                if (color == WHITE) {
                    temp_boardstate.white_king_pos = new_position;
                } else {
                    temp_boardstate.black_king_pos = new_position;
                }
                temp_boardstate.check = false;
                for i in 0..temp_boardstate.checking_squares.len() {
                    update_attack_table(&temp_boardstate, temp_boardstate.checking_squares[i][0]);
                }
                if (!is_square_under_attack(&temp_boardstate, new_position, attacker)) {
                    valid_moves.push(new_position);
                }
            } else {
                if (!is_square_under_attack(&board_state, new_position, attacker)) {
                    valid_moves.push(new_position);
                }
            }
        }
    }
    if (can_castle(board_state, position, "king", color)) {
        valid_moves.push(position + 2);
    }
    if (can_castle(board_state, position, "queen", color)) {
        valid_moves.push(position - 2);
    }
    valid_moves
}
