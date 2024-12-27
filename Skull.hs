-- Author: Naitik Mundra (c) 2024
-- This is pretty much all the functions you need on the types to create a fully functional game of Skulls and Roses. The only thing missing really is a gameplay loop that reveals cards and updates wins, but that is not a part of the "backend" functionality.

module Main where

import Data.Data
import GHC.TypeLits

-- | Type for cards, which can either be Skull or Rose.
data Card = Skull | Rose
  deriving (Show, Eq)

-- | Type for player with a unique ID and their hand.
data Player = Player {playerId :: Int, hand :: [Card], wins :: Int}
  deriving (Show, Eq)

-- | Type for cards on the table, associated with a player.
data TableCard = TableCard {ownerId :: Int, card :: Card}
  deriving (Show, Eq)

-- | A game state parameterised by the number of players.
data GameState (n :: Nat) where
  GameState :: {players :: [Player], table :: [TableCard]} -> GameState n

-- | Create a player with a given ID and an empty hand.
createPlayer :: Int -> Player
createPlayer pid = Player{playerId = pid, hand = [Rose, Rose, Skull], wins = 0}

-- | Create a new game state with a specific number of players.
createGameState :: forall n. (KnownNat n) => Proxy n -> GameState n
createGameState _ =
  GameState
    { players =
        map
          createPlayer
          [1 .. fromIntegral (natVal (Proxy :: Proxy n))]
    , table = []
    }

-- | Function to place one card from each player onto the table.
placeCards :: [Card] -> GameState n -> GameState n
placeCards cards (GameState players table) =
  GameState
    { players = zipWith removeCardFromPlayer players cards
    , table = table ++ zipWith createTableCard players cards
    }
 where
  removeCardFromPlayer :: Player -> Card -> Player
  removeCardFromPlayer player card = player{hand = removeCard card (hand player)}

  createTableCard :: Player -> Card -> TableCard
  createTableCard player card = TableCard{ownerId = playerId player, card = card}

-- | Helper function to remove a card from a player's hand.
removeCard :: Card -> [Card] -> [Card]
removeCard card hand = case span (/= card) hand of
  (before, _ : after) -> before ++ after
  (before, []) -> before -- If the card isn't found, return the hand unchanged.

-- Making a bid is a game play thng, but we need a helper function to make sure it is done in order, and that the maximum number of cards requirement is met. So, the bid has the properties: the player who made it must have kept that many cards on the table and it must be strictly higher than the last bid. Since this returns only the new bid, we must make sure that gameplay loop handles showing error condition to players if new bid is not higher than last bid.
validBid :: Int -> [TableCard] -> Player -> Bool
validBid lastBid table player = lastBid + 1 <= countOwnerId table (playerId player)

makeBid :: Int -> [TableCard] -> Player -> Int
makeBid lastBid table player = lastBid + fromEnum (validBid lastBid table player)

countOwnerId :: [TableCard] -> Int -> Int
countOwnerId table playerId = length (filter (\tc -> ownerId tc == playerId) table)

-- | Main module to demonstrate the game state updates.
main :: IO ()
main = do
  let game = createGameState (Proxy :: Proxy 3)
  putStrLn "Game state start:"
  print (players game)
  let updatedGame = placeCards [Rose, Skull, Rose] game
  putStrLn "Game state after placing one card each:"
  print (players updatedGame)
  putStrLn "Cards on the table:"
  print (table updatedGame)
  print $ countOwnerId (table updatedGame) 1
  print $ makeBid 1 (table updatedGame) (players game !! 1)
