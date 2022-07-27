import os
import pathlib
from posixpath import split
import re
import sys

sql_model_dir = "./models/sql-models"
operation_template = """
use super::%sSqlOperate;

impl %sSqlOperate {
    todo!()
}
"""
folder_mod_template = """
pub mod checkers;
pub mod models;
pub mod operate;
"""

mod_patten = re.compile(r'(?:pub)? mod ([a-zA-Z_][a-zA-Z0-9_]*);')

class RustLib(object):
    def __init__(self, crate_path):
        self.need_add_mods = []
        self.src_dir = os.path.join(crate_path, "src")
        self.lib_file = os.path.join(self.src_dir, "lib.rs")
        with open(self.lib_file, "r+", encoding="utf-8") as f:
            self.file = f.read()

    def get_src_dir(self):
        return self.src_dir

    def get_lib_file(self):
        return self.lib_file

    def add_mod(self, rs_mod):
        self.need_add_mods.append(rs_mod)


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

class CMO(object):
    def __init__(self, path, mod_name) -> None:
        self.name = mod_name
        self.path = path

    def get_filename(self):
        return self.name

class CheckerMod(object):
    pass

class ModelsMod(object):
    pass

class OperateMod(object):
    pass

# path: ceobe/operation/video
def from_input_path(rs_lib:RustLib, path, base_path) -> CMO:
    _, cmo_name = os.path.split(path)
    rs_mods = []
    for mod_name in re.split(r"[\\/]", path):
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

    cmo = CMO(base_path, cmo_name)

    if len(rs_mods) == 0:
        rs_lib.add_mod(cmo.get_filename())
    else:
        rs_mods[len(rs_mods) - 1].add_mod(cmo.get_filename())
    return cmo



if __name__ == '__main__':
    operates = sys.argv[1:]
    
    if len(operates) == 0 :
        print(f"Using like `python {sys.argv[0]} \"<path1>/.../<path1>/<model_name> <c> <u> <r> <d>\"")
        sys.exit(1)
    
    rs_lib = RustLib(sql_model_dir)

    for operate in operates:
        path_operate = operate.split(" ")
        path = path_operate[0] 
        curd = path_operate[1:]







    