import fileinput
import functools

GRID_SIZE = 300


@functools.lru_cache(maxsize=None)
def power_level(x, y, serial_number, block_size):
  if block_size == 1:
    value = (((x + 10) * y + serial_number) * (x + 10)) // 100 % 10 - 5
  else:
    if block_size % 2:
      # Add the outer rim for odd grid sizes
      value = sum(
          power_level(x + block_size - 1, y + i, serial_number, 1)
          for i in range(block_size))
      value += sum(
          power_level(x + i, y + block_size - 1, serial_number, 1)
          for i in range(block_size - 1))
      value += power_level(x, y, serial_number, block_size - 1)
    else:
      half = block_size // 2
      value = (
          power_level(x, y, serial_number, half) + power_level(
              x + half, y, serial_number, half) + power_level(
                  x, y + half, serial_number, half) + power_level(
                      x + half, y + half, serial_number, half))

  return value


def best_block_of_size(serial_number, block_size):
  best_value = -5 * GRID_SIZE**2
  best_location = None
  for y in range(GRID_SIZE - block_size):
    for x in range(GRID_SIZE - block_size):
      value = power_level(x + 1, y + 1, serial_number, block_size)
      if value > best_value:
        best_value = value
        best_location = (x + 1, y + 1)

  return best_value, best_location


def p1(serial_number):
  return best_block_of_size(serial_number, 3)[1]


def p2(serial_number):
  best_value = -5 * GRID_SIZE**2
  best_location = None
  best_block_size = None
  for block_size in range(1, GRID_SIZE + 1):
    value, location = best_block_of_size(serial_number, block_size)
    if value > best_value:
      best_value = value
      best_location = location
      best_block_size = block_size

  return (*best_location, best_block_size)


if __name__ == "__main__":
  serial_number = int(fileinput.input()[0])
  print("%s,%s" % p1(serial_number))
  print("%s,%s,%s" % p2(serial_number))
