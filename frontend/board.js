//Click Listner
//Test
document.querySelectorAll('.cell').forEach(cell => {
    cell.addEventListener('click', handleCellClick);
});
function promptForPromotion(position, piece) {
    const pieceNames = ['queen', 'rook', 'bishop', 'knight'];
    const chosenPiece = window.prompt(`Choose a piece to promote to: ${pieceNames.join(', ')}`);
    const color = piece ^ pieceBitRep.pawn;
    switch (chosenPiece) {
        case 'queen':
            gameState.board[position] = color | pieceBitRep.queen;
            updateAttackTable(gameState, position);
            break;
        case 'rook':
            gameState.board[position] = color | pieceBitRep.rook;
            updateAttackTable(gameState, position)
            break;
        case 'bishop':
            gameState.board[position] = color | pieceBitRep.bishop;
            updateAttackTable(gameState, position)
            break;
        case 'knight':
            gameState.board[position] = color | pieceBitRep.knight;
            updateAttackTable(gameState, position)
            break;
        default:
            alert('Invalid choice. Please select again.');
            promptForPromotion(position, color); // Recursive call to force valid choice
            break;
    }

    // Update the DOM (this is an example, your implementation might vary)
    const cell = document.getElementById(`cell-${position}`);
    const pawnImage = cell.querySelector('img');
    if (pawnImage) {
        cell.removeChild(pawnImage);
    } else {
        // console.log('No image found in the cell.');
    }
    const newPieceImg = document.createElement('img');
    newPieceImg.src = svgPaths[gameState.board[position]];
    cell.appendChild(newPieceImg);
}
function getURLParameter(name) {
    return new URLSearchParams(window.location.search).get(name);
}
function movePieceToNotation(pieceType, fromIndex, toIndex) {
    const files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    const ranks = ['1', '2', '3', '4', '5', '6', '7', '8'];

    // Convert indices to file and rank
    if (pieceType === pieceBitRep.king) {
        if (Math.abs(fromIndex - toIndex) == 2 && fromIndex < toIndex) {
            return "O-O";
        } else if (Math.abs(fromIndex - toIndex) == 2 && fromIndex > toIndex) {
            return "O-O-O";
        }
    }
    const toFile = files[getFile(toIndex)];
    const toRank = ranks[getRank(toIndex)];

    // Piece notation (empty for pawn)
    const pieceNotation = pieceToLetter[pieceType];

    // Capture notation
    let capture = false;
    if (gameState.board[toIndex] != 0) {
        capture = true;
    } // Assuming you have a way to determine if it's a capture
    const captureNotation = capture ? 'x' : '';

    // Combine to create move notation
    const moveNotation = pieceNotation + captureNotation + toFile + toRank;

    return moveNotation;
}
function initializeGameMode() {
    const mode = getURLParameter('mode');
    switch (mode) {
        case 'engine':
            gameState.enginePlayingBlack = true;
            gameState.enginePlayingWhite = false;
            gameState.engine = true;
            gameState.randomMoves = false;
            // set boolean for engine mode
            break;
        case 'overboard':
            gameState.enginePlayingBlack = false;
            gameState.enginePlayingWhite = false;
            gameState.engine = false;
            // set boolean for board mode
            break;
        case 'random':
            gameState.enginePlayingBlack = true;
            gameState.enginePlayingWhite = false;
            gameState.engine = true;
            gameState.randomMoves = true;
            // set boolean for random moves mode
            break;
        default:
        // default mode or error handling
    }
}

// Call this function when the page loads
initializeGameMode();

let whiteTime = 600; // e.g., 10 minutes in seconds
let blackTime = 600;
let timeIncrement = 0; // e.g., Default 0 second increments
let intervalId;

function updateDisplay() {
    document.getElementById('white-timer').innerText = `White: ${formatTime(whiteTime)}`;
    document.getElementById('black-timer').innerText = `Black: ${formatTime(blackTime)}`;
}

function formatTime(seconds) {
    const min = Math.floor(seconds / 60);
    const sec = seconds % 60;
    return `${min}:${sec < 10 ? '0' : ''}${sec}`;
}

function startTimer() {
    // Clear existing interval
    clearInterval(intervalId);
    intervalId = setInterval(function () {
        if (gameState.turn == 0) {
            whiteTime--;
        } else {
            blackTime--;
        }
        updateDisplay();
        // Check if time has run out
        if (whiteTime === 0 || blackTime === 0) {
            clearInterval(intervalId); // Stop the timer
            endGame(); // Call a function to handle the end of the game
        }
    }, 1000);
}

async function sendMove(from, to) {
    const response = await fetch('http://127.0.0.1:8080/move', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ from, to }),
    });
    const moveResponse = await response.json();
    // Process the move response here
}

function updateCapturedPiecesDisplay(piece) {
    let pieceElement = document.createElement('img');
    pieceElement.className = `captured-piece ${color}`;
    pieceElement.src = svgPaths[piece]; // or use an image representing the piece
    if (isWhite(piece) === true) {
        document.getElementById('white-captured-pieces').appendChild(pieceElement);
    } else {
        document.getElementById('black-captured-pieces').appendChild(pieceElement);
    }
}


function promoteToQueen(position, piece, domUpdate = true) {
    const color = pieceColor(piece);
    const promotedQueen = color | pieceBitRep.queen;
    gameState.board[position] = promotedQueen;
    if (domUpdate === true) {
        const cell = document.getElementById(`cell-${position}`);
        const pawnImage = cell.querySelector('img');
        if (pawnImage) {
            cell.removeChild(pawnImage);
        } else {
            // console.log('No image found in the cell.');
        }

        const newPieceImg = document.createElement('img');
        newPieceImg.src = svgPaths[gameState.board[position]];
        cell.appendChild(newPieceImg);
    }
    return;
}
//Checks if it's a pawn that has reached the back rank
function handlePawnPromotion(toIndex, domUpdate = true) {
    const piece = gameState.board[toIndex];
    const color = pieceColor(piece);
    const type = pieceType(piece);
    if (type == pieceBitRep.pawn) {
        // Check if it's a white pawn reaching rank 8 or a black pawn reaching rank 1
        if ((color === pieceBitRep.white) && toIndex >= toHexIndex(56) && toIndex <= toHexIndex(63)) {
            // console.log("White Pawn Promotion")
            promptForPromotion(toIndex, piece)
        }
        else if ((color === pieceBitRep.black) && toIndex >= 0 && toIndex <= 7) {
            // console.log("Black Pawn Promotion")
            promptForPromotion(toIndex, piece)
        }
    }
    return
}
function pawnToQueen(toIndex, domUpdate = true) {
    const piece = gameState.board[toIndex];
    const color = pieceColor(piece);
    const type = pieceType(piece);
    if (type == pieceBitRep.pawn) {
        // Check if it's a white pawn reaching rank 8 or a black pawn reaching rank 1
        if ((color === pieceBitRep.white) && toIndex >= toHexIndex(56) && toIndex <= toHexIndex(63)) {
            // console.log("White Pawn Promotion")
            promoteToQueen(toIndex, piece, domUpdate)
        }
        else if ((color === pieceBitRep.black) && toIndex >= 0 && toIndex <= 7) {
            // console.log("Black Pawn Promotion")
            promoteToQueen(toIndex, piece, domUpdate)
        }
    }
    return
}
var gameStateStack = [];
gameStateStack.push(copyGameState(gameState));
var selectedCell = null; // Variable to hold the currently highlighted cell index
var validMoves = []; // Variable to hold the valid moves for the currently selected piece



function clearHighlights() {
    document.querySelectorAll('.highlight').forEach(cell => {
        cell.classList.remove('highlight');

    });
    document.querySelectorAll('.selected').forEach(cell => {
        cell.classList.remove('selected');
    });
    // document.querySelectorAll('.origin-highlight').forEach(cell => {
    //     cell.classList.remove('origin-highlight');
    // });
    // document.querySelectorAll('.to-highlight').forEach(cell => {
    //     cell.classList.remove('to-highlight');
    // });
}
function clearOriginHighlights() {
    document.querySelectorAll('.selected').forEach(cell => {
        cell.classList.remove('selected');
    });
    document.querySelectorAll('.origin-highlight').forEach(cell => {
        cell.classList.remove('origin-highlight');
    });
    document.querySelectorAll('.to-highlight').forEach(cell => {
        cell.classList.remove('to-highlight');
    });
}
function clearFromHighlights() {
    document.querySelectorAll('.selected').forEach(cell => {
        cell.classList.remove('selected');
    });
}
function highlightValidMoves(index) {
    clearHighlights();
    fetch('http://127.0.0.1:8080/moves', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ index }),
    })
        .then(response => response.json())
        .then(data => {
            const validMoves = data.valid_moves;

            validMoves.forEach(moveIndex => {
                const cell = document.getElementById(`cell-${moveIndex}`);
                cell.classList.add('highlight');
            });

            const fromCell = document.getElementById(`cell-${index}`);
            fromCell.classList.add('selected');
            selectedCell = index;
        })
        .catch(error => console.error('Error:', error));
}
function sendConfirmedMove(fromIndex, toIndex) {
    fetch('http://127.0.0.1:8080/board', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ from: fromIndex, to: toIndex }),
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            return response.json();
        })
        .then(data => {
            console.log('Move confirmed:', data);
            // Here you can handle any additional tasks after the move is confirmed
        })
        .catch(error => console.error('Error:', error));
}
function handleCellClick(event) {
    const cell = event.currentTarget;
    const index = parseInt(cell.id.split('-')[1]);
    const piece = gameState.board[index];

    if ((gameState.turn === 0 && pieceColor(piece) === pieceBitRep.white) ||
        (gameState.turn === 1 && pieceColor(piece) === pieceBitRep.black)) {
        highlightValidMoves(index);
    } else if (selectedCell !== null && validMoves.includes(index)) {
        movePiece(selectedCell, index);
        //TODO: Implement send confirmed move
        sendConfirmedMove(selectedCell, index);
        clearHighlights();
        selectedCell = null;
    }
}

function updateCapturedPiecesDisplay(piece) {
    let pieceElement = document.createElement('img');
    pieceElement.className = `captured-piece ${color}`;
    pieceElement.src = svgPaths[piece]; // or use an image representing the piece
    if (isWhite(piece)) {
        document.getElementById('white-captured-pieces').appendChild(pieceElement);
    } else {
        document.getElementById('black-captured-pieces').appendChild(pieceElement);
    }
}



function movePiece(fromIndex, toIndex) {
    clearOriginHighlights();

    const piece = gameState.board[fromIndex];
    const fromType = piece & 0b111;
    if (fromType === pieceBitRep.king && (Math.abs(toIndex - fromIndex) == 2)) {
        if (toIndex === 2) {
            gameState.board[0] = 0;
            gameState.board[3] = pieceBitRep.white | pieceBitRep.rook;
            const rookFromCell = document.getElementById(`cell-0`);
            const rookToCell = document.getElementById(`cell-3`);
            while (rookToCell.firstChild) {
                rookToCell.removeChild(rookToCell.firstChild);
            }
            if (rookFromCell.firstChild) {
                rookToCell.appendChild(rookFromCell.firstChild);
            }
        } else if (toIndex === 6) {
            gameState.board[7] = 0;
            gameState.board[5] = pieceBitRep.white | pieceBitRep.rook;
            const rookFromCell = document.getElementById(`cell-7`);
            const rookToCell = document.getElementById(`cell-5`);
            while (rookToCell.firstChild) {
                rookToCell.removeChild(rookToCell.firstChild);
            }
            if (rookFromCell.firstChild) {
                rookToCell.appendChild(rookFromCell.firstChild);
            }
        } else if (toIndex === 114) {
            gameState.board[112] = 0;
            gameState.board[115] = pieceBitRep.black | pieceBitRep.rook;
            const rookFromCell = document.getElementById(`cell-112`);
            const rookToCell = document.getElementById(`cell-115`);
            while (rookToCell.firstChild) {
                rookToCell.removeChild(rookToCell.firstChild);
            }
            if (rookFromCell.firstChild) {
                rookToCell.appendChild(rookFromCell.firstChild);
            }
        } else if (toIndex === 118) {
            gameState.board[119] = 0;
            gameState.board[117] = pieceBitRep.black | pieceBitRep.rook;
            const rookFromCell = document.getElementById(`cell-119`);
            const rookToCell = document.getElementById(`cell-117`);
            while (rookToCell.firstChild) {
                rookToCell.removeChild(rookToCell.firstChild);
            }
            if (rookFromCell.firstChild) {
                rookToCell.appendChild(rookFromCell.firstChild);
            }
        }
    }
    let potentialCapturedPiece = gameState.board[toIndex];
    gameState.board[toIndex] = piece;
    gameState.board[fromIndex] = 0;


    const fromCell = document.getElementById(`cell-${fromIndex}`);
    const toCell = document.getElementById(`cell-${toIndex}`);

    if (!toCell) {
        console.error(`No element found with the ID: cell-${toIndex}`);
        return;
    }
    // If there's a piece on the target cell (toCell), remove it (capture).
    while (toCell.firstChild) {
        //Adds captured piece to capture box
        updateCapturedPiecesDisplay(potentialCapturedPiece);
        toCell.removeChild(toCell.firstChild);
    }
    //add highlight for origin cell
    toCell.classList.add('.to-highlight');
    fromCell.classList.add('.origin-highlight');
    // Move the piece from the source cell (fromCell) to the target cell (toCell).
    if (fromCell.firstChild) {
        toCell.appendChild(fromCell.firstChild);

    }
    var capturedPawnIndex = null;
    // Check for en passant
    if (toIndex === gameState.enpassant && (piece == (pieceBitRep.white | pieceBitRep.pawn) || piece == (pieceBitRep.black | pieceBitRep.pawn))) {

        if (piece === (pieceBitRep.white | pieceBitRep.pawn)) {
            capturedPawnIndex = toIndex - 16;
        }
        else if (piece === (pieceBitRep.black | pieceBitRep.pawn)) {
            capturedPawnIndex = toIndex + 16;
        }
        const epCell = document.getElementById(`cell-${capturedPawnIndex}`);
        while (epCell.firstChild) {
            //Adds captured piece to capture box
            updateCapturedPiecesDisplay(gameState.board[capturedPawnIndex]);
            epCell.removeChild(epCell.firstChild);
        }
        gameState.board[capturedPawnIndex] = 0;
    }
    handlePawnPromotion(toIndex);
    if (gameState.turn == 0) {

        if (gameState.check == true) {
            const kingCell = document.getElementById(`cell-${gameState.blackKingPos}`);
            kingCell.classList.add('blinking');
            // console.log("Black King in check")
            if (gameState.checkmate == true) {
                //playWinAnimation()
                winChessGame();
                showWinModal();
                // console.log("Checkmate");

            }
        } else {
            if (gameState.stalemate == true) {
                //playDrawAnimation();
                showDrawModal();
                drawChessGame();

                // console.log("Stalemate");

            }
        }
        //TODO: need to add engine logic for rust backend
        gameState.turn = 1;

    } else {
        if (gameState.check == true) {
            console.log("White King in check")
            const kingCell = document.getElementById(`cell-${gameState.whiteKingPos}`);
            kingCell.classList.add('blinking');
            if (gameState.checkmate == true) {

                console.log("Checkmate")
                //playLoseAnimation()
                showLossModal();
                // console.log("Checkmate")
                loseChessGame();

            }
        } else {
            if (gameState.stalemate == true) {
                //playDrawAnimation();

                // console.log("Stalemate");
                showDrawModal();
                drawChessGame();

            }

        }
        //TODO: need to add engine logic for rust backend
        gameState.turn = 0;

    }
}


//Same as move piece but has NO DOM UPDATE
// function testMovePiece(fromIndex, toIndex) {
//     clearOriginHighlights();
//     //Add a copy of the stat
//     gameStateStack.push(copyGameState(gameState));
//     // Update the board state array

//     gameState.check = false;
//     gameState.checkingSquares = [];
//     const piece = gameState.board[fromIndex];
//     const fromColor = pieceColor(piece);
//     const takenPiece = gameState.board[toIndex];
//     const fromType = piece & 0b111;
//     if (piece === (pieceBitRep.white | pieceBitRep.king)) {
//         gameState.castlingWhite["king"] = false;
//         gameState.castlingWhite["queen"] = false;
//     } else if (piece === (pieceBitRep.black | pieceBitRep.king)) {
//         gameState.castlingBlack["king"] = false;
//         gameState.castlingBlack["queen"] = false;
//     }
//     if (piece === (pieceBitRep.white | pieceBitRep.rook)) {
//         if (fromIndex === 0) {
//             gameState.castlingWhite["queen"] = false;
//         } else if (fromIndex === 7) {
//             gameState.castlingWhite["king"] = false;
//         }
//     }
//     if (piece === (pieceBitRep.black | pieceBitRep.rook)) {
//         if (fromIndex === 112) {
//             gameState.castlingBlack["queen"] = false;
//         } else if (fromIndex === 119) {
//             gameState.castlingBlack["king"] = false;
//         }
//     }
//     let rookPosition = null;
//     if (fromType === pieceBitRep.king && (Math.abs(toIndex - fromIndex) == 2)) {
//         if (toIndex === 2) {
//             gameState.board[0] = 0;
//             gameState.board[3] = pieceBitRep.white | pieceBitRep.rook;
//             // const rookFromCell = document.getElementById(`cell-0`);
//             // const rookToCell = document.getElementById(`cell-3`);
//             // while (rookToCell.firstChild) {
//             //     rookToCell.removeChild(rookToCell.firstChild);
//             // }
//             // if (rookFromCell.firstChild) {
//             //     rookToCell.appendChild(rookFromCell.firstChild);
//             // }
//             updateAttackTable(gameState, 3);
//             updateAttackTable(gameState, 0);
//             rookPosition = 3;

//         } else if (toIndex === 6) {
//             gameState.board[7] = 0;
//             gameState.board[5] = pieceBitRep.white | pieceBitRep.rook;
//             // const rookFromCell = document.getElementById(`cell-7`);
//             // const rookToCell = document.getElementById(`cell-5`);
//             // while (rookToCell.firstChild) {
//             //     rookToCell.removeChild(rookToCell.firstChild);
//             // }
//             // if (rookFromCell.firstChild) {
//             //     rookToCell.appendChild(rookFromCell.firstChild);
//             // }
//             updateAttackTable(gameState, 5);
//             updateAttackTable(gameState, 7);
//             rookPosition = 5;

//         } else if (toIndex === 114) {
//             gameState.board[112] = 0;
//             gameState.board[115] = pieceBitRep.black | pieceBitRep.rook;
//             // const rookFromCell = document.getElementById(`cell-112`);
//             // const rookToCell = document.getElementById(`cell-115`);
//             // while (rookToCell.firstChild) {
//             //     rookToCell.removeChild(rookToCell.firstChild);
//             // }
//             // if (rookFromCell.firstChild) {
//             //     rookToCell.appendChild(rookFromCell.firstChild);
//             // }
//             updateAttackTable(gameState, 115);
//             updateAttackTable(gameState, 116);
//             rookPosition = 115;


//         } else if (toIndex === 118) {
//             gameState.board[119] = 0;
//             gameState.board[117] = pieceBitRep.black | pieceBitRep.rook;
//             // const rookFromCell = document.getElementById(`cell-119`);
//             // const rookToCell = document.getElementById(`cell-117`);
//             // while (rookToCell.firstChild) {
//             //     rookToCell.removeChild(rookToCell.firstChild);
//             // }
//             // if (rookFromCell.firstChild) {
//             //     rookToCell.appendChild(rookFromCell.firstChild);
//             // }
//             updateAttackTable(gameState, 118);
//             updateAttackTable(gameState, 119);
//             rookPosition = 118;

//         }
//     }
//     let potentialCapturedPiece = gameState.board[toIndex];
//     gameState.board[toIndex] = piece;
//     gameState.board[fromIndex] = 0;


//     // const fromCell = document.getElementById(`cell-${fromIndex}`);
//     // const toCell = document.getElementById(`cell-${toIndex}`);

//     // if (!toCell) {
//     //     console.error(`No element found with the ID: cell-${toIndex}`);
//     //     return;
//     // }
//     // If there's a piece on the target cell (toCell), remove it (capture).
//     // while (toCell.firstChild) {
//     //     //Adds captured piece to capture box
//     //     updateCapturedPiecesDisplay(potentialCapturedPiece);
//     //     toCell.removeChild(toCell.firstChild);
//     // }
//     //add highlight for origin cell
//     // toCell.classList.add('.to-highlight');
//     // fromCell.classList.add('.origin-highlight');
//     // // Move the piece from the source cell (fromCell) to the target cell (toCell).
//     // if (fromCell.firstChild) {
//     //     toCell.appendChild(fromCell.firstChild);

//     // }

//     if (piece === (pieceBitRep.white | pieceBitRep.king)) {
//         gameState.whiteKingPos = toIndex;
//     } else if (piece === (pieceBitRep.black | pieceBitRep.king)) {
//         gameState.blackKingPos = toIndex;
//     }
//     var capturedPawnIndex = null;
//     // Check for en passant
//     if (toIndex === gameState.enpassant && (piece == (pieceBitRep.white | pieceBitRep.pawn) || piece == (pieceBitRep.black | pieceBitRep.pawn))) {

//         if (piece === (pieceBitRep.white | pieceBitRep.pawn)) {
//             capturedPawnIndex = toIndex - 16;
//         }
//         else if (piece === (pieceBitRep.black | pieceBitRep.pawn)) {
//             capturedPawnIndex = toIndex + 16;
//         }
//         // const epCell = document.getElementById(`cell-${capturedPawnIndex}`);
//         // while (epCell.firstChild) {
//         //     //Adds captured piece to capture box
//         //     updateCapturedPiecesDisplay(gameState.board[capturedPawnIndex]);
//         //     epCell.removeChild(epCell.firstChild);
//         // }
//         gameState.board[capturedPawnIndex] = 0;
//     }


//     //Calculates the En Passant Square if a pawn has moved twice
//     if ((piece === (pieceBitRep.white | pieceBitRep.pawn)) || (piece === (pieceBitRep.black | pieceBitRep.pawn))) {
//         if (Math.abs(fromIndex - toIndex) === 32) { // Moved 2 squares
//             // Compute and store the en passant square
//             gameState.enpassant = (fromIndex + toIndex) / 2;
//         } else {
//             gameState.enpassant = null;
//         }
//     } else {
//         gameState.enpassant = null;
//     }
//     pawnToQueen(toIndex, false);
//     //Update boardstate

//     gameState.lastMove = [fromIndex, toIndex, piece, takenPiece];
//     gameState.moveHistory.push([fromIndex, toIndex]);
//     //Check logic
//     clearPinnedPieces(gameState);
//     pinSearch(gameState, gameState.whiteKingPos);
//     pinSearch(gameState, gameState.blackKingPos);
//     updateAttackTable(gameState, toIndex);
//     updateAttackTable(gameState, fromIndex);
//     if (capturedPawnIndex !== null) {
//         updateAttackTable(gameState, capturedPawnIndex);
//     }
//     for (let i = 0; i < 64; i++) {
//         if (gameState.blackAttackTable[i].includes(toIndex)) {
//             updateAttackTable(gameState, toHexIndex(i));
//         } else if (gameState.blackAttackTable[i].includes(fromIndex)) {
//             updateAttackTable(gameState, toHexIndex(i));
//         } else if ((capturedPawnIndex !== null) && (gameState.blackAttackTable[i].includes(capturedPawnIndex))) {
//             updateAttackTable(gameState, toHexIndex(i));
//         } else if ((rookPosition !== null) && (gameState.blackAttackTable[i].includes(rookPosition))) {
//             updateAttackTable(gameState, toHexIndex(i));
//         }
//         if (gameState.whiteAttackTable[i].includes(toIndex)) {
//             updateAttackTable(gameState, toHexIndex(i));
//         } else if (gameState.whiteAttackTable[i].includes(fromIndex)) {
//             updateAttackTable(gameState, toHexIndex(i));
//         } else if ((capturedPawnIndex !== null) && (gameState.whiteAttackTable[i].includes(capturedPawnIndex))) {
//             updateAttackTable(gameState, toHexIndex(i));
//         } else if ((rookPosition !== null) && (gameState.whiteAttackTable[i].includes(rookPosition))) {
//             updateAttackTable(gameState, toHexIndex(i));
//         }
//     }
//     // runAttackTable(gameState);
//     if (gameState.turn == 0) {

//         if (isSquareUnderAttack(gameState, gameState.blackKingPos, pieceBitRep.white) == true) {
//             gameState.check = true;

//             // const kingCell = document.getElementById(`cell-${gameState.blackKingPos}`);
//             // kingCell.classList.add('blinking');
//             // console.log("Black King in check")
//             if (isCheckmate(gameState) == true) {
//                 console.log("checkmate");
//             }
//         } else {
//             if (isStalemate(gameState) == true) {
//                 console.log("stalemate");
//             }
//             gameState.check = false;
//             gameState.checkingSquares = [];
//         }
//         gameState.turn = 1;
//     } else {
//         if (isSquareUnderAttack(gameState, gameState.whiteKingPos, pieceBitRep.black) == true) {
//             gameState.check = true;

//             // console.log("White King in check")
//             // const kingCell = document.getElementById(`cell-${gameState.whiteKingPos}`);
//             // kingCell.classList.add('blinking');
//             if (isCheckmate(gameState)) {
//                 console.log("Checkmate")
//             }
//         } else {
//             if (isStalemate(gameState)) {
//                 console.log("Stalemate");
//             }
//             gameState.check = false;
//             gameState.checkingSquares = [];
//         }
//         gameState.turn = 0;
//     }
//     //Starts or swaps the timer when a piece is moved
//     // startTimer();
//     // //Adds Time Increments if turned on
//     // if (gameState.timeIncrementOn === true) {
//     //     addIncrementTime();
//     //     updateDisplay();
//     // }

// }
function restorePreviousGameState(depth = 1) {
    if (gameStateStack.length === 0) {
        return null;
    }
    for (let i = 0; i < depth; i++) {
        gameState = gameStateStack.pop();
    }


    // Logic to revert the gameState to the previous state...
    // updateDom();
    // After reverting the gameState, now update the DOM
    return gameState;
}

/*
Please familiarize yourself with how FEN works before reading this function
https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
*/
function loadFEN(fen) {
    //Start at the top left corner of the board
    // console.log("I made it here");
    var row = 7;
    var column = 0;
    //Squares is different from the 2d board that we may use later
    //Squares is a 1d array that represents the board in a linear fashion

    //Starting fen: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    //Clear board: 8/8/8/8/8/8/8/8 w - - 0 1
    //For populating the board, the only thing we care about is the first part of the FEN string
    // gameState[9] = new ulongInt(gameState, 0);
    // gameState[10] = new ulongInt(gameState, 0);
    // gameState[11] = new ulongInt(gameState, 0);
    // gameState[12] = new ulongInt(gameState, 0);
    // gameState[13] = new ulongInt(gameState, 0);
    // gameState[14] = new ulongInt(gameState, 0);
    // gameState[17] = new ulongInt(gameState, 0);
    // gameState[18] = new ulongInt(gameState, 0);
    // gameState[19] = new ulongInt(gameState, 0);
    // gameState[20] = new ulongInt(gameState, 0);
    // gameState[21] = new ulongInt(gameState, 0);
    // gameState[22] = new ulongInt(gameState, 0);

    var components = fen.split(" ");
    var boardfen = components[0];
    var turn = components[1];
    var castling = components[2];
    var enpassant = components[3];
    var halfmove = components[4];
    var fullmove = components[5];
    var slashCount = 0;
    //TODO: This section needs error handling
    // console.log(boardfen);
    // console.log(boardfen.length);
    for (let i = 0; i < boardfen.length; i++) {
        // console.log(i);
        let position = row * 16 + column;
        if (column > 8) {
            alert("Invalid FEN, slash omitted when expected")
            return null;
        }
        let c = boardfen[i]; //apparently using let here is preferred?
        // console.log(c);
        //In FEN a / indicates we move down a row, thus we decrement rank and set the file back to 0
        if (c === '/') {
            slashCount++;
            row--;
            column = 0;
        } else { //why tf is this the way you check if a char is a number in js?
            if (position < 0 || position > 119) {
                break;
            }
            if (c >= '0' && c <= '9') {
                //
                for (let j = 0; j < parseInt(c); j++) {
                    // console.log("Position: ", position)
                    gameState.board[position] = 0;
                    // console.log("Piece: ", pieceMapping[gameState.board[position]])
                    const cell = document.getElementById(`cell-${position}`);
                    while (cell.firstChild) {
                        cell.removeChild(cell.firstChild);
                    }
                    position++;
                    column++;
                }
            } else {
                if (c === 'k') {
                    gameState.blackKingPos = position;
                } else if (c === 'K') {
                    gameState.whiteKingPos = position;
                }

                let piece = fenMapping[c];
                gameState.board[position] = fenMapping[c];

                // Update the DOM
                const cell = document.getElementById(`cell-${position}`);

                // If there's a piece on the cell, remove it.
                while (cell.firstChild) {
                    cell.removeChild(cell.firstChild);
                }

                // Create a new child element for the piece and append it to the cell.
                let img = document.createElement('img');
                img.src = svgPaths[piece];
                cell.appendChild(img);
                position++;
                column++;
            }
        }
    }
    // console.log(slashCount);
    if (slashCount != 7) {
        alert("Invalid FEN, slash count wrong")
        return null;
    }
    // console.log(turn);
    if (!(turn === 'w' || turn === 'b')) {
        alert("Invalid turn")
        return null;
    }
    let turnNum = turn === 'w' ? 0 : 1;
    gameState.turn = turnNum;
    gameState.castlingWhite["king"] = false;
    gameState.castlingWhite["queen"] = false;
    gameState.castlingBlack["king"] = false;
    gameState.castlingBlack["queen"] = false;

    for (let i = 0; i < castling.length; i++) {
        let c = castling.charAt(i);
        if (c === 'K') {
            gameState.castlingWhite["king"] = true;
        } else if (c === 'Q') {
            gameState.castlingWhite["queen"] = true;
        } else if (c === 'k') {
            gameState.castlingBlack["king"] = true;
        } else if (c === 'q') {
            gameState.castlingBlack["queen"] = true;
        }
    }
    if (enpassant === '-') {
        gameState.enpassant = null;
    } else {
        var col = notationMapping[enpassant.charAt(0)];
        var row = notationMapping[parseInt(enpassant.charAt(1))];
        gameState.enpassant = row * 16 + col;
    }
    // gameState.halfMoveClock = parseInt(halfmove);
    // gameState.fullMoveNumber = parseInt(fullmove);
    // runAttackTable(gameState);
    // if (isSquareUnderAttack(gameState, gameState.whiteKingPos, pieceBitRep.black) == true) {
    //     gameState.check = true;
    // } else if (isSquareUnderAttack(gameState, gameState.blackKingPos, pieceBitRep.white) == true) {
    //     gameState.check = true;
    // }
    // pinSearch(gameState, gameState.whiteKingPos);
    // pinSearch(gameState, gameState.blackKingPos);
    // console.log(gameState.board);
}

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}



// async function loseChessGame() {
//     //await sleep(3500);
//     // First AJAX request to get_token.php to fetch the username
//     var xhrToken = new XMLHttpRequest();
//     xhrToken.open('GET', 'get_token.php', true);

//     xhrToken.onload = function () {
//         if (xhrToken.status >= 200 && xhrToken.status < 300) {
//             var response = JSON.parse(xhrToken.responseText);
//             if (response.status === 'valid') {
//                 var username = encodeURIComponent(response.username);

//                 // Second AJAX request to lose.php to update stats
//                 var xhrLose = new XMLHttpRequest();
//                 xhrLose.open('GET', 'lose.php', true);

//                 xhrLose.onload = function () {
//                     if (xhrLose.status >= 200 && xhrLose.status < 300) {
//                         console.log('lose.php executed successfully');
//                     } else {
//                         console.log('Request to lose.php failed with status: ' + xhrLose.status);
//                     }
//                     // Redirect to the user dashboard with username in URL, regardless of lose.php success
//                     //window.location.href = '../userDashboard?username=' + username;
//                 };

//                 xhrLose.onerror = function () {
//                     console.log('There was an error with the request to lose.php');
//                     // Redirect to the user dashboard with username in URL, even on error
//                     //window.location.href = '../userDashboard?username=' + username;
//                 };

//                 xhrLose.send();
//             } else {
//                 console.log('Invalid token or no user found.');
//                 //window.location.href = '../userDashboard';
//             }
//         } else {
//             console.log('Request to get_token.php failed with status: ' + xhrToken.status);
//             //window.location.href = '../userDashboard';
//         }
//     };

//     xhrToken.onerror = function () {
//         console.log('There was an error with the request to get_token.php');
//         //window.location.href = '../userDashboard';
//     };

//     xhrToken.send();
// }

// async function winChessGame() {
//     // Perform an AJAX request to win.php
//     winSound();
//     //await sleep(4000);
//     var xhrToken = new XMLHttpRequest();
//     xhrToken.open('GET', 'get_token.php', true);

//     xhrToken.onload = function () {
//         if (xhrToken.status >= 200 && xhrToken.status < 300) {
//             var response = JSON.parse(xhrToken.responseText);
//             if (response.status === 'valid') {
//                 var username = encodeURIComponent(response.username);

//                 // Second AJAX request to win.php to update stats
//                 var xhrLose = new XMLHttpRequest();
//                 xhrLose.open('GET', 'win.php', true);

//                 xhrLose.onload = function () {
//                     if (xhrLose.status >= 200 && xhrLose.status < 300) {
//                         console.log('win.php executed successfully');
//                     } else {
//                         console.log('Request to win.php failed with status: ' + xhrLose.status);
//                     }
//                     // Redirect to the user dashboard with username in URL, regardless of win.php success
//                     //window.location.href = '../userDashboard?username=' + username;
//                 };

//                 xhrLose.onerror = function () {
//                     console.log('There was an error with the request to win.php');
//                     // Redirect to the user dashboard with username in URL, even on error
//                     //window.location.href = '../userDashboard?username=' + username;
//                 };

//                 xhrLose.send();
//             } else {
//                 console.log('Invalid token or no user found.');
//                 //window.location.href = '../userDashboard';
//             }
//         } else {
//             console.log('Request to get_token.php failed with status: ' + xhrToken.status);
//             //window.location.href = '../userDashboard';
//         }
//     };

//     xhrToken.onerror = function () {
//         console.log('There was an error with the request to get_token.php');
//         //window.location.href = '../userDashboard';
//     };

//     xhrToken.send();
// }

// async function drawChessGame() {
//     //await sleep(3500);
//     // Perform an AJAX request to draw.php
//     var xhrToken = new XMLHttpRequest();
//     xhrToken.open('GET', 'get_token.php', true);

//     xhrToken.onload = function () {
//         if (xhrToken.status >= 200 && xhrToken.status < 300) {
//             var response = JSON.parse(xhrToken.responseText);
//             if (response.status === 'valid') {
//                 var username = encodeURIComponent(response.username);

//                 // Second AJAX request to draw.php to update stats
//                 var xhrLose = new XMLHttpRequest();
//                 xhrLose.open('GET', 'draw.php', true);

//                 xhrLose.onload = function () {
//                     if (xhrLose.status >= 200 && xhrLose.status < 300) {
//                         console.log('draw.php executed successfully');
//                     } else {
//                         console.log('Request to draw.php failed with status: ' + xhrLose.status);
//                     }
//                     // Redirect to the user dashboard with username in URL, regardless of draw.php success
//                     //window.location.href = '../userDashboard?username=' + username;
//                 };

//                 xhrLose.onerror = function () {
//                     console.log('There was an error with the request to draw.php');
//                     // Redirect to the user dashboard with username in URL, even on error
//                     //window.location.href = '../userDashboard?username=' + username;
//                 };

//                 xhrLose.send();
//             } else {
//                 console.log('Invalid token or no user found.');
//                 //window.location.href = '../userDashboard';
//             }
//         } else {
//             console.log('Request to get_token.php failed with status: ' + xhrToken.status);
//             //window.location.href = '../userDashboard';
//         }
//     };

//     xhrToken.onerror = function () {
//         console.log('There was an error with the request to get_token.php');
//         //window.location.href = '../userDashboard';
//     };

//     xhrToken.send();
// }
