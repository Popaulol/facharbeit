import dataclasses
import subprocess
from subprocess import call
from datetime import datetime


@dataclasses.dataclass
class Engine:
    name: str
    eval_function: str
    depth: int

    def generate_command(self) -> list[str]:
        return [
            "-engine",
            'cmd="/home/Strawby/Schulsachen/Q1/Informatik/facharbeit/target/release/facharbeit"',
            f'name="{self.name}"',
            f'option.EvalFunction={self.eval_function}',
            f'option.Depth={self.depth}',
        ]


pgn_path = f"game/python_script/{datetime.utcnow().strftime('%Y%m%d-%H%M%S')}"

eval_functions = ["piece_count", "piece_value", "attacks", "pawn_pos"]
depths = [0, 1, 2, 3, 4, 5]
concurrency = 16

engines = []
for eval_function in eval_functions:
    for depth in depths:
        engines.append(Engine(
            f"{eval_function} d{depth}",
            eval_function,
            depth
        ))

command = ["cutechess-cli", "-tournament", "round-robin", "-rounds", "1", "-games", "2", "-concurrency", f"{concurrency}", "-each", "proto=uci", "tc=inf"]

for engine in engines:
    command.extend(engine.generate_command())
subprocess.call(["cargo", "+nightly", "build", "--release"], )

subprocess.call(command)
