import day_2/cube.{type Cube, Cube}
import gleam/dict
import gleam/int
import gleam/list
import gleam/result
import gleam/string

pub type Hand {
  Hand(red: Cube, green: Cube, blue: Cube)
}

pub fn from_string(input: String) -> Hand {
  let cubes =
    input
    |> string.split(on: ", ")
    |> list.map(string.split(_, on: " "))
    |> list.map(to_tuple)
    |> dict.from_list

  let assert [red, green, blue] = [
    dict.get(cubes, "red")
      |> result.unwrap(0),
    dict.get(cubes, "green")
      |> result.unwrap(0),
    dict.get(cubes, "blue")
      |> result.unwrap(0),
  ]

  Hand(red: Cube(red), green: Cube(green), blue: Cube(blue))
}

fn to_tuple(input: List(String)) -> #(String, Int) {
  case input {
    [amount, color] -> tuplify(amount, color)
    _ -> panic("error in input: ")
  }
}

fn tuplify(amount: String, color: String) -> #(String, Int) {
  case
    amount
    |> int.parse
  {
    Ok(amount) -> #(color, amount)
    _ -> panic("error in input: ")
  }
}

pub fn is_possible(hand: Hand) -> Bool {
  let assert [red, green, blue] = [
    hand.red.amount,
    hand.green.amount,
    hand.blue.amount,
  ]

  red <= 12 && green <= 13 && blue <= 14
}
