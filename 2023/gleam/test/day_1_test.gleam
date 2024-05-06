import gleeunit
import gleeunit/should
import day_1/part_1

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn part_1_with_test_input_test() {
  "test-input-01.txt"
  |> part_1.part_1
  |> should.equal(142)
}

pub fn part_1_with_real_input_test() {
  "input.txt"
  |> part_1.part_1
  |> should.equal(54_667)
}
