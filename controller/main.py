import os
import json
import yaml
import sys
from pathlib import Path

from controller import Controller


def main():
    if len(sys.argv) < 2:
        print("Usage: finch <experiment_path>")
        sys.exit(1)

    experiment_path = sys.argv[1]
    controller = Controller()
    controller.load_experiment(experiment_path)