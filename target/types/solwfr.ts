/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/solwfr.json`.
 */
export type Solwfr = {
  "address": "GNsGuF2dfgs4NniXa5vwZ9UcE9XdQPBU9AsJFvxT9hzv",
  "metadata": {
    "name": "solwfr",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "buyTicket",
      "discriminator": [
        11,
        24,
        17,
        193,
        168,
        116,
        164,
        169
      ],
      "accounts": [
        {
          "name": "lottery",
          "writable": true
        },
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "ticket",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "ticketAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "claimPrize",
      "discriminator": [
        157,
        233,
        139,
        121,
        246,
        62,
        234,
        235
      ],
      "accounts": [
        {
          "name": "lottery",
          "writable": true
        },
        {
          "name": "ticket"
        },
        {
          "name": "buyer",
          "writable": true,
          "signer": true,
          "relations": [
            "ticket"
          ]
        },
        {
          "name": "siteAuthority",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "programState",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "closeTable",
      "discriminator": [
        149,
        214,
        44,
        14,
        190,
        244,
        132,
        48
      ],
      "accounts": [
        {
          "name": "programState",
          "writable": true
        },
        {
          "name": "table",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  97,
                  98,
                  108,
                  101
                ]
              },
              {
                "kind": "arg",
                "path": "tid"
              }
            ]
          }
        },
        {
          "name": "dealer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "tid",
          "type": "u64"
        }
      ]
    },
    {
      "name": "createLottery",
      "discriminator": [
        242,
        165,
        247,
        119,
        17,
        203,
        21,
        42
      ],
      "accounts": [
        {
          "name": "programState",
          "writable": true
        },
        {
          "name": "lottery",
          "writable": true
        },
        {
          "name": "authority",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "ticketPrice",
          "type": "u64"
        },
        {
          "name": "title",
          "type": "string"
        },
        {
          "name": "timeLimit",
          "type": "u64"
        },
        {
          "name": "maxTickets",
          "type": "u64"
        },
        {
          "name": "isAutomatic",
          "type": "bool"
        }
      ]
    },
    {
      "name": "createTable",
      "discriminator": [
        214,
        142,
        131,
        250,
        242,
        83,
        135,
        185
      ],
      "accounts": [
        {
          "name": "programState",
          "writable": true
        },
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "dealer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "title",
          "type": "string"
        },
        {
          "name": "maxBet",
          "type": "u64"
        },
        {
          "name": "minBet",
          "type": "u64"
        },
        {
          "name": "splitAllowed",
          "type": "bool"
        },
        {
          "name": "doubleAllowed",
          "type": "bool"
        },
        {
          "name": "dealerLiquidity",
          "type": "u64"
        },
        {
          "name": "maxPlayers",
          "type": "u8"
        },
        {
          "name": "timeBetweenHands",
          "type": "i64"
        },
        {
          "name": "timeBetweenActions",
          "type": "i64"
        },
        {
          "name": "inactivityTimeout",
          "type": "i64"
        },
        {
          "name": "tableFee",
          "type": "u8"
        },
        {
          "name": "huddleRoomId",
          "type": "string"
        },
        {
          "name": "numDecks",
          "type": "u8"
        },
        {
          "name": "deckPenetration",
          "type": "u8"
        }
      ]
    },
    {
      "name": "dealCards",
      "discriminator": [
        38,
        218,
        247,
        103,
        218,
        237,
        24,
        65
      ],
      "accounts": [
        {
          "name": "programState",
          "writable": true
        },
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "recentSlothashes",
          "address": "SysvarS1otHashes111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "clientEntropy",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        }
      ]
    },
    {
      "name": "dealerDeposit",
      "discriminator": [
        198,
        156,
        64,
        116,
        186,
        54,
        232,
        220
      ],
      "accounts": [
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "dealer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "dealerWithdraw",
      "discriminator": [
        30,
        205,
        65,
        222,
        35,
        109,
        51,
        200
      ],
      "accounts": [
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "dealer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "deposit",
      "discriminator": [
        242,
        35,
        198,
        137,
        82,
        225,
        242,
        182
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true
        },
        {
          "name": "player",
          "writable": true,
          "signer": true,
          "relations": [
            "playerWallet"
          ]
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "doubleDown",
      "discriminator": [
        224,
        14,
        59,
        137,
        102,
        107,
        222,
        109
      ],
      "accounts": [
        {
          "name": "programState",
          "writable": true
        },
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "playerWallet",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "recentSlothashes",
          "address": "SysvarS1otHashes111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "seatIndex",
          "type": "u8"
        },
        {
          "name": "clientEntropy",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        }
      ]
    },
    {
      "name": "emergencyCloseTable",
      "discriminator": [
        197,
        229,
        36,
        87,
        191,
        199,
        76,
        210
      ],
      "accounts": [
        {
          "name": "programState",
          "writable": true
        },
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "dealer",
          "writable": true
        },
        {
          "name": "siteAuthority",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "emergencyCloseWallet",
      "discriminator": [
        82,
        90,
        238,
        13,
        20,
        95,
        222,
        236
      ],
      "accounts": [
        {
          "name": "programState",
          "writable": true
        },
        {
          "name": "playerWallet",
          "writable": true
        },
        {
          "name": "player",
          "writable": true
        },
        {
          "name": "siteAuthority",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "hit",
      "discriminator": [
        237,
        225,
        125,
        176,
        242,
        47,
        187,
        184
      ],
      "accounts": [
        {
          "name": "programState",
          "writable": true
        },
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "playerWallet",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "recentSlothashes",
          "address": "SysvarS1otHashes111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "seatIndex",
          "type": "u8"
        },
        {
          "name": "clientEntropy",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        }
      ]
    },
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "programState",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  111,
                  103,
                  114,
                  97,
                  109,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "siteAuthority"
        },
        {
          "name": "deployer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initializePlayerWallet",
      "discriminator": [
        165,
        31,
        2,
        184,
        174,
        146,
        182,
        42
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  108,
                  97,
                  121,
                  101,
                  114,
                  95,
                  119,
                  97,
                  108,
                  108,
                  101,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "player"
              }
            ]
          }
        },
        {
          "name": "player",
          "writable": true,
          "signer": true
        },
        {
          "name": "siteAuthority"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "programState"
        }
      ],
      "args": []
    },
    {
      "name": "joinSeat",
      "discriminator": [
        163,
        253,
        32,
        120,
        247,
        157,
        181,
        142
      ],
      "accounts": [
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "playerWallet",
          "writable": true
        },
        {
          "name": "authority",
          "signer": true,
          "relations": [
            "playerWallet"
          ]
        },
        {
          "name": "programState"
        }
      ],
      "args": [
        {
          "name": "seatIndex",
          "type": "u8"
        }
      ]
    },
    {
      "name": "leaveSeat",
      "discriminator": [
        108,
        242,
        177,
        6,
        45,
        227,
        145,
        142
      ],
      "accounts": [
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "playerWallet",
          "writable": true
        },
        {
          "name": "authority",
          "signer": true,
          "relations": [
            "playerWallet"
          ]
        },
        {
          "name": "programState"
        }
      ],
      "args": [
        {
          "name": "seatIndex",
          "type": "u8"
        }
      ]
    },
    {
      "name": "pickWinner",
      "discriminator": [
        227,
        62,
        25,
        73,
        132,
        106,
        68,
        96
      ],
      "accounts": [
        {
          "name": "programState",
          "writable": true
        },
        {
          "name": "lottery",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  111,
                  116,
                  116,
                  101,
                  114,
                  121
                ]
              },
              {
                "kind": "arg",
                "path": "lotteryId"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        }
      ],
      "args": [
        {
          "name": "lotteryId",
          "type": "u64"
        }
      ]
    },
    {
      "name": "placeBet",
      "discriminator": [
        222,
        62,
        67,
        220,
        63,
        166,
        126,
        33
      ],
      "accounts": [
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "playerWallet",
          "writable": true
        },
        {
          "name": "authority",
          "signer": true,
          "relations": [
            "playerWallet"
          ]
        },
        {
          "name": "programState"
        }
      ],
      "args": [
        {
          "name": "seatIndex",
          "type": "u8"
        },
        {
          "name": "betAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "removeBet",
      "discriminator": [
        199,
        112,
        9,
        190,
        119,
        188,
        70,
        123
      ],
      "accounts": [
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "playerWallet",
          "writable": true
        },
        {
          "name": "authority",
          "signer": true,
          "relations": [
            "playerWallet"
          ]
        },
        {
          "name": "programState"
        }
      ],
      "args": [
        {
          "name": "seatIndex",
          "type": "u8"
        }
      ]
    },
    {
      "name": "stand",
      "discriminator": [
        215,
        119,
        10,
        61,
        97,
        252,
        126,
        243
      ],
      "accounts": [
        {
          "name": "programState",
          "writable": true
        },
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "playerWallet",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "recentSlothashes",
          "address": "SysvarS1otHashes111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "seatIndex",
          "type": "u8"
        },
        {
          "name": "clientEntropy",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        }
      ]
    },
    {
      "name": "updateTable",
      "discriminator": [
        224,
        23,
        10,
        48,
        181,
        73,
        121,
        187
      ],
      "accounts": [
        {
          "name": "table",
          "writable": true
        },
        {
          "name": "dealer",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "title",
          "type": "string"
        },
        {
          "name": "maxBet",
          "type": "u64"
        },
        {
          "name": "minBet",
          "type": "u64"
        },
        {
          "name": "maxPlayers",
          "type": "u8"
        }
      ]
    },
    {
      "name": "withdraw",
      "discriminator": [
        183,
        18,
        70,
        156,
        148,
        109,
        161,
        34
      ],
      "accounts": [
        {
          "name": "playerWallet",
          "writable": true
        },
        {
          "name": "player",
          "writable": true,
          "signer": true,
          "relations": [
            "playerWallet"
          ]
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "withdrawFees",
      "discriminator": [
        198,
        212,
        171,
        109,
        144,
        215,
        174,
        89
      ],
      "accounts": [
        {
          "name": "programState",
          "writable": true
        },
        {
          "name": "treasury",
          "writable": true
        },
        {
          "name": "siteAuthority",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "lottery",
      "discriminator": [
        162,
        182,
        26,
        12,
        164,
        214,
        112,
        3
      ]
    },
    {
      "name": "playerWallet",
      "discriminator": [
        70,
        208,
        240,
        254,
        222,
        47,
        4,
        36
      ]
    },
    {
      "name": "programState",
      "discriminator": [
        77,
        209,
        137,
        229,
        149,
        67,
        167,
        230
      ]
    },
    {
      "name": "table",
      "discriminator": [
        34,
        100,
        138,
        97,
        236,
        129,
        230,
        112
      ]
    },
    {
      "name": "ticket",
      "discriminator": [
        41,
        228,
        24,
        165,
        78,
        90,
        235,
        200
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "alreadyInitialized",
      "msg": "Already initialized"
    },
    {
      "code": 6001,
      "name": "lotteryAlreadyExists",
      "msg": "Lottery already exists"
    },
    {
      "code": 6002,
      "name": "winnerAlreadyExists",
      "msg": "Winner already exists"
    },
    {
      "code": 6003,
      "name": "noTicketsPurchased",
      "msg": "No tickets purchased"
    },
    {
      "code": 6004,
      "name": "invalidWinner",
      "msg": "Invalid winner"
    },
    {
      "code": 6005,
      "name": "alreadyClaimed",
      "msg": "Already claimed"
    },
    {
      "code": 6006,
      "name": "lotteryNotFound",
      "msg": "Lottery not found"
    },
    {
      "code": 6007,
      "name": "ticketNotFound",
      "msg": "Ticket not found"
    },
    {
      "code": 6008,
      "name": "lotteryAlreadyClaimed",
      "msg": "Lottery already claimed"
    },
    {
      "code": 6009,
      "name": "invalidLotteryId",
      "msg": "Invalid lottery id"
    },
    {
      "code": 6010,
      "name": "noWinner",
      "msg": "No winner"
    },
    {
      "code": 6011,
      "name": "invalidTicketId",
      "msg": "Invalid ticket id"
    },
    {
      "code": 6012,
      "name": "invalidTicketAmount",
      "msg": "Invalid ticket amount"
    },
    {
      "code": 6013,
      "name": "titleTooLong",
      "msg": "Title too long"
    },
    {
      "code": 6014,
      "name": "invalidTimeLimit",
      "msg": "Invalid time limit"
    },
    {
      "code": 6015,
      "name": "invalidMaxTickets",
      "msg": "Invalid max tickets"
    },
    {
      "code": 6016,
      "name": "invalidLotteryParameters",
      "msg": "Invalid lottery parameters"
    },
    {
      "code": 6017,
      "name": "tableNotFound",
      "msg": "Table not found"
    },
    {
      "code": 6018,
      "name": "insufficientLiquidity",
      "msg": "Insufficient liquidity"
    },
    {
      "code": 6019,
      "name": "insufficientFunds",
      "msg": "Insufficient funds"
    },
    {
      "code": 6020,
      "name": "invalidDealer",
      "msg": "Invalid dealer"
    },
    {
      "code": 6021,
      "name": "invalidBetAmount",
      "msg": "Invalid bet amount"
    },
    {
      "code": 6022,
      "name": "playerAlreadyBet",
      "msg": "Player already bet"
    },
    {
      "code": 6023,
      "name": "invalidGameState",
      "msg": "Invalid game state"
    },
    {
      "code": 6024,
      "name": "playerAlreadyJoined",
      "msg": "Player already joined"
    },
    {
      "code": 6025,
      "name": "invalidPlayer",
      "msg": "Invalid player"
    },
    {
      "code": 6026,
      "name": "activeBetExists",
      "msg": "Active bet exists"
    },
    {
      "code": 6027,
      "name": "playerNotJoined",
      "msg": "Player not joined"
    },
    {
      "code": 6028,
      "name": "noBet",
      "msg": "No bet"
    },
    {
      "code": 6029,
      "name": "invalidAuthority",
      "msg": "Invalid authority"
    },
    {
      "code": 6030,
      "name": "playerInAction",
      "msg": "Player in action"
    },
    {
      "code": 6031,
      "name": "playerHandNotSet",
      "msg": "Player hand not set"
    },
    {
      "code": 6032,
      "name": "dealerHandNotSet",
      "msg": "Dealer hand not set"
    },
    {
      "code": 6033,
      "name": "playerNotInAction",
      "msg": "Player not in action"
    },
    {
      "code": 6034,
      "name": "dealerNotInAction",
      "msg": "Dealer not in action"
    },
    {
      "code": 6035,
      "name": "noHandFound",
      "msg": "No hand found"
    },
    {
      "code": 6036,
      "name": "gameInProgress",
      "msg": "Game in progress"
    },
    {
      "code": 6037,
      "name": "doubleDownNotAllowed",
      "msg": "Double down not allowed"
    },
    {
      "code": 6038,
      "name": "invalidDoubleDown",
      "msg": "Invalid double down"
    },
    {
      "code": 6039,
      "name": "tableFull",
      "msg": "Table is full"
    },
    {
      "code": 6040,
      "name": "unauthorized",
      "msg": "unauthorized"
    },
    {
      "code": 6041,
      "name": "invalidSeatIndex",
      "msg": "Invalid seat index"
    },
    {
      "code": 6042,
      "name": "seatTaken",
      "msg": "Seat is already taken"
    },
    {
      "code": 6043,
      "name": "noActivePlayers",
      "msg": "No active players"
    },
    {
      "code": 6044,
      "name": "notYourTurn",
      "msg": "Not your turn"
    },
    {
      "code": 6045,
      "name": "notInAction",
      "msg": "Not in action"
    },
    {
      "code": 6046,
      "name": "noBetsPlaced",
      "msg": "No bets placed"
    },
    {
      "code": 6047,
      "name": "playerNotFound",
      "msg": "Player not found"
    },
    {
      "code": 6048,
      "name": "outOfCards",
      "msg": "Out of cards"
    },
    {
      "code": 6049,
      "name": "mathOverflow",
      "msg": "Math overflow"
    },
    {
      "code": 6050,
      "name": "invalidRemainingAccounts",
      "msg": "Invalid remaining accounts"
    },
    {
      "code": 6051,
      "name": "invalidTableBalance",
      "msg": "Invalid table balance"
    },
    {
      "code": 6052,
      "name": "invalidPrizePoolPayout",
      "msg": "Invalid prize pool payout"
    },
    {
      "code": 6053,
      "name": "seatOccupied",
      "msg": "Seat occupied"
    },
    {
      "code": 6054,
      "name": "insufficientSeats",
      "msg": "Insufficient seats"
    },
    {
      "code": 6055,
      "name": "invalidNumDecks",
      "msg": "Invalid num decks"
    },
    {
      "code": 6056,
      "name": "invalidDeckPenetration",
      "msg": "Invalid deck penetration"
    },
    {
      "code": 6057,
      "name": "tooManyVerifiedPrograms",
      "msg": "Too many verified programs"
    }
  ],
  "types": [
    {
      "name": "card",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rank",
            "type": "u8"
          },
          {
            "name": "suit",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "gameResult",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "won"
          },
          {
            "name": "lost"
          },
          {
            "name": "push"
          }
        ]
      }
    },
    {
      "name": "hand",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "cards",
            "type": {
              "vec": {
                "defined": {
                  "name": "card"
                }
              }
            }
          },
          {
            "name": "isSoft",
            "type": "bool"
          },
          {
            "name": "value",
            "type": "u8"
          },
          {
            "name": "isBlackjack",
            "type": "bool"
          },
          {
            "name": "isBust",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "lottery",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "ticketPrice",
            "type": "u64"
          },
          {
            "name": "lastTicketId",
            "type": "u64"
          },
          {
            "name": "winnerSet",
            "type": "u32"
          },
          {
            "name": "winnerId",
            "type": {
              "option": "u64"
            }
          },
          {
            "name": "claimed",
            "type": "bool"
          },
          {
            "name": "claimedBy",
            "type": "pubkey"
          },
          {
            "name": "title",
            "type": "string"
          },
          {
            "name": "timeLimit",
            "type": "u64"
          },
          {
            "name": "maxTickets",
            "type": "u64"
          },
          {
            "name": "isAutomatic",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "playerHandResult",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "playerHand",
            "type": {
              "defined": {
                "name": "hand"
              }
            }
          },
          {
            "name": "betAmount",
            "type": "u64"
          },
          {
            "name": "payoutAmount",
            "type": "u64"
          },
          {
            "name": "handResult",
            "type": {
              "defined": {
                "name": "gameResult"
              }
            }
          }
        ]
      }
    },
    {
      "name": "playerWallet",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "player",
            "type": "pubkey"
          },
          {
            "name": "balance",
            "type": "u64"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "programState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "initialized",
            "type": "bool"
          },
          {
            "name": "tableCount",
            "type": "u64"
          },
          {
            "name": "playerCount",
            "type": "u64"
          },
          {
            "name": "platformFee",
            "type": "u64"
          },
          {
            "name": "feesCollected",
            "type": "u64"
          },
          {
            "name": "platformAddress",
            "type": "pubkey"
          },
          {
            "name": "siteAuthority",
            "type": "pubkey"
          },
          {
            "name": "lotteryCount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "seat",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "player",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "playerWallet",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "bet",
            "type": "u64"
          },
          {
            "name": "inAction",
            "type": "bool"
          },
          {
            "name": "hand",
            "type": {
              "option": {
                "defined": {
                  "name": "hand"
                }
              }
            }
          },
          {
            "name": "isTurn",
            "type": "bool"
          },
          {
            "name": "previousHand",
            "type": {
              "option": {
                "defined": {
                  "name": "playerHandResult"
                }
              }
            }
          },
          {
            "name": "lastActionTimestamp",
            "type": "i64"
          },
          {
            "name": "lastHandPlayedTimestamp",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "table",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "dealer",
            "type": "pubkey"
          },
          {
            "name": "dealerLiquidity",
            "type": "u64"
          },
          {
            "name": "minBet",
            "type": "u64"
          },
          {
            "name": "maxBet",
            "type": "u64"
          },
          {
            "name": "totalHands",
            "type": "u64"
          },
          {
            "name": "title",
            "type": "string"
          },
          {
            "name": "timestamp",
            "type": "u64"
          },
          {
            "name": "usedCardIndexes",
            "type": {
              "vec": "bool"
            }
          },
          {
            "name": "deck",
            "type": {
              "vec": {
                "defined": {
                  "name": "card"
                }
              }
            }
          },
          {
            "name": "numDecks",
            "type": "u8"
          },
          {
            "name": "deckPenetration",
            "type": "u8"
          },
          {
            "name": "deckSeed",
            "type": "u64"
          },
          {
            "name": "active",
            "type": "bool"
          },
          {
            "name": "tableStatus",
            "type": "u8"
          },
          {
            "name": "tableFee",
            "type": "u8"
          },
          {
            "name": "huddleRoomId",
            "type": "string"
          },
          {
            "name": "hasPlayers",
            "type": "bool"
          },
          {
            "name": "seats",
            "type": {
              "vec": {
                "defined": {
                  "name": "seat"
                }
              }
            }
          },
          {
            "name": "currentTurnIndex",
            "type": {
              "option": "u8"
            }
          },
          {
            "name": "dealerHand",
            "type": {
              "option": {
                "defined": {
                  "name": "hand"
                }
              }
            }
          },
          {
            "name": "gameStatus",
            "type": "u8"
          },
          {
            "name": "previousDealerHand",
            "type": {
              "option": {
                "defined": {
                  "name": "hand"
                }
              }
            }
          },
          {
            "name": "splitAllowed",
            "type": "bool"
          },
          {
            "name": "doubleAllowed",
            "type": "bool"
          },
          {
            "name": "maxPlayers",
            "type": "u8"
          },
          {
            "name": "lastHandTimestamp",
            "type": "i64"
          },
          {
            "name": "showHandTimestamp",
            "type": "i64"
          },
          {
            "name": "timeBetweenHands",
            "type": "i64"
          },
          {
            "name": "timeBetweenActions",
            "type": "i64"
          },
          {
            "name": "inactivePlayerTimeout",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "ticket",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "size",
            "type": "u64"
          },
          {
            "name": "buyer",
            "type": "pubkey"
          },
          {
            "name": "lotteryId",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
