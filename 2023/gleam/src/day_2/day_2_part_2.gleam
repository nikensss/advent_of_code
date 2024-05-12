import day_2/game
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
  |> list.filter(fn(str) { str != "" })
  |> list.map(game.from_string)
  |> list.map(game.power)
  |> list.fold(0, int.add)
}
