template = """
use sea_orm_migration::prelude::*;


pub struct Migration;
impl MigrationName for Migration {
    fn name(&self) -> &str { "%s" }
}
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        todo!()
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        todo!()
    }
}
"""
migrate_dir = "./migrate/sql-migration"

# migrate dir
from cProfile import label
import datetime
from functools import reduce
import os
import pathlib
import sys

extra_mods = []
extra_migrate = []


def add_migrate(model, name):
    name = f"{model}::{name}::Migration"
    extra_migrate.append(name)


try:
    # migrate 名称
    name = sys.argv[1]
    # migrate 所在的mod
    model = sys.argv[2]
except IndexError as e:
    print("need migrate name and model")
    print(f"using like `python {sys.argv[0]} <example_migrate> <example_mod>`")
    sys.exit(1)

print(f"starting add migration {model}->{name}")

lib_dir = os.path.join(migrate_dir, "src")
lib_file = os.path.join(lib_dir, "lib.rs")

# 生成migrate
now = datetime.datetime.now()

format_str = f"m{now.year:04}{now.month:02}{now.day:02}_{now.hour:02}{now.minute:02}{now.second:02}_{name}"
migrate_file = f"{format_str}.rs"

print(f"new migrate file name is {migrate_file}")

target_mod = os.path.join(lib_dir, model)
mod_file = os.path.join(target_mod, "mod.rs")
migrate_filename = os.path.join(target_mod, migrate_file)

if os.path.exists(migrate_filename):
    raise Exception("target migrate exist")

if not os.path.exists(target_mod):
    print(f"target model {target_mod} not exist, create it")
    os.makedirs(target_mod)
    pathlib.Path(mod_file).touch()
    extra_mods.append(model)

print(f"adding new migration file[{mod_file}] to mod")
with open(mod_file, "a") as mod:
    mod.write(f"pub mod {format_str};\n")

print(f"writing migration file[{mod_file}]")
with open(migrate_filename, "w") as migrate:
    migrate.write(template % format_str)

print(f"register migration file[{model}::{format_str}]")
add_migrate(model, format_str)

# 添加到跟lib
import re

print(f"reading crate lib [{lib_file}]")
with open(lib_file, "r") as crate_lib:
    lib_file_inner = crate_lib.read()

print(f"adding new models size:[{len(extra_mods)}]")
mods = reduce(lambda x, y: f"{x}mod {y};\n", extra_mods, "")
add_mods = f"{mods}\n{lib_file_inner}"

patten = re.compile(r'crate::migrate_group!\[([\n\sa-z_:A-Z0-9]+)(?=\n\s+])')

start, end = patten.search(add_mods).span()
upper = add_mods[:end]
down = add_mods[end:]
print(f"adding new migrate size: {len(extra_migrate)}")
extra_mig = reduce(lambda x, y: f"{x}\n{' ' * 12}{y}", extra_migrate, "")
final = f"{upper}{extra_mig}{down}"

with open(lib_file, "w") as crate_lib:
    crate_lib.write(final)
