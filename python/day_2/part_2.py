from typing import List

import pandas as pd


def read_input_file(path: str) -> List[dict]:
    with open(path) as f:
        lines = f.readlines()
    return [
        {
            "action": line.split(" ")[0],
            "value": int(line.split(" ")[1].replace("\n", "")),
        }
        for line in lines
    ]


def calculate_final_position(moves: List[dict]) -> dict:
    final_positions = {"forward": 0, "depth": 0, "aim": 0}
    for move in moves:
        if move["action"] == "forward":
            final_positions["forward"] += move["value"]
            final_positions["depth"] += move["value"] * final_positions["aim"]
        if move["action"] == "down":
            final_positions["aim"] += move["value"]
        if move["action"] == "up":
            final_positions["aim"] -= move["value"]
    return final_positions


if __name__ == "__main__":
    moves = read_input_file("day_2/input.txt")
    final_positions = calculate_final_position(moves)
    print(f"result: {final_positions['forward'] * final_positions['depth']}")
