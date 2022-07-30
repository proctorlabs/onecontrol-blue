from .encoding.crc8 import calculate

result = calculate(b"Hello23!")
print(f"CRC8 is {hex(result)}")
