use crate::game::initialization::*;
use crate::game::utilities::*;

pub fn find_pinned_pieces_north(board_state: &mut BoardState, position: usize) -> Option<usize> {
    let mut own_piece_pos: Option<usize> = None;
    let own_color = piece_color(board_state.board[position]);
    let direction_n = Direction::North.value();
    let mut pos = position as isize + direction_n;
    while on_board(pos) {
        let piece = board_state.board[pos as usize];
        if piece == 0 {
            pos += N;
            continue;
        }
        let color = piece_color(piece);

        match color {
            color if color == own_color => {
                if own_piece_pos.is_some() {
                    break;
                }
                own_piece_pos = Some(pos as usize);
            }
            _ => {
                if is_rook_or_queen(piece) && own_piece_pos.is_some() {
                    board_state.pinned_pieces.push((own_piece_pos.unwrap(), N));
                    board_state.pinners.push(pos as usize);
                    return own_piece_pos;
                }
                break;
            }
        }
        pos += direction_n;
    }

    None
}
pub fn find_pinned_pieces_south(board_state: &mut BoardState, position: usize) -> Option<usize> {
    let mut own_piece_pos: Option<usize> = None;
    let own_color = piece_color(board_state.board[position]);

    let mut pos = position as isize + S;
    let direction_s = Direction::South.value();
    let mut pos = position as isize + direction_s;
    while on_board(pos) {
        let piece = board_state.board[pos as usize];
        if piece == 0 {
            pos += S;
            continue;
        }
        let color = piece_color(piece);

        match color {
            color if color == own_color => {
                if own_piece_pos.is_some() {
                    break; // Another own piece found, no pin possible
                }
                own_piece_pos = Some(pos as usize);
            }
            _ => {
                if is_rook_or_queen(piece) && own_piece_pos.is_some() {
                    board_state.pinned_pieces.push((own_piece_pos.unwrap(), S));
                    board_state.pinners.push(pos as usize);
                    return own_piece_pos;
                }
                break;
            }
        }
        pos += S;
    }

    None
}
pub fn find_pinned_pieces_east(board_state: &mut BoardState, position: usize) -> Option<usize> {
    let mut own_piece_pos: Option<usize> = None;
    let own_color = piece_color(board_state.board[position]);

    let direction_e = Direction::East.value();
    let mut pos = position as isize + direction_e;
    while on_board(pos) {
        let piece = board_state.board[pos as usize];
        if piece == 0 {
            pos += E;
            continue; // Empty square
        }
        let color = piece_color(piece);

        match color {
            color if color == own_color => {
                if own_piece_pos.is_some() {
                    break; // Another own piece found, no pin possible
                }
                own_piece_pos = Some(pos as usize);
            }
            _ => {
                if is_rook_or_queen(piece) && own_piece_pos.is_some() {
                    // Pinning logic
                    board_state.pinned_pieces.push((own_piece_pos.unwrap(), E));
                    board_state.pinners.push(pos as usize);
                    return own_piece_pos;
                }
                break; // Stop after finding the first opponent's piece
            }
        }
        pos += direction_e;
    }

    None
}
pub fn find_pinned_pieces_west(board_state: &mut BoardState, position: usize) -> Option<usize> {
    let mut own_piece_pos: Option<usize> = None;
    let own_color = piece_color(board_state.board[position]);

    let direction_w = Direction::West.value();
    let mut pos = position as isize + direction_w;
    while on_board(pos) {
        let piece = board_state.board[pos as usize];
        if piece == 0 {
            pos += W;
            continue;
        }
        let color = piece_color(piece);

        match color {
            color if color == own_color => {
                if own_piece_pos.is_some() {
                    break;
                }
                own_piece_pos = Some(pos as usize);
            }
            _ => {
                if is_rook_or_queen(piece) && own_piece_pos.is_some() {
                    board_state
                        .pinned_pieces
                        .push((own_piece_pos.unwrap(), direction_w));
                    board_state.pinners.push(pos as usize);
                    return own_piece_pos;
                }
                break;
            }
        }
        pos += direction_w;
    }

    None
}
pub fn find_pinned_pieces_ne(board_state: &mut BoardState, position: usize) -> Option<usize> {
    let mut own_piece_pos: Option<usize> = None;
    let own_color = piece_color(board_state.board[position]);
    let direction_ne = Direction::NorthEast.value();
    let mut pos = position as isize + direction_ne;
    while on_board(pos) {
        let piece = board_state.board[pos as usize];
        if piece == 0 {
            pos += direction_ne;
            continue;
        }
        let color = piece_color(piece);

        match color {
            color if color == own_color => {
                if own_piece_pos.is_some() {
                    break;
                }
                own_piece_pos = Some(pos as usize);
            }
            _ => {
                if is_bishop_or_queen(piece) && own_piece_pos.is_some() {
                    board_state
                        .pinned_pieces
                        .push((own_piece_pos.unwrap(), direction_ne));
                    board_state.pinners.push(pos as usize);
                    return own_piece_pos;
                }
                break;
            }
        }
        pos += direction_ne;
    }

    None
}
pub fn find_pinned_pieces_nw(board_state: &mut BoardState, position: usize) -> Option<usize> {
    let mut own_piece_pos: Option<usize> = None;
    let own_color = piece_color(board_state.board[position]);
    let direction_nw = Direction::NorthWest.value();
    let mut pos = position as isize + direction_nw;
    while on_board(pos) {
        let piece = board_state.board[pos as usize];
        if piece == 0 {
            pos += direction_nw;
            continue; // Empty square
        }
        let color = piece_color(piece);

        match color {
            color if color == own_color => {
                if own_piece_pos.is_some() {
                    break;
                }
                own_piece_pos = Some(pos as usize);
            }
            _ => {
                if is_bishop_or_queen(piece) && own_piece_pos.is_some() {
                    board_state
                        .pinned_pieces
                        .push((own_piece_pos.unwrap(), direction_nw));
                    board_state.pinners.push(pos as usize);
                    return own_piece_pos;
                }
                break;
            }
        }
        pos += direction_nw;
    }

    None
}
pub fn find_pinned_pieces_se(board_state: &mut BoardState, position: usize) -> Option<usize> {
    let mut own_piece_pos: Option<usize> = None;
    let own_color = piece_color(board_state.board[position]);

    let mut pos = position as isize + SE;
    while on_board(pos) {
        let piece = board_state.board[pos as usize];
        if piece == 0 {
            pos += SE;
            continue; // Empty square
        }
        let color = piece_color(piece);

        match color {
            color if color == own_color => {
                if own_piece_pos.is_some() {
                    break;
                }
                own_piece_pos = Some(pos as usize);
            }
            _ => {
                if is_bishop_or_queen(piece) && own_piece_pos.is_some() {
                    board_state.pinned_pieces.push((own_piece_pos.unwrap(), SE));
                    board_state.pinners.push(pos as usize);
                    return own_piece_pos;
                }
                break;
            }
        }
        pos += SE;
    }

    None
}
pub fn find_pinned_pieces_sw(board_state: &mut BoardState, position: usize) -> Option<usize> {
    let mut own_piece_pos: Option<usize> = None;
    let own_color = piece_color(board_state.board[position]);

    let mut pos = position as isize + SW;
    while on_board(pos) {
        let piece = board_state.board[pos as usize];
        if piece == 0 {
            pos += SW;
            continue; // Empty square
        }
        let color = piece_color(piece);

        match color {
            color if color == own_color => {
                if own_piece_pos.is_some() {
                    break;
                }
                own_piece_pos = Some(pos as usize);
            }
            _ => {
                if is_bishop_or_queen(piece) && own_piece_pos.is_some() {
                    board_state.pinned_pieces.push((own_piece_pos.unwrap(), SW));
                    board_state.pinners.push(pos as usize);
                    return own_piece_pos;
                }
                break;
            }
        }
        pos += SW;
    }

    None
}
pub fn clear_pinned_pieces(board_state: &mut BoardState) {
    board_state.pinned_pieces.clear();
    board_state.pinners.clear();
}
pub fn pin_search(board_state: &mut BoardState, position: usize, king: bool) {
    if king {
        find_pinned_pieces_north(board_state, position);
        find_pinned_pieces_south(board_state, position);
        find_pinned_pieces_east(board_state, position);
        find_pinned_pieces_west(board_state, position);
        find_pinned_pieces_ne(board_state, position);
        find_pinned_pieces_nw(board_state, position);
        find_pinned_pieces_se(board_state, position);
        find_pinned_pieces_sw(board_state, position);
    }
}
pub fn update_attack_table(board_state: &mut BoardState, position: usize) {
    let piece = board_state.board[position];
    if piece == 0 {
        return;
    }
    let color = piece_color(piece);

    let attacked_squares = calculate_valid_moves(position, board_state, true);
    match color {
        WHITE => {
            board_state.white_attack_table[position].clear();
            for &move_pos in &attacked_squares {
                board_state.white_attack_table[position].push(move_pos);
            }
        }
        BLACK => {
            board_state.black_attack_table[position].clear();
            for &move_pos in &attacked_squares {
                board_state.black_attack_table[position].push(move_pos);
            }
        }
        _ => {}
    }
}
pub fn run_attack_table(board_state: &mut BoardState) {
    for i in 0..128 {
        if on_board(i as isize) {
            update_attack_table(board_state, i);
        }
    }
}
pub fn is_square_under_attack(
    board_state: &mut BoardState,
    square: usize,
    attacker_color: u8,
) -> bool {
    let attack_table = if attacker_color == WHITE {
        &board_state.white_attack_table
    } else {
        &board_state.black_attack_table
    };

    let king_position = if attacker_color == WHITE {
        board_state.black_king_pos
    } else {
        board_state.white_king_pos
    };

    for i in 0..128 {
        if attack_table[i].contains(&square) {
            if square == king_position as usize {
                let dir = vector_calc(i, square);
                board_state.checking_squares.push((i, dir));
            }
            return true;
        }
    }

    false
}
pub fn vector_calc(from_index: usize, to_index: usize) -> Option<Direction> {
    let diff = to_index as isize - from_index as isize;

    let directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
        Direction::NorthEast,
        Direction::NorthWest,
        Direction::SouthEast,
        Direction::SouthWest,
    ];

    for &direction in &directions {
        let dir_value = direction.value();
        if diff * dir_value > 0 && diff % dir_value == 0 {
            return Some(direction);
        }
    }

    None
}
pub fn is_checkmate(board_state: &mut BoardState) -> bool {
    let color_to_check = if board_state.turn == 0 { BLACK } else { WHITE };

    for i in 0..128 {
        if !on_board(i as isize) {
            continue;
        }
        if piece_color(board_state.board[i]) == color_to_check {
            let moves = calculate_valid_moves(board_state, board_state.board[i], i);
            if !moves.is_empty() {
                return false;
            }
        }
    }

    board_state.checkmate = true;
    true
}
pub fn is_stalemate(board_state: &mut BoardState) -> bool {
    let color_to_check = if board_state.turn == 0 { BLACK } else { WHITE };

    for i in 0..128 {
        if !on_board(i as isize) {
            continue;
        }
        if piece_color(board_state.board[i]) == color_to_check {
            let moves = calculate_valid_moves(board_state, board_state.board[i], i);
            if !moves.is_empty() {
                return false;
            }
        }
    }

    board_state.stalemate = true;
    true
}
