import day_2/part_1.{part_1 as d2p1}
import gleam/io
import gleam/list

pub fn main() {
  "td2p1.txt"
  |> d2p1
  |> list.each(io.debug)
}
