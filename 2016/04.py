from utils import lines, read_data
from collections import Counter


def decrypt_word(word, offset):
    return ''.join(chr((ord(c) - ord('a') + offset) % 26 + ord('a')) for c in word)


def decrypt_room_name(room):
    name, sector, rest = split_room_name(room)
    return (' '.join(decrypt_word(word, int(sector)) for word in name.split('-')), int(sector))


def split_room_name(room):
    name, rest = room.rsplit('-', 1)
    sector, rest = rest.split('[')
    return name, sector, rest


def is_real(room):
    name, sector, rest = split_room_name(room)
    checksum = set(rest[:-1])
    counts = Counter(name.replace('-', ''))
    sorted_counts = sorted(counts.items(), key=lambda x: (-x[1], x[0]))
    most_common = {a for a, b in sorted_counts[:5]}
    return most_common == checksum


def total_real_sectors(data):
    total = 0
    for room in lines(data):
        if is_real(room):
            total += int(split_room_name(room)[1])
    return total


def find_north_pole_sector(data):
    for room in lines(data):
        name, sector = decrypt_room_name(room)
        if 'north' in name:
            return sector


if __name__ == '__main__':
    data = read_data(4)

    assert total_real_sectors(
        """
        aaaaa-bbb-z-y-x-123[abxyz]
        a-b-c-d-e-f-g-h-987[abcde]
        not-a-real-room-404[oarel]
        totally-real-room-200[decoy]
        """
    ) == 1514
    assert decrypt_room_name("qzmt-zixmtkozy-ivhz-343[ihzmt]") == ('very encrypted name', 343)
    print('All tests passed')

    print(total_real_sectors(data))
    print(find_north_pole_sector(data))
