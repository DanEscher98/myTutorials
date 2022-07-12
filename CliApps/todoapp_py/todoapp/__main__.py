""""To-Do entry point script """"

from todoapp import cli, __main__

def main():
    cli.app(prog_name=__app_name__)

if __name__ == '__main__':
    main()
