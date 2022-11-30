""""To-Do entry point script """

from todoapp import __app_name__, __main__, app, cli


def main():
    cli.typer_app(prog_name=__app_name__)


if __name__ == "__main__":
    main()
