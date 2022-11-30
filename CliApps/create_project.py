import subprocess
from argparse import ArgumentParser
from pathlib import Path


def create_new_project(name: str):
    project_folder = Path.cwd().absolute() / name
    project_folder.mkdir()
    (project_folder / "README.md").touch()

    with open(project_folder / ".gitignore", mode="w") as file:
        file.write("\n".join(["venv", "__pycache__"]))
