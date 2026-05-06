from experiment import Experiment
import math
import random


def sample_from_search_space(search_space: dict) -> dict:
    sampled = {}
    for param, dist in search_space.items():
        try:
            dist_type = dist["type"]

            if dist_type == "uniform":
                sampled[param] = random.uniform(float(dist["min"]), float(dist["max"]))

            elif dist_type == "log_uniform":
                log_min = math.log(float(dist["min"]))
                log_max = math.log(float(dist["max"]))
                sampled[param] = math.exp(random.uniform(log_min, log_max))

            elif dist_type == "int_uniform":
                step = int(dist.get("step", 1))
                steps = range(int(dist["min"]), int(dist["max"]) + 1, step)
                sampled[param] = random.choice(list(steps))

            elif dist_type == "categorical":
                sampled[param] = random.choice(dist["values"])

            elif dist_type == "fixed":
                sampled[param] = dist["value"]

            else:
                raise ValueError(f"Unknown distribution type: '{dist_type}'")

        except (KeyError, ValueError, TypeError) as e:
            raise ValueError(f"Error sampling parameter '{param}': {e}")

    return sampled


def seed_population(template: Experiment) -> list[Experiment]:
    population = []
    for _ in range(template.swarm.population_size):
        sampled = sample_from_search_space(template.search_space)
        exp = Experiment(
            meta=template.meta,
            model=template.model,
            training=template.training,
            search_space=template.search_space,
            swarm=template.swarm,
            sampled_params=sampled,
        )
        population.append(exp)
    return population

