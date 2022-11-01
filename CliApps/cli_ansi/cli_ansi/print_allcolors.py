import random
import sys
import time

cli_goleft = lambda n: "\u001b[{}D".format(n)


class CliMove:
    @classmethod
    def left(cls, n: int) -> str:
        return "\u001b[{}D".format(n)

    @classmethod
    def up(cls, n: int) -> str:
        return "\u001b[{}A".format(n)


def print_colors():
    for i in range(0, 16):
        for j in range(0, 16):
            code = str(i * 16 + j)
            color = "\u001b[38;5;" + code + "m " + code.ljust(4)
            sys.stdout.write(color)
        print("\u001b[0m")


def loading():
    print("Loading...")
    for t in range(0, 101):
        time.sleep(0.01)
        sys.stdout.write("{}{}%".format(cli_goleft(1000), t))
        sys.stdout.flush()
    print()


def loading_1(length: int):
    print("Loading ... v2")
    for width in range(0, length + 1):
        time.sleep(0.05)
        bar = "[" + "#" * width + " " * (length - width) + "]"
        sys.stdout.write(CliMove.left(1000) + bar)
        sys.stdout.flush()
    print()


def loading_2(count: int, length: int):
    print("\nLoading ... v3")
    all_progress = [0] * count
    sys.stdout.write("\n" * count)
    while any(x < length for x in all_progress):
        time.sleep(0.01)
        unfinished = [(i, v) for (i, v) in enumerate(all_progress) if v < length]
        index, _ = random.choice(unfinished)
        all_progress[index] += 1

        # Draw the progress bar
        sys.stdout.write(CliMove.left(1000))
        sys.stdout.write(CliMove.up(count))

        for progress in all_progress:

            bar = "[" + "#" * progress + " " * (length - progress) + "]"
            print(bar)


loading()
loading_1(45)
loading_2(5, 50)
