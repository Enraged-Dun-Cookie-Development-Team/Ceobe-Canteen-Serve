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

import datetime
from functools import reduce
import os
import pathlib
import sys
import re

mod_patten = re.compile(r'(?:pub )?mod ([a-zA-Z_][a-zA-Z0-9_]*);')
migrate_patten = re.compile(r'crate::migrate_group!\[([\n\sa-z_:A-Z0-9]+)(?=\n\s+])')


class RustLib(object):
    def __init__(self, crate_path):
        self.need_add_mods = []
        self.need_add_migrate = []
        self.src_dir = os.path.join(crate_path, "src")
        self.lib_file = os.path.join(self.src_dir, "lib.rs")
        with open(self.lib_file, "r+", encoding="utf-8") as f:
            self.file = f.read()

    def __del__(self):
        with open(self.lib_file,"w") as f :
            f.write(self.file)

    def get_src_dir(self):
        return self.src_dir

    def add_mod(self, rs_mod):
        self.need_add_mods.append(rs_mod)

    def add_migration(self, migrate):
        self.need_add_migrate.append(migrate)

    def writing_mods(self):
        exist_mod = mod_patten.findall(self.file)
        vec = []
        for rs_mod in self.need_add_mods:
            if rs_mod not in exist_mod:
                vec.append(f"mod {rs_mod};")
        
        add_mods = '\n'.join(vec) + "\n" if vec else ""
        self.file = f"{add_mods}{self.file}"
        
        
    def writing_extra_migration(self):
        result = migrate_patten.search(self.file)
        end = result.end()
        vec = []

        for migrate in self.need_add_migrate:
            migrate: Migration
            path = migrate.get_use_way()
            vec.append(f"\n{' ' * 12}{path}")
        
        start = self.file[:end]
        migrates = ''.join(vec)
        end = self.file[end:]
        self.file = f"{start}{migrates}{end}"


class RustMod(object):
    def __init__(self, mod_name) -> None:
        self.name = mod_name
        self.need_add_mods = []

    def get_dir_path(self, base: str) -> str:
        return os.path.join(base, self.name)

    def get_inner_mod_file_path(self, base) -> str:
        return os.path.join(self.get_dir_path(base), "mod.rs")

    def get_outer_mod_file_path(self, base) -> str:
        return os.path.join(base, f"{self.name}.rs")

    def get_name(self):
        return self.name

    def add_mod(self, rs_mod: str):
        self.need_add_mods.append(rs_mod)
        
    def __str__(self) -> str:
        return f"Mod {self.name}"
    
    __repr__ = __str__

    # 检查model 是否存在    
    def model_exist(self, base_path: str) -> bool:
        # 1 拼接路径
        # 2 判断路径指定文件夹是否存在
        # 3 判断 指定文件夹内 `mod.rs` 文件或者 `base_path` 同级同名rs文件存在
        dir_path = self.get_dir_path(base=base_path)

        dir_exist = os.path.exists(dir_path) & os.path.isdir(dir_path)

        inner_file = self.get_inner_mod_file_path(base_path)
        outer_file = self.get_outer_mod_file_path(base_path)

        mod_file_exist = (os.path.exists(inner_file) & os.path.isfile(inner_file)) | (
                os.path.exists(outer_file) & os.path.isfile(outer_file))

        return dir_exist & mod_file_exist

    def create_mod(self, base_path, using_inner: bool = True):
        """
        创建一个mod
        :param using_inner: 使用内部mod.rs文件 ，否则使用外部文件
        :return: None
        """
        dir_path = self.get_dir_path(base_path)
        if not self.model_exist(base_path):
            # dir not exist need create
            if not os.path.exists(dir_path):
                os.makedirs(self.get_dir_path(base_path))
            # create mob file
            if using_inner:
                mod_file_path = self.get_inner_mod_file_path(base_path)
            else:
                mod_file_path = self.get_inner_mod_file_path(base_path)
            if os.path.exists(mod_file_path):
                pathlib.Path(mod_file_path).touch()
        return dir_path

    def writing_mod(self, base_path, using_inner: True):
        # 先创建相关文件
        next_path = self.create_mod(base_path, using_inner)

        # 写入需要包括的mod
        if using_inner:
            mod_file_path = self.get_inner_mod_file_path(base_path)
        else:
            mod_file_path = self.get_outer_mod_file_path(base_path)

        with open(mod_file_path, "a+") as mod_file:
            mod_file.seek(0, 0)
            all_code = mod_file.read()
            exist_mod = mod_patten.findall(all_code)

            for rs_mod in self.need_add_mods:
                if rs_mod not in exist_mod:
                    mod_file.write(f"pub mod {rs_mod};\n")

        return next_path


class Migration(object):
    def __init__(self, prefix: list, name):
        self.path = prefix
        self.migrate_name, self.file_name = Migration.get_names(self.path, name)

    @staticmethod
    def get_names(path, file_name):
        now_time = datetime.datetime.now()
        pre_time = f"m{now_time.year:04}{now_time.month:02}{now_time.day:02}_" \
                   f"{now_time.hour:02}{now_time.minute:02}{now_time.second:02}"

        pre =   f"_{'_'.join(map(lambda x: x.get_name(), path))}" if len(path) >0 else ""

        return f"{pre_time}{pre}_{file_name}", f"{pre_time}_{file_name}"

    def get_filename(self):
        return self.file_name

    def get_migrate_name(self):
        return self.migrate_name

    def get_mod_path(self, base):
        path = map(lambda x: x.get_name(), self.path)
        path = reduce(lambda x, y: os.path.join(x, y), path, "")
        return os.path.join(base,path)

    def get_use_way(self):
        base = "::".join(map(lambda x: x.get_name(), self.path))
        base = f"{base}::" if len(base)>0 else ""
        return f"{base}{self.file_name}::Migration"

    def get_file_local(self, base):
        # 构造路径
        mod_path = self.get_mod_path(base)
        return os.path.join(mod_path, f"{self.file_name}.rs")

    def create_file_dir(self, base):
        base_dir = base
        for rs_mod in self.path:
            rs_mod: RustMod
            base_dir = rs_mod.writing_mod(base_dir, True)

    def writing_migrate_file(self, base_dir):
        self.create_file_dir(base_dir)

        with open(self.get_file_local(base_dir), "w") as migrate_file:
            migrate_file.write(template % self.get_migrate_name())


# ceobe/video/add_update
def from_input_path(rs_lib:RustLib,path, base_path) -> Migration:
    head, migrate_name = os.path.split(path)
    rs_mods = []
    for mod_name in re.split(r"[\\/]", head):
        mod_name = mod_name.strip()
        if mod_name == "": continue
        rs_mod = RustMod(mod_name)

        # 添加 mod 到上一级
        if len(rs_mods) == 0:
            # 第一级
            rs_lib.add_mod(rs_mod.get_name())
        else:
            if not rs_mod.model_exist(base_path):
                rs_mods[len(rs_mods) - 1].add_mod(rs_mod.get_name())
        rs_mods.append(rs_mod)

        base_path = os.path.join(base_path, mod_name)

    migration = Migration(rs_mods, migrate_name)

    if len(rs_mods) == 0:
        rs_lib.add_mod(migration.get_filename())
    else:
        rs_mods[len(rs_mods) - 1].add_mod(migration.get_filename())
    rs_lib.add_migration(migration)
    return migration


if __name__ == '__main__':
    paths = sys.argv[1:]
    
    if len(paths) == 0 :
        print(f"Using like `python {sys.argv[0]} <path1>/.../<path1>/<migrate_name>")
        sys.exit(1)
    
    rs_lib = RustLib(migrate_dir)
    for path in paths:
        
        migrate = from_input_path(rs_lib, path, rs_lib.get_src_dir())
        print(migrate.path, migrate.migrate_name, migrate.file_name, sep="\n")

        migrate.writing_migrate_file(rs_lib.get_src_dir())
    rs_lib.writing_mods()
    rs_lib.writing_extra_migration()
    
    del rs_lib
