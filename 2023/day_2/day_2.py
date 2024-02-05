import re
from pathlib import Path


PLAYS_PAT = re.compile(";|:")
GAME_ID_PAT = re.compile(r"Game (\d+)")
MAX_PLAY = {
    "red": 12,
    "green": 13,
    "blue": 14
}


class Game:
    def __init__(self, /, game_string: str):
        self.game_string = game_string.strip()
        game_id_string, *plays = PLAYS_PAT.split(self.game_string)
        self._id = int(GAME_ID_PAT.findall(game_id_string)[0])
        self.plays = [Play(play) for play in plays]

    def is_valid(self):
        return all(all(count <= MAX_PLAY[color] for color, count in play.items()) for play in self.plays)

    def __repr__(self) -> str:
        return f"Game(id={self._id})"

    def sum_of_power_sets(self):
        max_r, max_g, max_b = 1, 1, 1
        for play in self.plays:
            max_r = max(max_r, play.max("red"))
            max_g = max(max_g, play.max("green"))
            max_b = max(max_b, play.max("blue"))
        return max_r * max_g * max_b


class Play:
    def __init__(self, /, play_string: str):
        self.play_string = play_string.strip()
        self.play = {y: int(x) for x, y in map(str.split, (x.strip() for x in self.play_string.split(",")))}

    def items(self):
        return self.play.items()

    def __repr__(self) -> str:
        return f"Play({self.play})"

    def max(self, /, color: str):
        return self.play.get(color, 0)

if __name__ == '__main__':
    games = (Path("inputs") / "test_1.txt").read_text().splitlines()
    part_a_score = sum(g._id for g in (Game(game) for game in games) if g.is_valid())
    print("A: ", part_a_score)

    part_b_score = sum(g.sum_of_power_sets() for g in (Game(game) for game in games))
    print("B: ", part_b_score)
