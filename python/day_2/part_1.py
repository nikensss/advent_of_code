from typing import List

import pandas as pd


def read_input_file(path:str) -> List[dict]:
  with open(path) as f:
    lines = f.readlines()
  return [{'action': line.split(' ')[0],'value':int(line.split(' ')[1].replace('\n','')) } for line in lines]
  

if __name__ == '__main__':
  lines = read_input_file('day_2/input.txt')
  df = pd.DataFrame(lines)
  total_move_by_action = df.groupby('action').sum()
  total_y = total_move_by_action.loc['down'] - total_move_by_action.loc['up']
  total_x = total_move_by_action.loc['forward']
  print(f"result: {total_x.values[0] * total_y.values[0]}")
