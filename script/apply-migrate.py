import os
import subprocess
import sys
from tkinter.tix import Tree

migrate_path = "./migration"
abs_m=os.path.abspath(migrate_path)
entity_path = "./libs/db-entity"

print("Migrate Database")

print("------------------------------------")

mig_resp = subprocess.run(["cargo", "run", "--", "up"],
                          capture_output=True, text=True, shell=True,
                          cwd=os.path.abspath(migrate_path))

print(mig_resp.stderr)
print(mig_resp.stdout)

print("------------------------------------")

print("loading Entity")
resp = subprocess.run(["sea-orm-cli", "generate", "entity", "-o",
                      entity_path], shell=True, capture_output=True, text=True)

print(resp.stderr)
print(resp.stdout)

print("------------------------------------")

print("done")
