"""This module provides the ToDoApp config functionality"""

import configparser
from pathlib import Path

import typer

from todoapp import DB_WRITE_ERROR, DIR_ERROR, FILE_ERROR, SUCESS, __app_name__

CONFIG_DIR_PATH = Path(typer.get_app_dir(__app_name__))
CONFIG_FILE_PATH = CONFIG_DIR_PATH / "config.ini"


def init_app(db_path: str) -> int:
    """Initialize the application"""
    config_code = _init_config_file()
    if config_code != SUCESS:
        return config_code
    database_code = _create_database(db_path)
    if database_code != SUCESS:
        return database_code
    return SUCESS


def _init_config_file() -> int:
    try:
        CONFIG_DIR_PATH.mkdir(exist_ok=True)
    except OSError:
        return DIR_ERROR
    try:
        CONFIG_FILE_PATH.touch(exist_ok=True)
    except OSError:
        return FILE_ERROR
    return SUCESS


def _create_database(db_path: str) -> int:
    config_parser = configparser.ConfigParser()
    config_parser["General"] = {"database": db_path}
    try:
        with CONFIG_FILE_PATH.open("w") as file:
            config_parser.write(file)
    except OSError:
        return DB_WRITE_ERROR
    return SUCESS
