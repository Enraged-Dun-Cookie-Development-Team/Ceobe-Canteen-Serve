import os
import pathlib
from posixpath import split
import re
import sys
from this import d
from unittest import result
from venv import create
from string import punctuation

sql_model_dir = "./models/sql-models"
operation_template = """
use super::%sSqlOperate;

impl %sSqlOperate {
    todo!();
}
"""
folder_mod_template = """
pub mod checkers;
pub mod models;
pub mod operate;
"""

checker_mod_template = """
use std::convert::Infallible;
use thiserror::Error;

use status_err::{ErrPrefix, StatusErr, HttpCode};
pub use CheckError::*;

#[derive(Debug, Error)]
pub enum CheckError {
    
}

impl StatusErr for CheckError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            
        }
    }

    fn code(&self) -> u16 {
        match self {
            
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            
        }
    }
}
"""
checker_template = """
use checker::check_obj;
use typed_builder::TypedBuilder;

use super::CheckError;
use crate::%s::models::model_%s;

#[derive(Debug, TypedBuilder)]
pub struct %s {
    
}

check_obj! {
    #[derive(Debug,serde::Deserialize)]
    pub struct %sUncheck = %sChecker > %s{
        todo!();
    }
    err : CheckError
}

impl model_announcement::ActiveModel {
    todo!();
}
"""
operate_mod_template = """
use thiserror::Error;
use status_err::{ErrPrefix, StatusErr, HttpCode};

pub struct %sSqlOperate;

pub use OperateError::*;

#[derive(Debug, Error)]
pub enum OperateError {
    
}
#[allow(dead_code)]
type OperateResult<T> = Result<T, OperateError>;

impl StatusErr for OperateError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            
        }
    }

    fn code(&self) -> u16 {
        match self {
            
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            
        }
    }
}
"""
operate_template = """
use super::%sSqlOperate;

impl %sSqlOperate {
    todo!();
}
"""
model_template = """
use chrono::Local;
use sea_orm::{ entity::prelude::*, Set };

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "%s")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef { panic!("No Relate") }
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    // 软删除
    pub fn soft_remove(&mut self) {
        let now = Local::now().naive_local();
        self.delete_at = Set(now);
    }

    // 还原删除
    pub fn soft_recover(&mut self) {
        let date_time = chrono::NaiveDateTime::from_timestamp(0, 0);
        self.delete_at = Set(date_time)
    }
}
"""


mod_patten = re.compile(r'(?:pub)? mod ([a-zA-Z_][a-zA-Z0-9_]*);')

def name_convert_to_camel(name: str) -> str:
    name = name_convert_to_snack(name)
    return re.sub(r'(_[a-z])', lambda x: x.group(1)[1].upper(), name.capitalize())
def name_convert_to_snack(name: str) -> str:
    return name.replace("::", "_")

class RustLib(object):
    def __init__(self, crate_path):
        self.need_add_mods = []
        self.src_dir = os.path.join(crate_path, "src")
        self.lib_file = os.path.join(self.src_dir, "lib.rs")
        with open(self.lib_file, "r+", encoding="utf-8") as f:
            self.file = f.read()

    def __del__(self):
        with open(self.lib_file,"w") as f :
            f.write(self.file)

    def get_src_dir(self):
        return self.src_dir

    def get_lib_file(self):
        return self.lib_file

    def add_mod(self, rs_mod):
        if rs_mod not in self.need_add_mods:
            self.need_add_mods.append(rs_mod)

# 在lib文件中，将need_add_mods加入现有mod的后面
    def writing_mods(self):
        exist_mod = mod_patten.findall(self.file)
        vec = []

        for rs_mod in self.need_add_mods:
            if rs_mod not in exist_mod:
                vec.append(f"pub mod {rs_mod};\n")

        mods = ''.join(vec)
        self.file = f"{mods}{self.file}"

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
        if rs_mod not in self.need_add_mods:
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
                os.makedirs(mod_file_path)
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
    def __init__(self, mod_list: list, mod_name) -> None:
        self.name = mod_name
        self.path = mod_list
        self.need_add_mods = ["checkers", "models", "operate"]
        self.before_path = self.get_before_path()

    def get_filename(self):
        return self.name

    def get_before_path(self):
        file = ""
        for mod in self.path:
            file = file + f"{mod.get_name()}::"
        file = "".join([ele.strip(punctuation) for ele in file.split()])
        return file

    def create_file_dir(self, base):
        base_dir = base
        for rs_mod in self.path:
            rs_mod: RustMod
            base_dir = rs_mod.writing_mod(base_dir, True)
        return base_dir

    def create_self_mod(self, base_dir):
        with open(os.path.join(base_dir, "mod.rs"), "w") as mod_file:
            for rs_mod in self.need_add_mods:
                mod_file.write(f"pub mod {rs_mod};\n")

    def writing_cmo_mod_file(self, base_dir, curd):
        base_dir = self.create_file_dir(base_dir)
        self.create_self_mod(base_dir)

        self.create_cmo_folder(base_dir, curd)

    def create_cmo_folder(self, base_dir, curd):
        check = CheckerMod(self.name, base_dir, self.before_path)
        model = ModelsMod(self.name, base_dir, self.before_path)
        operate = OperateMod(self.name, base_dir, self.before_path)
        operate.add_operate(curd)

        check.create_mod()
        check.create_files()
        model.create_mod()
        model.create_files()
        operate.create_mod()
        operate.create_files()




class CheckerMod(object):
    def __init__(self, name, base_dir, before_path):
        self.path = os.path.join(base_dir, "checkers")
        if not os.path.exists(self.path):
            os.makedirs(self.path)
        self.name = name
        self.before_path = before_path
        self.need_add_mods = []
        self.add_mod(f"{self.name}_data")

    def add_mod(self, mod_name):
        if mod_name not in self.need_add_mods:
            self.need_add_mods.append(mod_name)

    def create_mod(self):
        if not os.path.exists(self.path):
            os.makedirs(self.path)
        path = os.path.join(self.path, "mod.rs")
        with open(path, "w") as mod_file:
            for rs_mod in self.need_add_mods:
                mod_file.write(f"pub mod {rs_mod};\n")
            mod_file.write(checker_mod_template)
    
    def create_files(self):
        if not os.path.exists(self.path):
                os.makedirs(self.path)
        for rs_mod in self.need_add_mods: 
            path = os.path.join(self.path, f"{rs_mod}.rs")
            with open(path, "w") as mod_file:
                mod_file.write(checker_template % ( self.before_path, self.name, name_convert_to_camel(self.before_path), name_convert_to_camel(self.before_path), name_convert_to_camel(self.before_path), name_convert_to_camel(self.before_path) ))

    


class ModelsMod(object):
    def __init__(self, name, base_dir, before_path):
        self.path = os.path.join(base_dir, "models")
        if not os.path.exists(self.path):
            os.makedirs(self.path)

        self.name = name
        self.need_add_mods = []
        self.before_path = before_path
        self.add_mod(f"model_{self.name}")

    def add_mod(self, mod_name):
        if mod_name not in self.need_add_mods:
            self.need_add_mods.append(mod_name)


    def create_mod(self):
        if not os.path.exists(self.path):
            os.makedirs(self.path)
        path = os.path.join(self.path, "mod.rs")
        with open(path, "w") as mod_file:
            for rs_mod in self.need_add_mods:
                mod_file.write(f"pub mod {rs_mod};\n")
    
    def create_files(self):
        if not os.path.exists(self.path):
                os.makedirs(self.path)
        for rs_mod in self.need_add_mods: 
            path = os.path.join(self.path, f"{rs_mod}.rs")
            with open(path, "w") as mod_file:
                mod_file.write(model_template % f"{name_convert_to_snack(self.before_path)}_{self.name}")



class OperateMod(object):
    def __init__(self,name, base_dir, before_path):
        self.path = os.path.join(base_dir, "operate")
        if not os.path.exists(self.path):
            os.makedirs(self.path)
        self.name = name
        self.need_add_mods = []
        self.before_path = before_path

    def add_mod(self, mod_name):
        if mod_name not in self.need_add_mods:
            self.need_add_mods.append(mod_name)


    def add_operate(self, operations="curd"):
        if "c" in operations:
            self.add_mod("create")
        if "u" in operations:
            self.add_mod("update")
        if "r" in operations:
            self.add_mod("retrieve")
        if "d" in operations:
            self.add_mod("delete")
    
    def create_mod(self):
        if not os.path.exists(self.path):
            os.makedirs(self.path)
        path = os.path.join(self.path, "mod.rs")
        with open(path, "w") as mod_file:
            for rs_mod in self.need_add_mods:
                mod_file.write(f"pub mod {rs_mod};\n")
            mod_file.write(operate_mod_template % f"{name_convert_to_camel(self.before_path)}")
    
    def create_files(self):
        if not os.path.exists(self.path):
            os.makedirs(self.path)
        for rs_mod in self.need_add_mods: 
            path = os.path.join(self.path, f"{rs_mod}.rs")
            with open(path, "w") as mod_file:
                mod_file.write(operate_template % (f"{name_convert_to_camel(self.before_path)}", f"{name_convert_to_camel(self.before_path)}"))


# 处理带个路径
# path: ceobe/operation/video
def from_input_path(rs_lib: RustLib, path, base_path) -> CMO:
    _, cmo_name = os.path.split(path)
    rs_mods = []
    for mod_name in re.split(r"[\\/]", path):
        mod_name = mod_name.strip()
        if mod_name == "":
            continue
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

    cmo = CMO(rs_mods, cmo_name)

    return cmo


if __name__ == '__main__':
    operates = sys.argv[1:]

    if len(operates) == 0:
        print(
            f"Using like `python {sys.argv[0]} \"<path1>/.../<path1>/<model_name> <curd>\"")
        sys.exit(1)

    rs_lib = RustLib(sql_model_dir)

    for operate in operates:
        path_operate = operate.split(" ")
        path = path_operate[0]
        curd = path_operate[1]

        cmo = from_input_path(rs_lib, path, rs_lib.get_src_dir())
        cmo.writing_cmo_mod_file( rs_lib.get_src_dir(), curd)

    rs_lib.writing_mods()
