from pathlib import Path

from experiment import Experiment, experiment_from_json, experiment_from_yaml
from population import seed_population

class Controller():
    
    def __init__(self):
        self.experiments: list[Experiment] = []
        self.gpus = []

    def load_experiment(self, path: str):
        p = Path(path)
        if not p.exists():
            raise FileNotFoundError(f"Config file not found: {p}")
        if p.suffix == ".json":
            template = experiment_from_json(p)
        elif p.suffix in (".yaml", ".yml"):
            template = experiment_from_yaml(p)
        else:
            raise ValueError(f"Unsupported file type: {p.suffix}")
        self.experiments = seed_population(template)
