import sys

def read_data():
    if len(sys.argv) > 2:
        print("Invalid command line arguments")
        sys.exit(1)
    filename = sys.argv[1] if len(sys.argv) == 2 else None
    if filename is None or filename == "-":
        return sys.stdin.read()
    else:
        with open(filename) as f:
            return f.read()


def get_lines(data):
    return [line.strip() for line in data.strip().split('\n')]
