"""This module provides ToDoApp database functionality"""

import configparser
import json
from pathlib import Path
from typing import Any, Dict, List, NamedTuple

from todoapp import (DB_READ_ERROR, DB_WRITE_ERROR, JSON_ERROR, SUCESS,
                     __app_name__)

DEFAULT_DB_FILE_PATH = Path.home().joinpath(f".{Path.home().stem}_{__app_name__}.json")


class DBResponse(NamedTuple):
    todo_list: List[Dict[str, Any]]
    error: int


class DatabaseHandler:
    def __init__(self, db_path: Path):
        self._db_path = db_path

    def read_todos(self) -> DBResponse:
        try:
            with self._db_path.open("r") as db:
                try:
                    return DBResponse(json.load(db), SUCESS)
                except json.JSONDecodeError:
                    return DBResponse([], JSON_ERROR)
        except OSError:
            return DBResponse([], DB_READ_ERROR)

    def write_todos(self, todo_list: List[Dict[str, Any]]) -> DBResponse:
        try:
            with self._db_path.open("w") as db:
                json.dump(todo_list, db, indent=4)
            return DBResponse(todo_list, SUCESS)
        except OSError:
            return DBResponse(todo_list, DB_WRITE_ERROR)
