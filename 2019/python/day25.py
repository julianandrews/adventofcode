import enum
import queue
import re

import readline

from utils import read_data
from utils.intcode import VM


def gray_code(n):
    mask = n >> 1
    while mask:
        n = n ^ mask
        mask = mask >> 1
    return n


class RoomParseError(ValueError):
    pass


class Room:
    def __init__(self, name, description, exits, items):
        self.name = name
        self.description = description
        self.exits = exits
        self.items = items

    @classmethod
    def from_text(cls, text):
        room_text = text.split("\n\n\n")[-1]
        sections = room_text.strip().split("\n\n")
        if len(sections) < 2:
            raise RoomParseError("Insufficient sections")

        description_lines = sections[0].split("\n")
        if len(description_lines) != 2:
            raise RoomParseError("Description not found")
        name = description_lines[0][3:-3]
        description = description_lines[1]
        if not name.startswith("== ") and name.endswith(" =="):
            raise RoomParseError("Name not found")

        if not sections[1].startswith("Doors here lead:\n"):
            raise RoomParseError("Exits not found")
        exit_lines = sections[1].split("\n")[1:]
        exits = set()
        for line in exit_lines:
            if not line.startswith("- "):
                raise RoomParseError("Incorrectly formatted exit")
            exits.add(line[2:])

        items = set()
        if len(sections) > 2 and sections[2].startswith("Items here:\n"):
            item_lines = sections[2].split("\n")[1:]
            for line in item_lines:
                if not line.startswith("- "):
                    raise RoomParseError("Incorrectly formatted item")
                items.add(line[2:])

        return Room(name, description, exits, items)


class MazeRunner:
    PROMPT = "\nCommand?\n"

    def __init__(self, program, print_output=True):
        self.print_output = print_output
        self.input_queue = queue.Queue()
        self.vm = VM(program[:], inputs=self.inputs())
        self.room = None
        self.inventory = set()
        self.keycode = None

    def print(self, text):
        if self.print_output:
            print(text, end="")

    def inputs(self):
        while True:
            yield self.input_queue.get()

    def handle_output(self, text):
        text = text[:-len(self.PROMPT)].strip()
        if re.match("You take the [^\n]+.", text):
            item = text[13:-1]
            self.inventory.add(item)
            self.room.items.remove(item)
        elif re.match("You drop the [^\n]+.", text):
            item = text[13:-1]
            self.inventory.remove(item)
            self.room.items.add(item)
        elif re.match("== [^\n]+ ==", text):
            self.room = Room.from_text(text)

    def handle_end_text(self, text):
        match = re.search(r"You should be able to get in by typing (\d+)", text)
        if match:
            self.keycode = match.group(1)

    def get_command(self):
        raise NotImplementedError

    def get_input(self):
        command = self.get_command()
        for c in command:
            self.input_queue.put(ord(c))
        self.input_queue.put(ord("\n"))

    def run(self):
        outputs = self.vm.outputs()
        while True:
            text = ""
            while not text.endswith(self.PROMPT):
                try:
                    text += chr(next(outputs))
                except StopIteration:
                    self.print(text)
                    self.handle_end_text(text)
                    return

            self.print(text)
            self.handle_output(text)
            try:
                self.get_input()
            except EOFError:
                return


class RunnerState(enum.Enum):
    EXPLORING = 0
    WALKING = 1
    SOLVING = 2


class ItemCombos:
    def __init__(self, items):
        self.items = list(items)
        self.current_combo = 0

    def __next__(self):
        mask = gray_code(self.current_combo)
        items = {
            item for i, item in enumerate(self.items)
            if (1 << i) & mask
        }
        self.current_combo += 1
        return items


class AutomaticRunner(MazeRunner):
    FORBIDDEN_ITEMS = {
        "escape pod",
        "giant electromagnet",
        "infinite loop",
        "molten lava",
        "photons",
    }

    def __init__(self, program, print_output):
        super().__init__(program, print_output)
        self.state = RunnerState.EXPLORING
        self.visited = {}
        self.path = []
        self.checkpoint_path = None
        self.item_combos = None
        self.desired_inventory = None

    @staticmethod
    def reverse_exit(exit):
        if exit == "north":
            return "south"
        elif exit == "south":
            return "north"
        elif exit == "east":
            return "west"
        else:
            return "east"

    def explore(self):
        allowed_items = self.room.items - self.FORBIDDEN_ITEMS
        allowed_exits = self.room.exits - self.visited[self.room.name]
        if allowed_items:
            item = next(iter(allowed_items))
            command = f"take {item}"
        elif allowed_exits:
            exit = next(iter(allowed_exits))
            self.visited[self.room.name].add(exit)
            self.path.append(exit)
            command = exit
        elif self.path:
            last_move = self.path.pop()
            command = self.reverse_exit(last_move)
        else:
            self.state = RunnerState.WALKING
            command = None
        return command

    def solve(self):
        if self.desired_inventory is None or self.desired_inventory == self.inventory:
            self.desired_inventory = next(self.item_combos)
            allowed_exits = self.room.exits - {self.reverse_exit(self.path[-1])}
            if len(allowed_exits) != 1:
                raise RuntimeError("Unexpected exit from Security Checkpoint")
            return next(iter(allowed_exits))
        else:
            to_drop = self.inventory - self.desired_inventory
            if to_drop:
                return f"drop {next(iter(to_drop))}"
            to_pick_up = self.desired_inventory - self.inventory
            if to_pick_up:
                return f"take {next(iter(to_pick_up))}"

    def get_command(self):
        if self.room.name not in self.visited:
            if self.room.name == "Security Checkpoint":
                self.checkpoint_path = list(reversed(self.path))

            self.visited[self.room.name] = set()

        if self.state == RunnerState.EXPLORING:
            command = self.explore()
        if self.state == RunnerState.WALKING:
            if self.checkpoint_path:
                command = self.checkpoint_path.pop()
                self.path.append(command)
            else:
                self.item_combos = ItemCombos(self.inventory)
                self.state = RunnerState.SOLVING
        if self.state == RunnerState.SOLVING:
            command = self.solve()
        self.print(command)
        return command


class InteractiveRunner(MazeRunner):
    # Winning combo: whirled peas, mutex, festive hat, coin
    SHORTCUTS = {"n": "north", "s": "south", "e": "east", "w": "west"}

    def __init__(self, program):
        super().__init__(program)
        readline.set_completer(self.tab_completer)
        readline.parse_and_bind("tab: complete")
        readline.set_completer_delims("")

    def tab_completer(self, text, state):
        all_options = ['inv']
        if self.room is not None:
            all_options += self.room.exits
            for item in self.room.items:
                all_options.append(f"take {item}")
            for item in self.inventory:
                all_options.append(f"drop {item}")
        options = [option for option in all_options if option.startswith(text)]

        return options[state] if state < len(options) else None

    def get_command(self):
        inp = input()
        return self.SHORTCUTS.get(inp, inp)


if __name__ == "__main__":
    data = read_data(25)
    program = [int(x) for x in data.strip().split(',')]

    runner = AutomaticRunner(program, print_output=False)
    # runner = InteractiveRunner(program)
    runner.run()
    print(f"Part 1: {runner.keycode}")
