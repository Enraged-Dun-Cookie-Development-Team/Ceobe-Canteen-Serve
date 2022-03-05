import os
import subprocess
import time

now = "./"
abs_now = os.path.abspath(now)
migrate_path = "./migration"
abs_m = os.path.abspath(migrate_path)
entity_path = "./libs/db-entity/src/entity"
abs_e = os.path.abspath(entity_path)

print("Migrate Database")

print("------------------------------------")

mig_resp = subprocess.run(["cargo", "run", "--", "up"],
                          capture_output=True, text=True, shell=True,
                          cwd=abs_m)

print(mig_resp.stderr)
print(mig_resp.stdout)

print("------------------------------------")

print("loading Entity")
subprocess.run(["del","*.rs"], shell=True,
               cwd=abs_e
               )

time.sleep(2)

resp = subprocess.run(["sea-orm-cli", "generate", "entity", "-o",
                      entity_path], shell=True, capture_output=True, text=True,
                      cwd=abs_now)

print(resp.stderr)
print(resp.stdout)

print("------------------------------------")

print("done")
