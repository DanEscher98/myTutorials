from todoapp import __version__
from todoapp.testing import CliRunner


def test_version():
    result = runner.invoke
    assert __version__ == "0.1.0"
