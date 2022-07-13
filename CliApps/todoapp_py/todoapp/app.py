"""Provides the model-controller"""

from pathlib import Path
from typing import Any, Dict, List, NamedTuple

from todoapp import DB_READ_ERROR
from todoapp.database import DatabaseHandler


class CurrentTodo(NamedTuple):
    todo: Dict[str, Any]
    error: int


class Todoer:
    def __init__(self, db_path: Path):
        self._db_handler = DatabaseHandler(db_path)

    def add(self, description: str, priority: int = 2) -> CurrentTodo:
        """Add a new to-do to the database"""
        # description_text " ".join(description)
        # if not description_text.endswith("."):
        #     description_text += "."
        todo = {"description": description, "priority": priority, "done": False}
        read = self._db_handler.read_todos()
        if read.error == DB_READ_ERROR:
            return CurrentTodo(todo, read.error)
        read.todo_list.append(todo)
        write = self._db_handler.write_todos(read.todo_list)
        return CurrentTodo(todo, write.error)
