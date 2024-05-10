import day_2/game
import gleam/int
import gleam/list
import gleam/result
import gleam/string
import simplifile

pub fn part_1(file_name: String) -> Int {
  file_name
  |> simplifile.read
  |> result.unwrap(or: "")
  |> string.split(on: "\n")
  |> list.filter(fn(str) { str != "" })
  |> list.map(game.from_string)
  |> list.filter(game.is_possible)
  |> list.map(fn(game) { game.index })
  |> list.fold(0, int.add)
}
