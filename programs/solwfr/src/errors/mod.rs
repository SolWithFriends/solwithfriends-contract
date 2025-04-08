use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Already initialized")]
    AlreadyInitialized,
    //Lottery
    #[msg("Lottery already exists")]
    LotteryAlreadyExists,

    #[msg("Winner already exists")]
    WinnerAlreadyExists,

    #[msg("No tickets purchased")]
    NoTicketsPurchased,

    #[msg("Invalid winner")]
    InvalidWinner,

    #[msg("Already claimed")]
    AlreadyClaimed,

    #[msg("Lottery not found")]
    LotteryNotFound,

    #[msg("Ticket not found")]
    TicketNotFound,

    #[msg("Lottery already claimed")]
    LotteryAlreadyClaimed,

    #[msg("Invalid lottery id")]
    InvalidLotteryId,

    #[msg("No winner")]
    NoWinner,

    #[msg("Invalid ticket id")]
    InvalidTicketId,

    #[msg("Invalid ticket amount")]
    InvalidTicketAmount,

    #[msg("Title too long")]
    TitleTooLong,

    #[msg("Invalid time limit")]
    InvalidTimeLimit,

    #[msg("Invalid max tickets")]
    InvalidMaxTickets,

    #[msg("Invalid lottery parameters")]
    InvalidLotteryParameters,

    #[msg("Table not found")]
    TableNotFound,

    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Invalid dealer")]
    InvalidDealer,

    #[msg("Invalid bet amount")]
    InvalidBetAmount,

    #[msg("Player already bet")]
    PlayerAlreadyBet,

    #[msg("Invalid game state")]
    InvalidGameState,

    #[msg("Player already joined")]
    PlayerAlreadyJoined,

    #[msg("Invalid player")]
    InvalidPlayer,

    #[msg("Active bet exists")]
    ActiveBetExists,

    #[msg("Player not joined")]
    PlayerNotJoined,

    #[msg("No bet")]
    NoBet,

    #[msg("Invalid authority")]
    InvalidAuthority,

    #[msg("Player in action")]
    PlayerInAction,

    #[msg("Player hand not set")]
    PlayerHandNotSet,

    #[msg("Dealer hand not set")]
    DealerHandNotSet,

    #[msg("Player not in action")]
    PlayerNotInAction,

    #[msg("Dealer not in action")]
    DealerNotInAction,
    
    #[msg("No hand found")]
    NoHandFound,

    #[msg("Game in progress")]
    GameInProgress,

    #[msg("Double down not allowed")]
    DoubleDownNotAllowed,

    #[msg("Invalid double down")]
    InvalidDoubleDown,

    #[msg("Table is full")]
    TableFull,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid seat index")]
    InvalidSeatIndex,

    #[msg("Seat is already taken")]
    SeatTaken,

    #[msg("No active players")]
    NoActivePlayers,

    #[msg("Not your turn")]
    NotYourTurn,

    #[msg("Not in action")]
    NotInAction,

    #[msg("No bets placed")]
    NoBetsPlaced,

    #[msg("Player not found")]
    PlayerNotFound,

    #[msg("Out of cards")]
    OutOfCards,

    #[msg("Math overflow")]
    MathOverflow,

    #[msg("Invalid remaining accounts")]
    InvalidRemainingAccounts,

    #[msg("Invalid table balance")]
    InvalidTableBalance,

    #[msg("Invalid prize pool payout")]
    InvalidPrizePoolPayout,

    #[msg("Seat occupied")]
    SeatOccupied,

    #[msg("Insufficient seats")]
    InsufficientSeats,

    #[msg("Invalid num decks")]
    InvalidNumDecks,

    #[msg("Invalid deck penetration")]
    InvalidDeckPenetration,

    #[msg("Too many verified programs")]
    TooManyVerifiedPrograms,
    
}