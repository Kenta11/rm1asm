#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import subprocess

import toml
import jinja2

dirname = os.path.dirname(__file__)
version = toml.load(open(f"{dirname}/../Cargo.toml"))["package"]["version"]
help = str(
    subprocess.run(
        ["cargo", "run", "--", "--help"], cwd=f"{dirname}/..", capture_output=True
    ).stdout,
    "utf-8",
)

env = jinja2.Environment(loader=jinja2.FileSystemLoader(f"{dirname}/templates"))
template = env.get_template("rm1asm.1.md")
data = {
    "version": version,
}
rendered = template.render(data)
with open("target/man/rm1asm.1.md", "w") as f:
    f.write(str(rendered))
