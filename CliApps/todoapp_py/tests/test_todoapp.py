import json
from pathlib import Path

import pytest
from typer.testing import CliRunner

import todoapp
from todoapp import DB_READ_ERROR, SUCESS, __app_name__, __version__, app, cli

runner = CliRunner()


def test_version():
    result = runner.invoke(cli.typer_app, ["--version"])
    assert result.exit_code == 0
    assert f"{__app_name__} v{__version__}\n" in result.stdout


@pytest.fixture
def mock_json_file(tmp_path: Path):
    todo = [{"Description": "Get some milk", "Priority": 2, "Done": False}]
    db_file = tmp_path / "todo.json"
    with db_file.open("w") as data_base:
        json.dump(todo, data_base, indent=4)
    return db_file


test_data1 = {"description": "Clean the hose.", "priority": 1, "done": False}

test_data2 = {"description": "Wash the car.", "priority": 2, "done": False}


@pytest.mark.parametrize(
    "description, priority, expected",
    [
        pytest.param(
            test_data1["description"], test_data1["priority"], (test_data1, SUCESS)
        ),
        pytest.param(
            test_data2["description"], test_data2["priority"], (test_data2, SUCESS)
        ),
    ],
)
def test_add(mock_json_file: Path, description, priority, expected):
    # pylint: disable=protected-access
    todoer = app.Todoer(mock_json_file)
    assert todoer.add(description, priority) == expected
    read = todoer._db_handler.read_todos()
    assert len(read.todo_list) == 2
