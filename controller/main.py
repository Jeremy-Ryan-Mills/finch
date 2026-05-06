import os
import json
import yaml
import sys
from pathlib import Path
import requests

from controller import Controller


def main():
    if len(sys.argv) < 2:
        print("Usage: finch <experiment_path>")
        sys.exit(1)

    # find experiment file and load experiment variations
    experiment_path = sys.argv[1]
    controller = Controller()
    controller.load_experiment(experiment_path)

    # connect to scheduler on port 8000
    url = "http://localhost:8000/allocations"
    response = requests.get(url)
    if response.status_code == 200:
        print(response.json())

if __name__ == "__main__":
    main()
