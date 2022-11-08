from typing import List

import pandas as pd


def read_input_file(path: str) -> List[int]:
  with open(path,'r') as f:
    lines = f.readlines()
  lines = [int(line.replace('\n','')) for line in lines]
  # lines = list(map(lambda x: int(x.replace('\n','')),lines))
  return lines


if __name__ == '__main__':
  lines = read_input_file('./day_1/input.txt')
  df = pd.DataFrame({'input': lines})
  has_increased = df.rolling(3).sum().diff().dropna().apply(lambda x: x > 0)
  print(str(has_increased[has_increased == True].dropna().count().values[0]))
