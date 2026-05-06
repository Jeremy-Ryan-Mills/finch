from __future__ import annotations

import json
import uuid
from pathlib import Path
from typing import Any

import yaml
from pydantic import BaseModel, Field


class ExperimentMeta(BaseModel):
    name: str
    version: int
    description: str = ""
    tags: list[str] = Field(default_factory=list)


class ModelConfig(BaseModel):
    architecture: str
    params: dict[str, Any] = Field(default_factory=dict)


class TrainingConfig(BaseModel):
    dataset: str
    epochs: int
    batch_size: int
    optimizer: str
    loss: str
    device: str = "cuda"
    num_workers: int = 4
    mixed_precision: bool = False
    dataset_path: str | None = None


class PruningConfig(BaseModel):
    enabled: bool = True
    strategy: str = "asha"
    brackets: list[int] = Field(default_factory=list)
    kill_fraction: float = 0.3


class SwarmConfig(BaseModel):
    population_size: int
    scheduler: str
    ucb_c: float = 1.4
    mutation_rate: float = 0.2
    time_slice_steps: int = 200
    report_interval: int = 50
    pruning: PruningConfig = Field(default_factory=PruningConfig)


class Experiment(BaseModel):
    id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    meta: ExperimentMeta
    model: ModelConfig
    training: TrainingConfig
    search_space: dict[str, Any] = Field(default_factory=dict)
    swarm: SwarmConfig


def experiment_from_dict(data: dict) -> Experiment:
    return Experiment(**data)


def experiment_from_yaml(path: str | Path) -> Experiment:
    raw = Path(path).read_text(encoding="utf-8")
    return experiment_from_dict(yaml.safe_load(raw))


def experiment_from_json(path: str | Path) -> Experiment:
    raw = Path(path).read_text(encoding="utf-8")
    return experiment_from_dict(json.loads(raw))