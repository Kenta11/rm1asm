#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import argparse
import os
import subprocess

import toml
import jinja2

parser = argparse.ArgumentParser(description="Process key(s)")
parser.add_argument('--helpmessage', action='store_true')
parser.add_argument('src')
parser.add_argument('dst')

project_root = os.path.dirname(__file__) + "/../"

args = parser.parse_args()
data = dict()

data["version"] = toml.load(open(f"{project_root}/Cargo.toml"))["package"]["version"]
if args.helpmessage:
    help = str(
        subprocess.run(
            ["cargo", "run", "--", "--help"], cwd=project_root, capture_output=True
        ).stdout,
        "utf-8",
    )
    data["help"] = help
else:
    data["help"] = ""

env = jinja2.Environment(loader=jinja2.FileSystemLoader(project_root))
template = env.get_template(args.src)
rendered = template.render(data)
with open(args.dst, "w") as f:
    f.write(str(rendered))
