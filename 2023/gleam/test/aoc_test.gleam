import gleeunit
import gleeunit/should
import lib/day_1

pub fn main() {
  gleeunit.main()
}

// gleeunit test functions end in `_test`
pub fn part_1_with_test_input_test() {
  "test-input-01.txt"
  |> day_1.part_1
  |> should.equal(142)
}

pub fn part_1_with_real_input_test() {
  "input.txt"
  |> day_1.part_1
  |> should.equal(54_667)
}

pub fn part_2_with_test_input_test() {
  "test-input-02.txt"
  |> day_1.part_2
  |> should.equal(281)
}

pub fn part_2_with_real_input_test() {
  "input.txt"
  |> day_1.part_2
  |> should.equal(54_203)
}
