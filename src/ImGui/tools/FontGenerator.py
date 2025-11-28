import binascii
import datetime
import os

os.system("clear")
print("This Python script supports TTF and OTF fonts")
file_path = input("Enter Font Path: ")

with open(file_path, "rb") as f:
    data = f.read()
    hex_data = ", ".join(["0x" + b.to_bytes(1, byteorder='big').hex() for b in data])
    FONT_DATA_SIZE = len(hex_data)
    custom_data = f"#pragma once\n#include <cstdint>\nconst std::uint8_t Custom[{FONT_DATA_SIZE}]\n{{\n{hex_data}\n}};"
    with open("Font.h", "w") as fw:
        fw.write(custom_data)

print("Done")
