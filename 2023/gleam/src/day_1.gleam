import gleam/io
import day_1/part_1

pub fn main() {
  "test-input-01.txt"
  |> part_1.part_1
  |> io.debug
}
