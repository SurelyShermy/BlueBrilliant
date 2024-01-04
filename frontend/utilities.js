const pieceMapping = {
    0: "Empty",
    9: " White king",
    10: "White pawn",
    11: "White knight",
    12: "White bishop",
    13: "White rook",
    14: "White queen",
    17: "Black king",
    18: "Black pawn",
    19: "Black knight",
    20: "Black bishop",
    21: "Black rook",
    22: "Black queen",

}

const pieceToLetter = {
    1: "K",
    2: "",
    3: "N",
    4: "B",
    5: "R",
    6: "Q",
}
const letterToPiece = {
    "K": 1,
    "N": 3,
    "B": 4,
    "R": 5,
    "Q": 6,
}

const fenMapping = {
    "r": pieceBitRep.black | pieceBitRep.rook,
    "n": pieceBitRep.black | pieceBitRep.knight,
    "b": pieceBitRep.black | pieceBitRep.bishop,
    "q": pieceBitRep.black | pieceBitRep.queen,
    "k": pieceBitRep.black | pieceBitRep.king,
    "p": pieceBitRep.black | pieceBitRep.pawn,
    "R": pieceBitRep.white | pieceBitRep.rook,
    "N": pieceBitRep.white | pieceBitRep.knight,
    "B": pieceBitRep.white | pieceBitRep.bishop,
    "Q": pieceBitRep.white | pieceBitRep.queen,
    "K": pieceBitRep.white | pieceBitRep.king,
    "P": pieceBitRep.white | pieceBitRep.pawn,
}

function isWhite(piece) {
    return (piece & pieceBitRep.white) === pieceBitRep.white;
}

function isBlack(piece) {
    return (piece & pieceBitRep.black) === pieceBitRep.black;
}

function pieceColor(piece) {
    if (isWhite(piece) == true) return pieceBitRep.white;
    if (isBlack(piece) == true) return pieceBitRep.black;
    return null;  // This is for cases where the piece might be "none".
}
function isRookOrQueen(piece) {
    if (piece === (pieceBitRep.rook | pieceBitRep.black) || piece === (pieceBitRep.rook | pieceBitRep.white) || piece === (pieceBitRep.queen | pieceBitRep.black) || piece === (pieceBitRep.queen | pieceBitRep.white)) {
        return true;
    }
    return false;
}
function isBishopOrQueen(piece) {
    if (piece === (pieceBitRep.bishop | pieceBitRep.black) || piece === (pieceBitRep.bishop | pieceBitRep.white) || piece === (pieceBitRep.queen | pieceBitRep.black) || piece === (pieceBitRep.queen | pieceBitRep.white)) {
        return true;
    }
    return false;
}
function isSlider(piece) {
    if (isRookOrQueen(piece) || isBishopOrQueen(piece)) {
        return true;
    }
    return false;
}

function copyGameState(gameState) {
    return JSON.parse(JSON.stringify(gameState));
}
function updateDom() {
    for (let i = 0; i < 128; i++) {
        if (onBoard(i) == false) {
            continue;
        }
        const cell = document.getElementById(`cell-${i}`);
        const piece = gameState.board[i];

        // Clear the current cell
        while (cell.firstChild) {
            cell.removeChild(cell.firstChild);
        }

        // If there is a piece in the current game state cell, create an element for it
        if (piece !== 0) {
            const img = document.createElement('img');
            img.src = svgPaths[piece];
            cell.appendChild(img)
        }
    }
}
function pieceType(piece) {
    return piece & 0b111;
}

//This does the modal:
function showWinModal() {
    var win = document.getElementById("win-modal-container");
    win.style.display = 'block';
    win.innerHTML = '<div class="modal-content" id="modal-content"><h2>CHECKMATE!</h2><p>You win! Congratulations!</p><button onclick="closeWindow()">Back to Dashboard</button></div>'
    var c = document.getElementById("modal-content");
    c.style.display = 'block';
}
function showLossModal() {
    var lose = document.getElementById("lose-modal-container");
    lose.style.display = 'block';
    lose.innerHTML = '<div class="modal-content" id="modal-content"><h2>CHECKMATED!</h2><p>You lose! Better luck next time!</p><button onclick="closeWindow()">Back to Dashboard</button></div>'
    var c = document.getElementById("modal-content");
    c.style.display = 'block';
}
function showDrawModal() {
    var draw = document.getElementById("draw-modal-container");
    draw.style.display = 'block';
    draw.innerHTML = '<div class="modal-content" id="modal-content"><h2>STALEMATE!</h2><p>Draw game! No winners here!</p><button onclick="closeWindow()">Back to Dashboard</button></div>'
    var c = document.getElementById("modal-content");
    c.style.display = 'block';
}

function closeWindow() {
    window.history.back();
}