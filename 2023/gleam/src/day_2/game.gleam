import day_2/hand.{type Hand, Hand}
import gleam/int
import gleam/list
import gleam/string

pub type Game {
  Game(index: Int, hands: List(Hand))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
pub fn from_string(input: String) -> Game {
  let assert [game_and_index, hands] =
    input
    |> string.split(on: ": ")

  let assert [_, index] =
    game_and_index
    |> string.split(on: " ")

  let index =
    index
    |> int.parse

  let index = case index {
    Ok(value) -> value
    Error(_) -> panic
  }

  let hands =
    hands
    |> string.split(on: "; ")
    |> list.map(hand.from_string)

  Game(index, hands)
}

pub fn is_possible(game: Game) -> Bool {
  game.hands
  |> list.all(hand.is_possible)
}

pub fn power(game: Game) -> Int {
  game.hands
  |> hand.power
}
