
use board::*;
// use board::nort;
// use board::sout;
// use board::east_one;
// use board::west_one;

//Piece Square tables
fn flip_index(index: usize) -> usize {
    index ^ 56
}
const PAWN_VALUE: i32 = 100;
const KNIGHT_VALUE: i32 = 320;
const BISHOP_VALUE: i32 = 330;
const ROOK_VALUE: i32 = 500;
const QUEEN_VALUE: i32 = 900;
const KING_VALUE: i32 = 20000;

const MG_PASSED_PAWN: i32 = 25;
const EG_PASSED_PAWN: i32 = 50;
const BROKEN_PAWN_SHELTER: i32 = -50;
const ROOK_OPEN_FILE: i32 = 30;
const ROOK_SEMI_FILE: i32 = 5;

// const BLACK_MG_PAWN_TABLE: [i32; 64] = [
//      0,   0,   0,   0,   0,   0,  0,   0,
//     198, 234, 161, 195, 168, 226, 134,  89,
//      94, 107, 126, 131, 165, 156, 125,  80,
//      86, 113, 106, 121, 123, 112, 117,  77,
//      73,  98,  95, 112, 117, 106, 110,  75,
//      74,  96,  96,  90, 103, 103, 133,  88,
//      65,  99,  80,  77,  85, 124, 138,  78,
//      0,   0,   0,   0,   0,   0,  0,   0,
// ];
// const WHITE_MG_PAWN_TABLE: [i32; 64] = [
//      0,   0,   0,   0,   0,   0,  0,   0,
//      65,  99,  80,  77,  85, 124, 138,  78,
//      74,  96,  96,  90, 103, 103, 133,  88,
//      73,  98,  95, 112, 117, 106, 110,  75,
//      86, 113, 106, 121, 123, 112, 117,  77,
//      94, 107, 126, 131, 165, 156, 125,  80,
//     198, 234, 161, 195, 168, 226, 134,  89,
//      0,   0,   0,   0,   0,   0,  0,   0,
// ];

// const BLACK_EG_PAWN_TABLE: [i32; 64] = [
//      0,   0,   0,   0,   0,   0,   0,   0,
//    278, 273, 258, 234, 247, 232, 265, 287,
//    194, 200, 185, 167, 156, 153, 182, 184,
//    132, 124, 113, 105,  98, 104, 117, 117,
//    113, 109,  97,  93,  93,  92, 103,  99,
//    104, 107,  94, 101, 100,  95,  99,  92,
//    113, 108, 108, 110, 113, 100, 102,  93,
//      0,   0,   0,   0,   0,   0,   0,   0,
// ];
// const WHITE_EG_PAWN_TABLE: [i32; 64] = [
//      0,   0,   0,   0,   0,   0,   0,   0,
//    113, 108, 108, 110, 113, 100, 102,  93,
//    104, 107,  94, 101, 100,  95,  99,  92,
//    113, 109,  97,  93,  93,  92, 103,  99,
//    132, 124, 113, 105,  98, 104, 117, 117,
//    194, 200, 185, 167, 156, 153, 182, 184,
//    278, 273, 258, 234, 247, 232, 265, 287,
//      0,   0,   0,   0,   0,   0,   0,   0,
// ];

// const BLACK_MG_KNIGHT_TABLE: [i32; 64] = [
//     153, 231, 286, 271, 381, 223, 305, 213,
//     247, 279, 392, 356, 343, 382, 327, 303,
//     273, 380, 357, 385, 404, 449, 393, 364,
//     311, 337, 339, 373, 357, 389, 338, 342,
//     307, 316, 336, 333, 348, 339, 341, 312,
//     297, 311, 332, 330, 339, 337, 345, 304,
//     291, 267, 308, 317, 319, 338, 306, 301,
//     215, 299, 262, 287, 303, 292, 301, 297,
// ];

// const WHITE_MG_KNIGHT_TABLE: [i32; 64] = [
//     215, 299, 262, 287, 303, 292, 301, 297,
//     291, 267, 308, 317, 319, 338, 306, 301,
//     297, 311, 332, 330, 339, 337, 345, 304,
//     307, 316, 336, 333, 348, 339, 341, 312,
//     311, 337, 339, 373, 357, 389, 338, 342,
//     273, 380, 357, 385, 404, 449, 393, 364,
//     247, 279, 392, 356, 343, 382, 327, 303,
//     153, 231, 286, 271, 381, 223, 305, 213,
// ];

// const BLACK_EG_KNIGHT_TABLE: [i32; 64] = [
//     262, 282, 307, 292, 289, 293, 257, 221,
//     295, 312, 295, 318, 311, 295, 296, 268,
//     296, 300, 330, 329, 319, 311, 301, 279,
//     303, 323, 342, 342, 342, 331, 328, 302,
//     302, 314, 336, 345, 336, 337, 324, 302,
//     297, 317, 319, 335, 330, 317, 300, 298,
//     278, 300, 310, 315, 318, 300, 297, 276,
//     291, 269, 297, 305, 298, 302, 270, 256,
// ];

// const WHITE_EG_KNIGHT_TABLE: [i32; 64] = [
//     291, 269, 297, 305, 298, 302, 270, 256,
//     278, 300, 310, 315, 318, 300, 297, 276,
//     297, 317, 319, 335, 330, 317, 300, 298,
//     302, 314, 336, 345, 336, 337, 324, 302,
//     303, 323, 342, 342, 342, 331, 328, 302,
//     296, 300, 330, 329, 319, 311, 301, 279,
//     295, 312, 295, 318, 311, 295, 296, 268,
//     262, 282, 307, 292, 289, 293, 257, 221,
// ];

// const BLACK_MG_BISHOP_TABLE: [i32; 64] = [
//     301, 334, 248, 293, 305, 288, 337, 322,
//     304, 346, 312, 317, 360, 389, 348, 283,
//     314, 367, 373, 370, 365, 380, 367, 328,
//     326, 335, 349, 380, 367, 367, 337, 328,
//     324, 343, 343, 356, 364, 342, 340, 334,
//     330, 345, 345, 345, 344, 357, 348, 340,
//     334, 345, 346, 330, 337, 351, 363, 331,
//     297, 327, 316, 309, 317, 318, 291, 309,
// ];

// const WHITE_MG_BISHOP_TABLE: [i32; 64] = [
//     297, 327, 316, 309, 317, 318, 291, 309,
//     334, 345, 346, 330, 337, 351, 363, 331,
//     330, 345, 345, 345, 344, 357, 348, 340,
//     324, 343, 343, 356, 364, 342, 340, 334,
//     326, 335, 349, 380, 367, 367, 337, 328,
//     314, 367, 373, 370, 365, 380, 367, 328,
//     304, 346, 312, 317, 360, 389, 348, 283,
//     301, 334, 248, 293, 305, 288, 337, 322,
// ];

// const BLACK_EG_BISHOP_TABLE: [i32; 64] = [
//     316, 309, 319, 322, 323, 321, 313, 306,
//     322, 326, 337, 318, 327, 317, 326, 316,
//     332, 322, 330, 329, 328, 336, 330, 334,
//     327, 339, 342, 339, 344, 340, 333, 332,
//     324, 333, 343, 349, 337, 340, 327, 321,
//     318, 327, 338, 340, 343, 333, 323, 315,
//     316, 312, 323, 329, 334, 321, 315, 303,
//     307, 321, 307, 325, 321, 314, 325, 313,
// ];
// const WHITE_EG_BISHOP_TABLE: [i32; 64] = [
//     307, 321, 307, 325, 321, 314, 325, 313,
//     316, 312, 323, 329, 334, 321, 315, 303,
//     318, 327, 338, 340, 343, 333, 323, 315,
//     324, 333, 343, 349, 337, 340, 327, 321,
//     327, 339, 342, 339, 344, 340, 333, 332,
//     332, 322, 330, 329, 328, 336, 330, 334,
//     322, 326, 337, 318, 327, 317, 326, 316,
//     316, 309, 319, 322, 323, 321, 313, 306,
// ];

// const BLACK_MG_ROOK_TABLE: [i32; 64] = [
//     532, 542, 532, 551, 563, 509, 531, 543,
//     527, 532, 558, 562, 580, 567, 526, 544,
//     495, 519, 526, 536, 517, 545, 561, 516,
//     476, 489, 507, 526, 524, 535, 492, 480,
//     464, 474, 488, 499, 509, 493, 506, 477,
//     455, 475, 484, 483, 503, 500, 495, 467,
//     456, 484, 480, 491, 499, 511, 494, 429,
//     481, 487, 501, 517, 516, 507, 463, 474,
// ];

// const WHITE_MG_ROOK_TABLE: [i32; 64] = [
//     481, 487, 501, 517, 516, 507, 463, 474,
//     456, 484, 480, 491, 499, 511, 494, 429,
//     455, 475, 484, 483, 503, 500, 495, 467,
//     464, 474, 488, 499, 509, 493, 506, 477,
//     476, 489, 507, 526, 524, 535, 492, 480,
//     495, 519, 526, 536, 517, 545, 561, 516,
//     527, 532, 558, 562, 580, 567, 526, 544,
//     532, 542, 532, 551, 563, 509, 531, 543,
// ];


// const BLACK_EG_ROOK_TABLE: [i32; 64] = [
//     513, 510, 518, 515, 512, 512, 508, 505,
//     511, 513, 513, 511, 497, 503, 508, 503,
//     507, 507, 507, 505, 504, 497, 495, 497,
//     504, 503, 513, 501, 502, 501, 499, 502,
//     503, 505, 508, 504, 495, 494, 492, 489,
//     496, 500, 495, 499, 493, 488, 492, 484,
//     494, 494, 500, 502, 491, 491, 489, 497,
//     491, 502, 503, 499, 495, 487, 504, 480,
// ];

// const WHITE_EG_ROOK_TABLE: [i32; 64] = [
//     491, 502, 503, 499, 495, 487, 504, 480,
//     494, 494, 500, 502, 491, 491, 489, 497,
//     496, 500, 495, 499, 493, 488, 492, 484,
//     503, 505, 508, 504, 495, 494, 492, 489,
//     504, 503, 513, 501, 502, 501, 499, 502,
//     507, 507, 507, 505, 504, 497, 495, 497,
//     511, 513, 513, 511, 497, 503, 508, 503,
//     513, 510, 518, 515, 512, 512, 508, 505,
// ];

// const BLACK_MG_QUEEN_TABLE: [i32; 64] = [
//     872, 900, 929, 912, 959, 944, 943, 945,
//     876, 861, 895, 901, 884, 957, 928, 954,
//     887, 883, 907, 908, 929, 956, 947, 957,
//     873, 873, 884, 884, 899, 917, 898, 901,
//     891, 874, 891, 890, 898, 896, 903, 897,
//     886, 902, 889, 898, 895, 902, 914, 905,
//     865, 892, 911, 902, 908, 915, 897, 901,
//     899, 882, 891, 910, 885, 875, 869, 850,
// ];

// const WHITE_MG_QUEEN_TABLE: [i32; 64] = [
//     899, 882, 891, 910, 885, 875, 869, 850,
//     865, 892, 911, 902, 908, 915, 897, 901,
//     886, 902, 889, 898, 895, 902, 914, 905,
//     891, 874, 891, 890, 898, 896, 903, 897,
//     873, 873, 884, 884, 899, 917, 898, 901,
//     887, 883, 907, 908, 929, 956, 947, 957,
//     876, 861, 895, 901, 884, 957, 928, 954,
//     872, 900, 929, 912, 959, 944, 943, 945,
// ];

// const BLACK_EG_QUEEN_TABLE: [i32; 64] = [
//     891, 922, 922, 927, 927, 919, 910, 920,
//     883, 920, 932, 941, 958, 925, 930, 900,
//     880, 906, 909, 949, 947, 935, 919, 909,
//     903, 922, 924, 945, 957, 940, 957, 936,
//     882, 928, 919, 947, 931, 934, 939, 923,
//     884, 873, 915, 906, 909, 917, 910, 905,
//     878, 877, 870, 884, 884, 877, 864, 868,
//     867, 872, 878, 857, 895, 868, 880, 859,
// ];

// const WHITE_EG_QUEEN_TABLE: [i32; 64] = [
//     867, 872, 878, 857, 895, 868, 880, 859,
//     878, 877, 870, 884, 884, 877, 864, 868,
//     884, 873, 915, 906, 909, 917, 910, 905,
//     882, 928, 919, 947, 931, 934, 939, 923,
//     903, 922, 924, 945, 957, 940, 957, 936,
//     880, 906, 909, 949, 947, 935, 919, 909,
//     883, 920, 932, 941, 958, 925, 930, 900,
//     891, 922, 922, 927, 927, 919, 910, 920,
// ];

// const BLACK_MG_KING_TABLE: [i32; 64] = [
//     19935, 20023, 20016, 19985, 19944, 19966, 20002, 20013,
//     20029, 19999, 19980, 19993, 19992, 19996, 19962, 19971,
//     19991, 20024, 20002, 19984, 19980, 20006, 20022, 19978,
//     19983, 19980, 19988, 19973, 19970, 19975, 19986, 19964,
//     19951, 19999, 19973, 19961, 19954, 19956, 19967, 19949,
//     19986, 19986, 19978, 19954, 19956, 19970, 19985, 19973,
//     20001, 20007, 19992, 19936, 19957, 19984, 20009, 20008,
//     19985, 20036, 20012, 19946, 20008, 19972, 20024, 20014,
// ];

// const WHITE_MG_KING_TABLE: [i32; 64] = [
//     19985, 20036, 20012, 19946, 20008, 19972, 20024, 20014,
//     20001, 20007, 19992, 19936, 19957, 19984, 20009, 20008,
//     19986, 19986, 19978, 19954, 19956, 19970, 19985, 19973,
//     19951, 19999, 19973, 19961, 19954, 19956, 19967, 19949,
//     19983, 19980, 19988, 19973, 19970, 19975, 19986, 19964,
//     19991, 20024, 20002, 19984, 19980, 20006, 20022, 19978,
//     20029, 19999, 19980, 19993, 19992, 19996, 19962, 19971,
//     19935, 20023, 20016, 19985, 19944, 19966, 20002, 20013,
// ];

// const BLACK_EG_KING_TABLE: [i32; 64] = [
//     19926, 19965, 19982, 19982, 19989, 20015, 20004, 19983,
//     19988, 20017, 20014, 20017, 20017, 20038, 20023, 20011,
//     20010, 20017, 20023, 20015, 20020, 20045, 20044, 20013,
//     19992, 20022, 20024, 20027, 20026, 20033, 20026, 20003,
//     19982, 19996, 20021, 20024, 20027, 20023, 20009, 19989,
//     19981, 19997, 20011, 20021, 20023, 20016, 20007, 19991,
//     19973, 19989, 20004, 20013, 20014, 20004, 19995, 19983,
//     19947, 19966, 19979, 19989, 19972, 19986, 19976, 19957
// ];

// const WHITE_EG_KING_TABLE: [i32; 64] = [
//     19947, 19966, 19979, 19989, 19972, 19986, 19976, 19957,
//     19973, 19989, 20004, 20013, 20014, 20004, 19995, 19983,
//     19981, 19997, 20011, 20021, 20023, 20016, 20007, 19991,
//     19982, 19996, 20021, 20024, 20027, 20023, 20009, 19989,
//     19992, 20022, 20024, 20027, 20026, 20033, 20026, 20003,
//     20010, 20017, 20023, 20015, 20020, 20045, 20044, 20013,
//     19988, 20017, 20014, 20017, 20017, 20038, 20023, 20011,
//     19926, 19965, 19982, 19982, 19989, 20015, 20004, 19983
// ];
const MG_PAWN_TABLE: [i32; 64] = [
     0,   0,   0,   0,   0,   0,  0,   0,
    98, 134,  61,  95,  68, 126, 34, -11,
    -6,   7,  26,  31,  65,  56, 25, -20,
   -14,  13,   6,  21,  23,  12, 17, -23,
   -27,  -2,  -5,  12,  17,   6, 10, -25,
   -26,  -4,  -4, -10,   3,   3, 33, -12,
   -35,  -1, -20, -23, -15,  24, 38, -22,
     0,   0,   0,   0,   0,   0,  0,   0,
];

const EG_PAWN_TABLE: [i32; 64] = [
     0,   0,   0,   0,   0,   0,   0,   0,
   178, 173, 158, 134, 147, 132, 165, 187,
    94, 100,  85,  67,  56,  53,  82,  84,
    32,  24,  13,   5,  -2,   4,  17,  17,
    13,   9,  -3,  -7,  -7,  -8,   3,  -1,
     4,   7,  -6,   1,   0,  -5,  -1,  -8,
    13,   8,   8,  10,  13,   0,   2,  -7,
     0,   0,   0,   0,   0,   0,   0,   0,
];
const MG_KNIGHT_TABLE: [i32; 64] = [
  -167, -89, -34, -49,  61, -97, -15, -107,
   -73, -41,  72,  36,  23,  62,   7,  -17,
   -47,  60,  37,  65,  84, 129,  73,   44,
    -9,  17,  19,  53,  37,  69,  18,   22,
   -13,   4,  16,  13,  28,  19,  21,   -8,
   -23,  -9,  12,  10,  19,  17,  25,  -16,
   -29, -53, -12,  -3,  -1,  18, -14,  -19,
  -105, -21, -58, -33, -17, -28, -19,  -23,
];


const EG_KNIGHT_TABLE: [i32; 64] = [
    -58, -38, -13, -28, -31, -27, -63, -99,
    -25,  -8, -25,  -2,  -9, -25, -24, -52,
    -24, -20,  10,   9,  -1,  -9, -19, -41,
    -17,   3,  22,  22,  22,  11,   8, -18,
    -18,  -6,  16,  25,  16,  17,   4, -18,
    -23,  -3,  -1,  15,  10,  -3, -20, -22,
    -42, -20, -10,  -5,  -2, -20, -23, -44,
    -29, -51, -23, -15, -22, -18, -50, -64,
];

const MG_BISHOP_TABLE: [i32; 64] = [
  -29,   4, -82, -37, -25, -42,   7,  -8,
  -26,  16, -18, -13,  30,  59,  18, -47,
  -16,  37,  43,  40,  35,  50,  37,  -2,
   -4,   5,  19,  50,  37,  37,   7,  -2,
   -6,  13,  13,  26,  34,  12,  10,   4,
    0,  15,  15,  15,  14,  27,  18,  10,
    4,  15,  16,   0,   7,  21,  33,   1,
  -33,  -3, -14, -21, -13, -12, -39, -21,
];

const EG_BISHOP_TABLE: [i32; 64] = [
  -14, -21, -11,  -8, -7,  -9, -17, -24,
   -8,  -4,   7, -12, -3, -13,  -4, -14,
    2,  -8,   0,  -1, -2,   6,   0,   4,
   -3,   9,  12,   9, 14,  10,   3,   2,
   -6,   3,  13,  19,  7,  10,  -3,  -9,
  -12,  -3,   8,  10, 13,   3,  -7, -15,
  -14, -18,  -7,  -1,  4,  -9, -15, -27,
  -23,  -9, -23,  -5, -9, -16,  -5, -17,
];

const MG_ROOK_TABLE: [i32; 64] = [
   32,  42,  32,  51, 63,  9,  31,  43,
   27,  32,  58,  62, 80, 67,  26,  44,
   -5,  19,  26,  36, 17, 45,  61,  16,
  -24, -11,   7,  26, 24, 35,  -8, -20,
  -36, -26, -12,  -1,  9, -7,   6, -23,
  -45, -25, -16, -17,  3,  0,  -5, -33,
  -44, -16, -20,  -9, -1, 11,  -6, -71,
  -19, -13,   1,  17, 16,  7, -37, -26,
];

const EG_ROOK_TABLE: [i32; 64] = [
  13, 10, 18, 15, 12,  12,   8,   5,
  11, 13, 13, 11, -3,   3,   8,   3,
   7,  7,  7,  5,  4,  -3,  -5,  -3,
   4,  3, 13,  1,  2,   1,  -1,   2,
   3,  5,  8,  4, -5,  -6,  -8, -11,
  -4,  0, -5, -1, -7, -12,  -8, -16,
  -6, -6,  0,  2, -9,  -9, -11,  -3,
  -9,  2,  3, -1, -5, -13,   4, -20,
];

const MG_QUEEN_TABLE: [i32; 64] = [
  -28,   0,  29,  12,  59,  44,  43,  45,
  -24, -39,  -5,   1, -16,  57,  28,  54,
  -13, -17,   7,   8,  29,  56,  47,  57,
  -27, -27, -16, -16,  -1,  17,  -2,   1,
   -9, -26,  -9, -10,  -2,  -4,   3,  -3,
  -14,   2, -11,  -2,  -5,   2,  14,   5,
  -35,  -8,  11,   2,   8,  15,  -3,   1,
   -1, -18,  -9,  10, -15, -25, -31, -50,
];

const EG_QUEEN_TABLE: [i32; 64] = [
   -9,  22,  22,  27,  27,  19,  10,  20,
  -17,  20,  32,  41,  58,  25,  30,   0,
  -20,   6,   9,  49,  47,  35,  19,   9,
    3,  22,  24,  45,  57,  40,  57,  36,
  -18,  28,  19,  47,  31,  34,  39,  23,
  -16, -27,  15,   6,   9,  17,  10,   5,
  -22, -23, -30, -16, -16, -23, -36, -32,
  -33, -28, -22, -43,  -5, -32, -20, -41,
];

const MG_KING_TABLE: [i32; 64] = [
  -65,  23,  16, -15, -56, -34,   2,  13,
   29,  -1, -20,  -7,  -8,  -4, -38, -29,
   -9,  24,   2, -16, -20,   6,  22, -22,
  -17, -20, -12, -27, -30, -25, -14, -36,
  -49,  -1, -27, -39, -46, -44, -33, -51,
  -14, -14, -22, -46, -44, -30, -15, -27,
    1,   7,  -8, -64, -43, -16,   9,   8,
  -15,  36,  12, -54,   8, -28,  24,  14,
];

const EG_KING_TABLE: [i32; 64] = [
  -74, -35, -18, -18, -11,  15,   4, -17,
  -12,  17,  14,  17,  17,  38,  23,  11,
   10,  17,  23,  15,  20,  45,  44,  13,
   -8,  22,  24,  27,  26,  33,  26,   3,
  -18,  -4,  21,  24,  27,  23,   9, -11,
  -19,  -3,  11,  21,  23,  16,   7,  -9,
  -27, -11,   4,  13,  14,   4,  -5, -17,
  -53, -34, -21, -11, -28, -14, -24, -43
];
//File utility functions from https://www.chessprogramming.org/Pawn_Fills
pub fn north_fill(pawns: u64) -> u64 {
  let mut pawns = pawns;
  pawns |= pawns << 8;
  pawns |= pawns << 16;
  pawns |= pawns << 32;
  pawns
}

pub fn south_fill(pawns: u64)-> u64 {
  let mut pawns = pawns;
  pawns |= pawns >> 8;
  pawns |= pawns >> 16;
  pawns |= pawns >> 32;
  pawns
}

pub fn white_front_span(wpawns: u64)-> u64{
  nort(north_fill(wpawns))
}

pub fn black_rear_span(bpawns: u64)-> u64{
  nort(north_fill(bpawns))
}

pub fn black_front_span(bpawns: u64)-> u64{
  sout(south_fill(bpawns))
}

pub fn white_rear_span(wpawns: u64)-> u64{
  sout(south_fill(wpawns))
}
pub fn file_fill(pawns: u64) -> u64 {
  north_fill(pawns) | south_fill(pawns)
}
pub fn w_pawns_behind(wpawns: u64) -> u64 {
  wpawns & white_rear_span(wpawns)
}
pub fn w_pawns_front(wpawns: u64) -> u64 {
  wpawns & white_front_span(wpawns)
}
//These are setwise, they simply count the number of passed pawns
pub fn w_passed_pawn_count(wpawns: u64, bpawns: u64)->i32{
  let mut all_front_spans = black_front_span(bpawns);
  all_front_spans |= east_one(all_front_spans) | west_one(all_front_spans);
  (wpawns & !all_front_spans).count_ones() as i32
}
pub fn b_passed_pawn_count(wpawns: u64, bpawns: u64)->i32{
  let mut all_front_spans = white_front_span(wpawns);
  all_front_spans |= east_one(all_front_spans) | west_one(all_front_spans);
  (bpawns & !all_front_spans).count_ones() as i32
}

//Not sure if we are going to need this
pub fn triple_pawns_count(wpawns: u64) -> u32 {
  let pawns_ahead_own: u64 = wpawns & white_front_span(wpawns);
  let pawns_behind_own: u64 = wpawns & white_rear_span(wpawns);
  let pawns_ahead_and_behind: u64 = pawns_ahead_own & pawns_behind_own;
  pawns_ahead_and_behind.count_ones()
}

pub fn evaluate_board(board: & mut Board) -> i32 {
  let mut score: i32 = 0;
  let mut egPhase: i32 = 0;
  //Perhaps calculate material can also pass the eg phase to allow for phase tapering

  [score, egPhase] = calculate_material(board);
  score += calculate_pawn_structure(board, egPhase);
  // score += calculate_king_safety(board);
  score += calculate_bishop_pair(board);

  score += rook_on_open_file(board);
  score += rook_on_semi_open_file(board);

  score
}

pub fn calculate_material(board: &Board) -> [i32; 2] {
  //These numbers are based on pesto game phase increment count
  let knight = 1;
  let bishop = 1;
  let rook = 2;
  let queen = 4;

  let mut mgscore: i32 = 0;
  let mut egscore: i32 = 0;
  let mut game_phase: i32 = 0;
  let mut wpawns = board.pawns & board.white;
  let mut bpawns = board.pawns & board.black;
  while wpawns != 0 {
    let index = wpawns.trailing_zeros() as usize;
    mgscore += PAWN_VALUE + MG_PAWN_TABLE[flip_index(index)];
    egscore += PAWN_VALUE + EG_PAWN_TABLE[flip_index(index)];
    wpawns &= wpawns - 1;
  }
  while bpawns != 0 {
    let index = bpawns.trailing_zeros() as usize;
    mgscore -= PAWN_VALUE + MG_PAWN_TABLE[index];
    egscore -= PAWN_VALUE + EG_PAWN_TABLE[index];
    bpawns &= bpawns - 1;
  }
  let mut wknights = board.knights & board.white;
  let mut bknights = board.knights & board.black;
  while wknights != 0 {
    let index = wknights.trailing_zeros() as usize;
    mgscore += KNIGHT_VALUE + MG_KNIGHT_TABLE[flip_index(index)];
    egscore += KNIGHT_VALUE + EG_KNIGHT_TABLE[flip_index(index)];
    game_phase += knight;
    wknights &= wknights - 1;
  }
  while bknights != 0 {
    let index = bknights.trailing_zeros() as usize;
    mgscore -= KNIGHT_VALUE + MG_KNIGHT_TABLE[index];
    egscore -= KNIGHT_VALUE + EG_KNIGHT_TABLE[index];
    game_phase += knight;
    bknights &= bknights - 1;
  }
  let mut wbishops = board.bishops & board.white;
  let mut bbishops = board.bishops & board.black;
  while wbishops != 0 {
    let index = wbishops.trailing_zeros() as usize;
    mgscore += BISHOP_VALUE + MG_BISHOP_TABLE[flip_index(index)];
    egscore += BISHOP_VALUE + EG_BISHOP_TABLE[flip_index(index)];
    game_phase += bishop;
    wbishops &= wbishops - 1;
  }
  while bbishops != 0 {
    let index = bbishops.trailing_zeros() as usize;
    mgscore -= BISHOP_VALUE + MG_BISHOP_TABLE[index];
    egscore -= BISHOP_VALUE + EG_BISHOP_TABLE[index];
    game_phase += bishop;
    bbishops &= bbishops - 1;
  }
  let mut wrooks = board.rooks & board.white;
  let mut brooks = board.rooks & board.black;
  while wrooks != 0 {
    let index = wrooks.trailing_zeros() as usize;
    mgscore += ROOK_VALUE + MG_ROOK_TABLE[flip_index(index)];
    egscore += ROOK_VALUE + EG_ROOK_TABLE[flip_index(index)];
    game_phase += rook;
    wrooks &= wrooks - 1;
  }
  while brooks != 0 {
    let index = brooks.trailing_zeros() as usize;
    mgscore -= ROOK_VALUE + MG_ROOK_TABLE[index];
    egscore -= ROOK_VALUE + EG_ROOK_TABLE[index];
    game_phase += rook;
    brooks &= brooks - 1;
  }
  let mut wqueens = board.queens & board.white;
  let mut bqueens = board.queens & board.black;
  while wqueens != 0 {
    let index = wqueens.trailing_zeros() as usize;
    mgscore += QUEEN_VALUE + MG_QUEEN_TABLE[flip_index(index)];
    egscore += QUEEN_VALUE + EG_QUEEN_TABLE[flip_index(index)];
    game_phase += queen;
    wqueens &= wqueens - 1;
  }
  while bqueens != 0 {
    let index = bqueens.trailing_zeros() as usize;
    mgscore -= QUEEN_VALUE + MG_QUEEN_TABLE[index];
    egscore -= QUEEN_VALUE + EG_QUEEN_TABLE[index];
    game_phase += queen;
    bqueens &= bqueens - 1;
  }
  let mut wkings = board.kings & board.white;
  let mut bkings = board.kings & board.black;
  while wkings != 0 {
    let index = wkings.trailing_zeros() as usize;
    mgscore += KING_VALUE + MG_KING_TABLE[flip_index(index)];
    egscore += KING_VALUE + EG_KING_TABLE[flip_index(index)];
    wkings &= wkings - 1;
  }
  while bkings != 0 {
    let index = bkings.trailing_zeros() as usize;
    mgscore -= KING_VALUE + MG_KING_TABLE[index];
    egscore -= KING_VALUE + EG_KING_TABLE[index];
    bkings &= bkings - 1;
  }
  if game_phase > 24 {
    game_phase = 24;
  }
  let egPhase = 24 - game_phase;
  [(mgscore * game_phase + egscore * egPhase) / 24, egPhase]
  
}

pub fn calculate_pawn_structure(board: &Board, egphase: i32) -> i32 {
  let mut score: i32 = 0;
  let mut wpawns: u64 = board.pawns & board.white;
  let mut bpawns: u64 = board.pawns & board.black;
  let multiplier = (MG_PASSED_PAWN * (24-egphase) + EG_PASSED_PAWN * egphase)/24;
  let wpassed_pawns = w_passed_pawn_count(wpawns, bpawns);
  let bpassed_pawns = b_passed_pawn_count(wpawns, bpawns);
  score += wpassed_pawns * multiplier;
  score -= bpassed_pawns * multiplier;
  //TODO: Add more pawn structure evaluation

  score
}

pub fn calculate_king_safety(board: &Board) -> i32 {
  let mut score: i32 = 0;

  score
}


pub fn calculate_bishop_pair(board: &Board) -> i32 {
  let mut score: i32 = 0;
  let mut wbishops = board.bishops & board.white;
  let mut bbishops = board.bishops & board.black;
  if wbishops.count_ones() > 1 {
    score += 50;
  }
  if bbishops.count_ones() > 1 {
    score -= 50;
  }
  score
}

pub fn rook_on_open_file(board: &Board) -> i32 {
  let mut score: i32 = 0;
  let mut wrooks = board.rooks & board.white;
  let mut brooks = board.rooks & board.black;
  let mut wrooks_on_open_file = wrooks & !file_fill(board.pawns);
  let mut brooks_on_open_file = brooks & !file_fill(board.pawns);
  while wrooks_on_open_file != 0 {
    let index = wrooks_on_open_file.trailing_zeros() as usize;
    score += ROOK_OPEN_FILE;
    wrooks_on_open_file &= wrooks_on_open_file - 1;
  }
  while brooks_on_open_file != 0 {
    let index = brooks_on_open_file.trailing_zeros() as usize;
    score -= ROOK_OPEN_FILE;
    brooks_on_open_file &= brooks_on_open_file - 1;
  }
  score
}

pub fn rook_on_semi_open_file(board: &Board) -> i32 {
  let mut score: i32 = 0;
  let mut wrooks = board.rooks & board.white;
  let mut brooks = board.rooks & board.black;
  let mut wrooks_on_semi_open_file = wrooks & file_fill(board.pawns) & !board.pawns;
  let mut brooks_on_semi_open_file = brooks & file_fill(board.pawns) & !board.pawns;
  while wrooks_on_semi_open_file != 0 {
    let index = wrooks_on_semi_open_file.trailing_zeros() as usize;
    score += ROOK_SEMI_FILE;
    wrooks_on_semi_open_file &= wrooks_on_semi_open_file - 1;
  }
  while brooks_on_semi_open_file != 0 {
    let index = brooks_on_semi_open_file.trailing_zeros() as usize;
    score -= ROOK_SEMI_FILE;
    brooks_on_semi_open_file &= brooks_on_semi_open_file - 1;
  }
  score
}

pub fn ab_pruning(board: &mut Board, initial_alpha: i32, initial_beta: i32, mve: (u8, u8), depth: u32, maximizing_player: bool) -> (i32, (u8, u8), u32) {
  let mut node_count = 1;

  if depth == 0 {
      return (evaluate_board(board) as i32, mve, node_count);
  }

  let moves = generate_legal_moves(board);
  if moves.len() == 0 {
      if is_check(board) {
          if maximizing_player {
              return (i32::MIN + depth as i32, mve, node_count);
          } else {
              return (i32::MAX - depth as i32, mve, node_count);
          }
      } else {
          return (0, mve, node_count);
      }
  }
  let mut best_move = mve;
  if maximizing_player {
      let mut value = i32::MIN;
      let mut alpha = initial_alpha;

      for i in (0..moves.len()).step_by(2) {
          let mut new_board: Board = simulate_move(board, moves[i], moves[i + 1]);


          let (score, _, child_node_count) = ab_pruning(&mut new_board, alpha, initial_beta, (moves[i], moves[i + 1]), depth - 1, false);
          node_count += child_node_count;

          if score > value {
              value = score;
              best_move = (moves[i], moves[i + 1]);
          }
          alpha = alpha.max(value);
          if value >= initial_beta {
              break;
          }
      }
      (value, best_move, node_count)
  } else {
      let mut value = i32::MAX;
      let mut beta = initial_beta;

      for i in (0..moves.len()).step_by(2) {
          let mut new_board = simulate_move(board, moves[i], moves[i + 1]);

          let (score, _, child_node_count) = ab_pruning(&mut new_board, initial_alpha, beta, (moves[i], moves[i + 1]), depth - 1, true);
          node_count += child_node_count;

          if score < value {
              value = score;
              best_move = (moves[i], moves[i + 1]);
          }
          beta = beta.min(value);
          if value <= initial_alpha {
              break;
          }
      }
      (value, best_move, node_count)
  }
}
