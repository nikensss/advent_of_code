import day_1/day_1_part_1.{part_1 as d1p1}
import day_1/day_1_part_2.{part_2 as d1p2}
import day_2/day_2_part_1.{part_1 as d2p1}
import day_2/day_2_part_2.{part_2 as d2p2}
import gleeunit
import gleeunit/should

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn day_1_part_1_with_test_input_test() {
  "src/day_1/test_1.txt"
  |> d1p1
  |> should.equal(142)
}

pub fn day_1_part_1_with_real_input_test() {
  "src/day_1/input.txt"
  |> d1p1
  |> should.equal(54_667)
}

pub fn day_1_part_2_with_test_input_test() {
  "src/day_1/test_2.txt"
  |> d1p2
  |> should.equal(281)
}

pub fn day_1_part_2_with_real_input_test() {
  "src/day_1/input.txt"
  |> d1p2
  |> should.equal(54_203)
}

pub fn day_2_part_1_with_test_input_test() {
  "src/day_2/test_1.txt"
  |> d2p1
  |> should.equal(8)
}

pub fn day_2_part_1_with_complete_input_test() {
  "src/day_2/input.txt"
  |> d2p1
  |> should.equal(2239)
}

pub fn day_2_part_2_with_test_input_test() {
  "src/day_2/test_1.txt"
  |> d2p2
  |> should.equal(2286)
}

pub fn day_2_part_2_with_complete_input_test() {
  "src/day_2/input.txt"
  |> d2p2
  |> should.equal(83_435)
}
