# Steps to take before an engine is even designed

1. A chess engine relies on a bug free implementation of Chess. Most commonly, bugs will arise from castling, pawn promotion, pins, and checks. Keeping these tricky scenarios in mind will aid us greatly. Additionally, 3 fold repetition draws will be a necessary rule to implement to avoid the engine simply moving back and forth forever.
2. It is recommended that a script is run to cycle through all of the possible positions within at least 5 moves from the starting position. If the number of positions reached matches the number found by other engines (stockfish in particular), then it confirms we have a legal game.
   - After our game passes 5 moves from the starting position, we must then run it from a different position. There are [pre-calculated positions](https://www.chessprogramming.org/Perft_Results) available on the chess programming website. If we pass this, we can confidently claim that our board is accurate.
3. Optimizations are going to be crucial when it comes to these position evaluations. A typical optimization will be to record piece locations as opposed to constantly looping over the board. Engine optimizations will be discussed in greater detail in the next section.

# Chess Engine Basics

Now that we have escaped bug hell and have a working chess board that follows the rules we can begin to take the first steps towards designing an engine. There are 3 types of chess engines: algorithmic, [neural network](https://www.chessprogramming.org/Neural_Networks) and hybrid. For now, (unless I really go down the rabbit hole), we should plan on implementing an _algorithmic_ chess engine. Stockfish is the most famous algorithmic chess engine and it is one of the most successful and widely used engines ever created. At its most basic, algorithmic chess engines have 2 foundational principles: **Search** and **Evaluation**.

## Search

If youre anything like me, the first idea that comes to mind when hearing about search with a chess engine is depth first search from algorithms class. In the simplest terms, a chess engine's search function is going to be an enhanced and optimized version of DFS. A position and its derivatives can be displayed as a _tree_ data structure. This tree is called a [search tree](https://www.chessprogramming.org/Search_Tree). Naturally, the root of the tree is the position to be evaluated and the leaves are possible moves from the position. However, this tree will soon grow to have billions of leaves if pruning is not done to eliminate bogus lines. This is where the [Alpha-Beta](https://www.chessprogramming.org/Alpha-Beta) pruning algorithm will be crucial.

## Alpha-Beta

Alpha beta is a depth first search algorithm that aids in pruning the tree of possible moves. The principle behind alpha beta is actually surprisingly simple. At a depth of 2, (or a ply of 1), we are only considering white's move and black's response. First, we will choose a possible first move for white and proceed to list every possible response from black. For this example imagine that black's responses result in an even position at _worst_. We can then proceed to look at white's 2nd possible move. Now for this scenario, imagine a line is found in which black wins a piece. This line can now be completely ignored because we know that white's first evaluated move was _even_ in the worst case. This effectively prunes an entire line off of the tree increasing efficiency!

Please read page 89 of the cornell paper 2209 listed as bibliography entry 6 for more information on alphabeta

This is a crucially important optimization but still not enough alone. The next optimization is called a [Transposition table](https://www.chessprogramming.org/Transposition_Table).

## Transposition Tables

In chess, the same position can be reached in _many_ different ways. When our search engine encounters a tranposition, having the ability to store evaluations from pre-analyzed positions can be extremely powerful. Storing these positions requires the use of a hash function to convert positions into 64 bit strings. Now if you're a particularly attentive computer scientist, you will notice that a 64 bit number is simply not large enough for the 1 x 10<sup>46</sup> possible chess positions. However it is important to note that many of these positions are bogus and not likely to occur. Additionally, collision probability can be extremely low depending on how many positions are stored and how many bits are stored.

## Quiescence Searches

After the main search is performed, it will be beneficial to also add a [quiescence search](https://www.chessprogramming.org/Quiescence_Search) that contains fewer moves. _Quiet Moves_ are moves in which no captures, promotions, or checks occur. They are executed in order to prevent the horizon effect. Simply put, the horizon effect occurs when a search stops at an insufficient depth. As the wiki states, imagine a queen captures a knight and evaluation stops at this move. Without further evaluation, this sounds like a great option. However, if the next move is pawn takes queen, we have simply lost 6 points of material!

## Evaluation

So now we can search thousands of positions stemming from an initial position, but how do we know which moves are better? The first step is a piece square table. Some squares on the board are superior to others. A queen on d4 controls a whopping 28 of 64 squares whereas a queen on a1 controls only 22 squares. Therefore there should certainly be a motivation for most pieces to become centralized. The piece square table weighs each square from -50 to +20.
In addition to the piece square table, each piece should have a unique weight as well. Luckily this is something that has been pretty well optimized. Pawns are worth 100, Knights 310, Bishops 320, Rooks 500, and Queens 900. Now that we have piece values and square values, a board state can be evaluated. Note that this state cannot detect incoming checkmate or tactics. Therefore searching with this evaluation algorithm is necessary.
Evaluation functions can be improved by attempting to detect whether the game is in the opening, middle game or endgame. During these different gamestates, king position, castling, and pawn positions should have different weights.

With this in mind we can actually define a pretty simple evaluation function in pseudocode:

```
func evaluate(board)
   scoreWhite = 0
   scoreBlack = 0
   for i in range(8):
      for j in range(8):
      if piece == "P":
         scoreWhite += (pawn_value + table[i][j])
      if piece == "p"
         scoreBlack += (pawn_value + table[i][j])
      .
      .
      .
   return scoreWhite-scoreBlack
```

And just like that we can already evaluate a single board state! Optimizing this code (if possible) would be significantly helpful since this evaluation must be run every single time a search finds a new position.

Another interesting algorithm to look at is montecarlo search but not sure if this is the direction we want to go. Maybe something for the future?

## Acknowledgements

1. Alex Sherman
2.
3.
4.
5.

# Bibliography

1. https://www.chess.com/blog/the_real_greco/understanding-alphazero-a-basic-chess-neural-network
2. https://www.chessprogramming.org/Perft_Results
3. https://www.chessprogramming.org
4. https://www.gamedev.net/tutorials/_/technical/artificial-intelligence/chess-programming-part-ii-data-structures-r1046/ (found off of chessprogramming.org)
5. https://web.archive.org/web/20071030220820/http://www.brucemo.com/compchess/programming/minmax.htm
6. https://arxiv.org/ftp/arxiv/papers/2209/2209.01506.pdf
7. https://www.chessprogramming.org/Search
