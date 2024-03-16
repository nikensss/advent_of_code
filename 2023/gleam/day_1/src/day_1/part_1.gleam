import gleam/result
import gleam/string
import gleam/list
import gleam/int
import simplifile

pub fn part_1(file_name: String) -> Int {
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

  let last_digit =
    line
    |> string.reverse
    |> string.to_graphemes
    |> get_first_digit

  first_digit
  |> string.append(last_digit)
  |> int.parse
}

fn get_first_digit(chars: List(String)) -> String {
  chars
  |> list.find(is_digit)
  |> result.unwrap("")
}

fn is_digit(char: String) -> Bool {
  char
  |> int.parse
  |> result.is_ok
}
