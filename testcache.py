#! /usr/bin/python3

import http.client
import os
import re
import shutil

import json5

BASE_DIR = "test-www"
GAME_DIR = "game"
IMG_DIR = "screenshot"
IMG_REGEX = re.compile(f'<img src="/({IMG_DIR}/[^"]*)" ')
LINKS = ["index.html", "pkg"]
ASSETS = "assets"
CFG = "main.cfg.json5"


def download(resource: str) -> bytes:
    print(f"GET {resource}")
    conn = http.client.HTTPSConnection("si.bitcrafter.net")
    conn.request("GET", f"/{resource}")
    rsp = conn.getresponse()
    data = rsp.read()
    with open(os.path.join(BASE_DIR, resource), "wb") as f:
        f.write(data)
    return data


def cleanmkdir(path):
    shutil.rmtree(path, ignore_errors=True)
    os.mkdir(path)


cleanmkdir(BASE_DIR)

with open("islets.json5") as f:
    islets = json5.load(f)

for d in [GAME_DIR, IMG_DIR]:
    cleanmkdir(os.path.join(BASE_DIR, d))

for islet in islets:
    if (bitcrafter := islet.get("bitcrafter")) is None:
        continue
    data = download(f"{GAME_DIR}/{bitcrafter}")
    for m in IMG_REGEX.finditer(data.decode()):
        download(m.group(1))

for link in LINKS:
    print(f"LNK {link}")
    os.symlink(f"../{link}", os.path.join(BASE_DIR, link))

os.mkdir(os.path.join(BASE_DIR, ASSETS))
for f in os.listdir(ASSETS):
    if f == CFG:
        continue
    asset = os.path.join(ASSETS, f)
    print(f"LNK {asset}")
    os.symlink(f"../../{asset}", os.path.join(BASE_DIR, asset))

with open(os.path.join(BASE_DIR, ASSETS, CFG), "w") as f:
    print(f"CFG {ASSETS}/{CFG}")
    json5.dump(
        {
            "base_url": "http://localhost:8000",
            "border_px": 1.0,
        },
        f,
    )
