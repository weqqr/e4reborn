.ram:
# name (address) value
  total(0x0000) 0
  i    (0x0001) 0x0003
  len  (0x0002) 5
  array(0x0003) [1, 2, 3, 4, 5]

# Command (16 bits)
#
# A-TYPE:
#  OPCODE | ADDRESS (12 bits)
# --------+------------------------
# 0 0 0 0 | 1 0 1 1 1 0 0 1 0 1 1 1
#
# C-TYPE:
#  OPCODE | CONSTANT (12 bits)
# --------+------------------------
# 0 0 0 0 | 0 0 0 0 1 1 1 0 1 0 1 0

# One register: ACC
# name  | description                  | TYPE | OPCODE
# ------+------------------------------+------+----
# load  | Load value from RAM to ACC   | A    | 0000
# store | Store value from ACC to RAM  | A    | 0001
# loadc | Load constant value to ACC   | C    | 0010
# storec| Store constant value to RAM  | C    | 0011
# add   | Add value from RAM to ACC    | A    | 0100
# addc  | Add constant value to ACC    | C    | 0101
# jz    | Jump if acc is zero          | A    | 0110
# jmp   | Jump unconditionally         | A    | 0111
# halt  | Halt                         | -    | 1000

addition:
  # Add value from array to total
  load  total
  add   i
  store total

  # Increment counter
  load  i
  addc  1
  store i

  # Check if the loop should continue
  load  i
  sub   len
  jz    addition

  # Break the loop
  jmp   stop

stop:
  halt


