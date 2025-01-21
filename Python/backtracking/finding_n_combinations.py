#!/usr/bin/env python3

def print_vector(arr):
  index = 0
  print("\n")
  for item in arr:
    print(f'{arr[index]}')
    index += 1
  print("\n")


def combosN(vector_input, N, vector_result, ind):
  if ind >= len(vector_input):
    return
  if len(vector_result) == 0 and ind > len(vector_input) - N:
    return
  for item in vector_input:
    index = 0
    vector_result.append(vector_input[index-1])
    if len(vector_result) == N:
      print_vector(vector_result)
    else:
      combosN(vector_input, N, vector_result, item+1)
    vector_result.pop()


if __name__ == '__main__':
  vector_input = [1, 2, 3, 4, 5]
  vector_result = []
  combosN(vector_input, 3, vector_result, 0)
