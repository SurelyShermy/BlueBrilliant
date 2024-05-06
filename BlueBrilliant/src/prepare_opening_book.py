import chess

def square_to_index(square):
    return chess.square(ord(square[0]) - ord('a'), int(square[1]) - 1)

def parse_and_convert_games(filename):
    with open(filename, 'r') as file:
        all_games_indices = []
        
        # Define the set of game results to exclude
        game_results = {"1-0", "1/2-1/2", "0-1"}
        
        for line in file:
            if line.strip():
                board = chess.Board()
                # Exclude game results by checking if the move is not in the set
                moves_san = [move for move in line.strip().split() if move not in game_results]
                moves_indices = []
                
                for move_san in moves_san:
                    try:
                        move = board.push_san(move_san)
                        from_index = square_to_index(chess.square_name(move.from_square))
                        to_index = square_to_index(chess.square_name(move.to_square))
                        moves_indices.append((from_index, to_index))
                    except ValueError:
                        # In case of any invalid SAN moves, continue to next move
                        continue
                
                all_games_indices.append(moves_indices)
    
    return all_games_indices

def write_indices_to_file(games_indices, output_filename):
    with open(output_filename, 'w') as file:
        for game_indices in games_indices:
            game_string = ' '.join(f"({from_idx},{to_idx})" for from_idx, to_idx in game_indices)
            file.write(game_string + '\n')

# Example usage
games_indices = parse_and_convert_games("games.txt")
write_indices_to_file(games_indices, "converted_games.txt")