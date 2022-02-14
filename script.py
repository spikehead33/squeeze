import json


with open("c.json", "r") as f:
    c = json.load(f)

with open("d.json", "r") as f:
    u = json.load(f)

for key, value in c.items():
    print(f"Equal?: {value == u[key]} {key}: compression: {value}, uncompression: {u[key]}")

