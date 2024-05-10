import day_1/day_1_part_1.{part_1 as d1p1}
import day_1/day_1_part_2.{part_2 as d1p2}
import day_2/day_2_part_1.{part_1 as d2p1}
import gleeunit
import gleeunit/should

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn day_1_part_1_with_test_input_test() {
  "td1p1.txt"
  |> d1p1
  |> should.equal(142)
}

pub fn day_1_part_1_with_real_input_test() {
  "d1.txt"
  |> d1p1
  |> should.equal(54_667)
}

pub fn day_1_part_2_with_test_input_test() {
  "td1p2.txt"
  |> d1p2
  |> should.equal(281)
}

pub fn day_1_part_2_with_real_input_test() {
  "d1.txt"
  |> d1p2
  |> should.equal(54_203)
}

pub fn day_2_part_1_with_test_input_test() {
  "td2p1.txt"
  |> d2p1
  |> should.equal(8)
}

pub fn day_2_part_1_with_complete_input_test() {
  "d2.txt"
  |> d2p1
  |> should.equal(2239)
}
