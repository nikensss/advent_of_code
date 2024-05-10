import gleam/int
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn part_2(file_name: String) -> Int {
  file_name
  |> simplifile.read
  |> result.unwrap(or: "")
  |> string.split(on: "\n")
  |> list.filter_map(to_digit)
  |> list.fold(0, int.add)
}

fn to_digit(line: String) -> Result(Int, Nil) {
  let first_digit =
    line
    |> string.to_graphemes
    |> get_first_digit
    |> result.unwrap(or: "0")

  let last_digit =
    line
    |> string.reverse
    |> string.to_graphemes
    |> get_last_digit
    |> result.unwrap(or: "0")

  first_digit
  |> string.append(last_digit)
  |> int.parse
}

fn get_first_digit(chars: List(String)) -> Result(String, Nil) {
  case chars {
    ["1", ..] | ["o", "n", "e", ..] -> Ok("1")
    ["2", ..] | ["t", "w", "o", ..] -> Ok("2")
    ["3", ..] | ["t", "h", "r", "e", "e", ..] -> Ok("3")
    ["4", ..] | ["f", "o", "u", "r", ..] -> Ok("4")
    ["5", ..] | ["f", "i", "v", "e", ..] -> Ok("5")
    ["6", ..] | ["s", "i", "x", ..] -> Ok("6")
    ["7", ..] | ["s", "e", "v", "e", "n", ..] -> Ok("7")
    ["8", ..] | ["e", "i", "g", "h", "t", ..] -> Ok("8")
    ["9", ..] | ["n", "i", "n", "e", ..] -> Ok("9")
    [] -> Error(Nil)
    [_, ..rest] -> get_first_digit(rest)
  }
}

fn get_last_digit(chars: List(String)) -> Result(String, Nil) {
  case chars {
    ["1", ..] | ["e", "n", "o", ..] -> Ok("1")
    ["2", ..] | ["o", "w", "t", ..] -> Ok("2")
    ["3", ..] | ["e", "e", "r", "h", "t", ..] -> Ok("3")
    ["4", ..] | ["r", "u", "o", "f", ..] -> Ok("4")
    ["5", ..] | ["e", "v", "i", "f", ..] -> Ok("5")
    ["6", ..] | ["x", "i", "s", ..] -> Ok("6")
    ["7", ..] | ["n", "e", "v", "e", "s", ..] -> Ok("7")
    ["8", ..] | ["t", "h", "g", "i", "e", ..] -> Ok("8")
    ["9", ..] | ["e", "n", "i", "n", ..] -> Ok("9")
    [] -> Error(Nil)
    [_, ..rest] -> get_last_digit(rest)
  }
}
