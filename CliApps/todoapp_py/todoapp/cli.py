""" This module provides the To-Do CLI """

from typing import List, Optional

import typer

from todoapp import ERRORS, __app_name__, __version__, app, config, database

typer_app = typer.Typer()


def _version_callback(value: bool) -> None:
    if value:
        typer.echo(f"{__app_name__} v{__version__}")
        raise typer.Exit()


@typer_app.callback()
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


@typer_app.command()
def init(
    db_path: str = typer.Option(
        str(database.DEFAULT_DB_FILE_PATH),
        "--db-path",
        "-db",
        prompt=f"{__app_name__} database location?",
    )
) -> None:
    """Initialize the todoapp database"""
    if app_init_error := config.init_app(db_path):
        typer.secho(
            f"Creating config file failed with: {ERRORS[app_init_error]}",
            fg=typer.colors.RED,
        )
        raise typer.Exit(1)
    typer.secho(f"The {__app_name__} database is {db_path}", fg=typer.colors.GREEN)


def get_todoer() -> app.Todoer:
    if config.CONFIG_FILE_PATH.exists():
        db_path = config.get_database_path(config.CONFIG_FILE_PATH)
    else:
        typer.secho(
            f'Config file not found. Please, run "{__app_name__} init"',
            fg=typer.colors.RED,
        )
        raise typer.Exit(1)
    if db_path.exists():
        return app.Todoer(db_path)
    typer.secho(
        f'Database not found. Please, run "{__app_name__} init"', fg=typer.colors.RED
    )
    raise typer.Exit(1)


@typer_app.command()
def add(
    description: str = typer.Argument(...),
    priority: int = typer.Option(2, "--priority", "-p", min=1, max=3),
) -> None:
    """Add a new to-do with a DESCRIPTION"""
    todoer = get_todoer()
    _, error = todoer.add(description, priority)
    if error:
        typer.secho(f'Adding to-do failed with "{ERRORS[error]}"', fg=typer.colors.RED)
        raise typer.Exit(1)
    typer.secho(
        f'{__app_name__}: "{description}" was added with priority: {priority}',
        fg=typer.colors.GREEN,
    )
