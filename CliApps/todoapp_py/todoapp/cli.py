""" This module provides the To-Do CLI """

from typing import Optional

import typer

from todoapp import ERRORS, __app_name__, __version__, config, database

app = typer.Typer()


def _version_callback(value: bool) -> None:
    if value:
        typer.echo(f"{__app_name__} v{__version__}")
        raise typer.Exit()


@app.callback()
def main(
    version: Optional[bool] = typer.Option(
        None,
        "--version",
        "-v",
        help="Show the apliccation's version and exit",
        callback=_version_callback,
    )
) -> None:
    pass


@app.command()
def init(
    db_path: str = typer.Option(
        str(database.DEFAULT_DB_FILE_PATH),
        "--db-path",
        "-db",
        prompt="ToDo database location?",
    )
) -> None:
    """Initialize the todoapp database"""
    app_init_error = config.init_app(db_path)
