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


@typer_app.command(name="list")
def list_all() -> None:
    """List all to-dos"""
    todoer = get_todoer()
    todo_list = todoer.get_todo_list()
    if len(todo_list) == 0:
        typer.secho("There are no tasks in the to-do list yet", fg=typer.colors.RED)
        raise typer.Exit()
    typer.secho("\nto-do list:\n", fg=typer.colors.BLUE, bold=True)
    columns = ("ID.  ", "| Priority  ", "| Done  ", "| Description  ")
    headers = "".join(columns)
    typer.secho(headers, fg=typer.colors.BLUE, bold=True)
    typer.secho("-" * len(headers), fg=typer.colors.BLUE)

    for id, todo in enumerate(todo_list, 1):
        description, priority, done = todo.values()
        typer.secho(
            f"{id}{' ' * (len(columns[0]) - len(str(id)))}"
            f"| ({priority}){' ' * (len(columns[1]) - len(str(priority)) - 4)}"
            f"| {done}{' ' * (len(columns[2]) - len(str(done)) - 2)}"
            f"| {description}",
            fg=typer.colors.BLUE,
        )
    typer.secho("-" * len(headers) + "\n", fg=typer.colors.BLUE)


@typer_app.command(name="complete")
def set_done(todo_id: int = typer.Argument(...)) -> None:
    """Complete a to-do by setting it as done using its TODO_ID"""
    todoer = get_todoer()
    todo, error = todoer.set_done(todo_id)
    if error:
        typer.secho(
            f"Completing to-do #{todo_id} failed with {ERRORS[error]}",
            fg=typer.colors.RED,
        )
        raise typer.Exit(1)
    typer.secho(
        f"""to-do #{todo_id} '{todo["description"]}' completed!""",
        fg=typer.colors.GREEN,
    )


@typer_app.command(name="remove")
def remove(
    todo_id: int = typer.Argument(...),
    force: bool = typer.Option(
        False, "--force", "-f", help="Force deletion without confirmation"
    ),
) -> None:
    """Remove a to-do using its ID"""
    todoer = get_todoer()

    def _remove():
        todo, error = todoer.remove(todo_id)
        if error:
            typer.secho(
                f"Removing to-do #{todo_id} failed with {ERRORS[error]}",
                fg=typer.colors.RED,
            )
            raise typer.Exit(1)
        typer.secho(
            f"""to-do #{todo_id}: '{todo["description"]}' was removed.""",
            fg=typer.colors.GREEN,
        )

    if force:
        _remove()
    todo_list = todoer.get_todo_list()
    try:
        todo = todo_list[todo_id - 1]
    except IndexError as err:
        typer.secho("Invalid TODO_ID", fg=typer.colors.RED)
        raise typer.Exit(1) from err

    delete = typer.confirm(f"Delete to-do #{todo_id}: {todo['description']}?")
    if delete:
        _remove()
    typer.echo("Operation canceled")
