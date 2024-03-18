import dataclasses
import os
import subprocess
from subprocess import call
from datetime import datetime


@dataclasses.dataclass
class Engine:
    name: str
    eval_function: str
    depth: int
    use_nega: bool

    def generate_command(self) -> list[str]:
        return [
            "-engine",
            'cmd="/home/Strawby/Schulsachen/Q1/Informatik/facharbeit/target/release/facharbeit"',
            f'name="{self.name}"',
            f'option.EvalFunction={self.eval_function}',
            f'option.Depth={self.depth}',
            f'option.use_negamax={str(self.use_nega).lower()}',
        ]


pgn_path = f"game/python_script/{datetime.utcnow().strftime('%Y%m%d-%H%M%S')}"

eval_functions = [
                  "piece_count",
                  "piece_value",
                  "attacks",
                  "piece_tables",
                  "attacks_diff",
                  "combined"
                  ]
depths = [6]#[1, 2, 3, 4]
concurrency = 16

engines = []
for eval_function in eval_functions:
    for depth in depths:
        for use_nega in [True]: #[False, True]:
            engines.append(Engine(
                f"{eval_function} d{depth} {use_nega=}",
                eval_function,
                depth,
                use_nega
            ))

command = ["cutechess-cli", "-tournament", "round-robin", "-rounds", "1", "-games", "2", "-concurrency",
           f"{concurrency}", "-each", "proto=uci", "tc=inf"]

for engine in engines:
    command.extend(engine.generate_command())

print(" ".join(command))
subprocess.run(["cargo", "+nightly", "build", "--release"])

os.system(" ".join(command))
