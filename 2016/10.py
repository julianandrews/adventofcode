from utils import read_data

from collections import defaultdict
from functools import reduce


class Bot:
    def __init__(self):
        self.high = None
        self.output_high = None
        self.low = None
        self.output_low = None
        self.values = []

    def __str__(self):
        return "{} <- {} -> {}".format(self.low, self.values, self.high)

    def __repr__(self):
        return "Bot({})".format(self)


class Factory:
    def __init__(self, data):
        self.bots = defaultdict(Bot)
        self.outputs = defaultdict(list)
        rules = [line.strip() for line in data.strip().split('\n')]
        for rule in rules:
            words = rule.split()
            if words[0] == 'value':
                self.bots[int(words[5])].values.append(int(words[1]))
            elif words[0] == 'bot':
                bot = self.bots[int(words[1])]
                bot.low = int(words[6])
                bot.output_low = words[5] == 'output'
                bot.high = int(words[11])
                bot.output_high = words[10] == 'output'
        self.full_bots = {
            bot_id for bot_id, bot in self.bots.items()
            if len(bot.values) == 2
        }

    def __call__(self, interupt_values=None):
        while self.full_bots:
            to_process = []
            for bot_id in self.full_bots:
                bot = self.bots[bot_id]
                if interupt_values and set(bot.values) == set(interupt_values):
                    return bot_id
                low_open = bot.output_low or bot.low not in self.full_bots
                high_open = bot.output_high or bot.high not in self.full_bots
                if low_open and high_open:
                    to_process.append(bot_id)
            for bot_id in to_process:
                bot = self.bots[bot_id]
                targets = [
                    (bot.low, min(bot.values), bot.output_low),
                    (bot.high, max(bot.values), bot.output_high),
                ]
                for target_id, value, output in targets:
                    if output:
                        self.outputs[target_id].append(value)
                    else:
                        receiver = self.bots[target_id]
                        receiver.values.append(value)
                        if len(receiver.values) == 2:
                            self.full_bots.add(target_id)
                        elif len(receiver.values) > 2:
                            raise RuntimeError("Too many chips for bot!")
                bot.values = []
                self.full_bots.remove(bot_id)


if __name__ == '__main__':
    data = read_data(10)

    factory = Factory(
        """
        value 5 goes to bot 2
        bot 2 gives low to bot 1 and high to bot 0
        value 3 goes to bot 1
        bot 1 gives low to output 1 and high to bot 0
        bot 0 gives low to output 2 and high to output 0
        value 2 goes to bot 2
        """
    )
    factory()
    assert set(factory.outputs[0]) == {5}
    assert set(factory.outputs[1]) == {2}
    assert set(factory.outputs[2]) == {3}
    print("All tests passed")

    print(Factory(data)(interupt_values={61, 17}))
    factory = Factory(data)
    factory()
    print(reduce(int.__mul__, factory.outputs[0] + factory.outputs[1] + factory.outputs[2]))
