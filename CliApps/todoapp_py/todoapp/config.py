"""This module provides the ToDoApp config functionality"""

import configparser
from pathlib import Path

import typer

from todoapp import DB_WRITE_ERROR, DIR_ERROR, FILE_ERROR, SUCESS, __app_name__

CONFIG_DIR_PATH = Path(typer.get_app_dir(__app_name__))
CONFIG_FILE_PATH = CONFIG_DIR_PATH / "config.ini"


def init_app(db_path: str) -> int:
    """Initialize the application
    Expected side effects:
        - new file: config.ini
        - new file: $HOME/todoapp.json
    """
    config_code = _init_config_file(db_path)
    if config_code != SUCESS:
        return config_code
    database_code = _create_database(db_path)
    if database_code != SUCESS:
        return database_code
    return SUCESS


def _init_config_file(db_path) -> int:
    """Creates config.ini file"""
    try:
        CONFIG_DIR_PATH.mkdir(exist_ok=True)
    except OSError:
        return DIR_ERROR
    try:
        CONFIG_FILE_PATH.touch(exist_ok=True)
    except OSError:
        return FILE_ERROR

    config_parser = configparser.ConfigParser()
    config_parser["General"] = {"database": db_path}
    if Path(db_path).stat().st_size == 0:
        try:
            with CONFIG_FILE_PATH.open("w") as file:
                config_parser.write(file)
        except OSError:
            return DB_WRITE_ERROR

    return SUCESS


def _create_database(db_path: str) -> int:
    db_path = Path(db_path)
    return SUCESS


def get_database_path(config_file: Path) -> Path:
    """Return the current path to the todo database"""
    config_parser = configparser.ConfigParser()
    config_parser.read(config_file)
    return Path(config_parser["General"]["database"])


def init_database(db_path: Path) -> int:
    """Create the todo database"""
    try:
        db_path.write_text("[]")
        return SUCESS
    except OSError:
        return DB_WRITE_ERROR
