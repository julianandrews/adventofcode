import fileinput


def p1(num_recipes, initial_scores):
    scores = initial_scores[:]
    elves = [0, 1]
    while len(scores) < num_recipes + 10:
        current_scores= [scores[elf] for elf in elves]
        scores += [int(c) for c in str(sum(current_scores))]
        elves = [(elf + 1 + score) % len(scores) for (elf, score) in zip(elves, current_scores)]

    return ''.join(str(x) for x in scores[num_recipes: num_recipes + 10])


def p2(target_string, initial_scores):
    target_scores = [int(c) for c in target_string]
    scores = initial_scores[:]
    elves = [0, 1]
    while True:
        current_scores = [scores[elf] for elf in elves]
        for score in (int(c) for c in str(sum(current_scores))):
            scores.append(score)
            if scores[-len(target_scores):] == target_scores:
                return len(scores) - len(target_scores)

        elves = [(elf + 1 + score) % len(scores) for (elf, score) in zip(elves, current_scores)]

if __name__ == "__main__":
    data = next(iter(fileinput.input())).strip()
    print("Part 1: %s" % p1(int(data), [3, 7]))
    print("Part 2: %s" % p2(data, [3, 7]))
