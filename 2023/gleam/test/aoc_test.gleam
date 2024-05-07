import gleeunit
import gleeunit/should
import lib/day_1/part_1.{part_1 as day_1_part_1}
import lib/day_1/part_2.{part_2 as day_1_part_2}

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn day_1_part_1_with_test_input_test() {
  "td1p1.txt"
  |> day_1_part_1
  |> should.equal(142)
}

pub fn day_1_part_1_with_real_input_test() {
  "d1.txt"
  |> day_1_part_1
  |> should.equal(54_667)
}

pub fn day_1_part_2_with_test_input_test() {
  "td1p2.txt"
  |> day_1_part_2
  |> should.equal(281)
}

pub fn day_1_part_2_with_real_input_test() {
  "d1.txt"
  |> day_1_part_2
  |> should.equal(54_203)
}
