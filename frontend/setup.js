
//This Piece implementation is adapted from the following source: https://youtu.be/U4ogK0MIzqk?t=82

//It takes advantage of bit representations of pieces on the board to make the game more efficient. 

/*
Lets take a look at it like this:

The designation for a king is 1
0000001 = king

and a designation for white is 16
0010000 = white
We can then do a bitwise or operation | to get the following:
0010001 = white king!
0011111
//Liam was here

so we should be able to declare pieces as follows:
square  = piece.white | piece.king

*/
class pieceBitRep {
    static none = 0;
    static king = 1;
    static pawn = 2;
    static knight = 3;
    static bishop = 4;
    static rook = 5;
    static queen = 6;

    static white = 8;
    static black = 16;
}
// 00000001
const svgPaths = {
    [pieceBitRep.white | pieceBitRep.king]: "chessPieces/white/K.svg",
    [pieceBitRep.white | pieceBitRep.pawn]: "chessPieces/white/P.svg",
    [pieceBitRep.white | pieceBitRep.knight]: "chessPieces/white/N.svg",
    [pieceBitRep.white | pieceBitRep.bishop]: "chessPieces/white/B.svg",
    [pieceBitRep.white | pieceBitRep.rook]: "chessPieces/white/R.svg",
    [pieceBitRep.white | pieceBitRep.queen]: "chessPieces/white/Q.svg",
    [pieceBitRep.black | pieceBitRep.king]: "chessPieces/black/k.svg",
    [pieceBitRep.black | pieceBitRep.pawn]: "chessPieces/black/p.svg",
    [pieceBitRep.black | pieceBitRep.knight]: "chessPieces/black/n.svg",
    [pieceBitRep.black | pieceBitRep.bishop]: "chessPieces/black/b.svg",
    [pieceBitRep.black | pieceBitRep.rook]: "chessPieces/black/r.svg",
    [pieceBitRep.black | pieceBitRep.queen]: "chessPieces/black/q.svg"
};
var move = {
    fromIndex: 0,
    toIndex: 0,
}
const directions = {
    NW: 15,
    NE: 17,
    N: 16,
    S: -16,
    SW: -17,
    SE: -15,
    E: 1,
    W: -1,
    knightmoves: [33, 31, 18, 14, -33, -31, -18, -14],
};
function onBoard(index) {
    if (index < 0) {
        return false;
    }
    let sum = (index & 0x88);
    if (sum == 0) {
        return true;
    }
    return false;
}
//This converts the 0x88 index to the 0-63 index
function toDecIndex(index) {
    let newIndex = (index + (index & 7)) >> 1;
    return newIndex;
}
//This converts the  0-63 index to the 0x88 index
function toHexIndex(index) {
    let newIndex = (index + (index & ~7))
    return newIndex;
}
var boardState = new Array(128).fill(0);
var gameState = {
    turn: 0,
    board: boardState,
};


/*
OK so maybe this isnt the best way to do this but I figured hardcoding the initial position is probably for the best.

TODO: Need to see if its possible to put 0 at the bottom left rather than the top left, sides need to be swapped if this is done
TODO: Add flip board button
*/

//We will proceed by intializing the board to the starting position
boardState[0] = pieceBitRep.white | pieceBitRep.rook;
boardState[1] = pieceBitRep.white | pieceBitRep.knight;
boardState[2] = pieceBitRep.white | pieceBitRep.bishop;
boardState[3] = pieceBitRep.white | pieceBitRep.queen;
boardState[4] = pieceBitRep.white | pieceBitRep.king;
boardState[5] = pieceBitRep.white | pieceBitRep.bishop;
boardState[6] = pieceBitRep.white | pieceBitRep.knight;
boardState[7] = pieceBitRep.white | pieceBitRep.rook;
for (let i = 0x10; i < 0x18; i++) {
    boardState[i] = pieceBitRep.white | pieceBitRep.pawn;
}
for (let i = 0x60; i < 0x68; i++) {
    boardState[i] = pieceBitRep.black | pieceBitRep.pawn;
}
boardState[0x70] = pieceBitRep.black | pieceBitRep.rook;
boardState[0x71] = pieceBitRep.black | pieceBitRep.knight;
boardState[0x72] = pieceBitRep.black | pieceBitRep.bishop;
boardState[0x73] = pieceBitRep.black | pieceBitRep.queen;
boardState[0x74] = pieceBitRep.black | pieceBitRep.king;
boardState[0x75] = pieceBitRep.black | pieceBitRep.bishop;
boardState[0x76] = pieceBitRep.black | pieceBitRep.knight;
boardState[0x77] = pieceBitRep.black | pieceBitRep.rook;

const chessBoard = document.getElementById("chessBoard");
function getFile(index) {
    let ret = toDecIndex(index) & 7;
    return ret
}
function getRank(index) {
    let ret = toDecIndex(index) >> 3;
    return ret
}
const notationMapping = {
    a: 0,
    b: 1,
    c: 2,
    d: 3,
    e: 4,
    f: 5,
    g: 6,
    h: 7,
    1: 0,
    2: 1,
    3: 2,
    4: 3,
    5: 4,
    6: 5,
    7: 6,
    8: 7
}
const files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const ranks = ['1', '2', '3', '4', '5', '6', '7', '8'];
//Populating the Board

for (let i = 8; i > 0; i--) {
    let start = toHexIndex((i - 1) * 8);
    let end = toHexIndex((i * 8) - 1);
    while (start <= end) {
        const cell = document.createElement('div');
        cell.className = 'cell';
        cell.id = `cell-${start}`;
        if (Math.floor(start / 16) % 2 === 0) {
            cell.className += (start % 2 === 0) ? ' dark' : ' light';
        } else {
            cell.className += (start % 2 === 0) ? ' light' : ' dark';
        }
        let squareNotation = "";
        squareNotation = squareNotation.concat(files[getFile(start)]);
        squareNotation = squareNotation.concat(ranks[(getRank(start))]);
        cell.setAttribute('data-notation', squareNotation);
        piece = boardState[start];
        if (piece !== 0) {
            const img = document.createElement('img');
            img.src = svgPaths[piece];
            cell.appendChild(img);
        }
        chessBoard.appendChild(cell);
        start++;
    }
}
gameState.board = boardState;
console.log(gameState.board)