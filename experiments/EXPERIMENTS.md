# Experiment Configuration Reference
## FINCH — Experiment Definition Files

Experiments in FINCH are defined as `.yaml` or `.json` files. Both formats are fully supported and share an identical schema. YAML is recommended for hand-authored configs due to readability; JSON is useful for programmatically generated experiments.

---

## File Format

```
experiments/
├── resnet_search.yaml
├── transformer_search.json
└── search_spaces.py        # optional: shared search space definitions
```

Load either format the same way:

```bash
finch run experiments/resnet_search.yaml
finch run experiments/transformer_search.json
```

---

## Top-Level Structure

Every config file has five required top-level sections:

```yaml
experiment:   # identity and metadata
model:        # architecture definition
training:     # dataset, optimizer, loss
search_space: # hyperparameters to search over
swarm:        # finch-specific scheduling and evolution settings
```

---

## Section: `experiment`

Identity and metadata for this experiment definition.

```yaml
experiment:
  name: resnet-depth-search
  version: 1
  description: "Search over ResNet depth and learning rate"
  tags:
    - vision
    - cifar10
```

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | string | ✔ | Unique name for this experiment family. Used in logs, dashboard, and lineage graph. |
| `version` | integer | ✔ | Increment when making breaking changes to the config. |
| `description` | string | ✘ | Human-readable summary. |
| `tags` | list[string] | ✘ | Arbitrary labels for filtering in the dashboard. |

---

## Section: `model`

Defines the model architecture and its fixed (non-searched) parameters.

```yaml
model:
  architecture: resnet
  params:
    depth: 34
    num_classes: 10
    dropout: 0.2
```

| Field | Type | Required | Description |
|---|---|---|---|
| `architecture` | string | ✔ | Architecture identifier. Must match a registered model in `worker/trainer.py`. |
| `params` | object | ✘ | Fixed model parameters. Any key here that also appears in `search_space` will be overridden at runtime by the sampled value. |

### Supported Architectures

| Value | Description |
|---|---|
| `resnet` | ResNet family (depth controlled via `params.depth`) |
| `transformer` | Encoder-only transformer |
| `mlp` | Fully connected network |
| `custom` | User-defined model loaded from `params.module` path |

> To register a new architecture, add it to `worker/trainer.py` under the `ARCHITECTURE_REGISTRY` dict.

---

## Section: `training`

Controls the training loop — dataset, optimizer, loss function, and hardware target.

```yaml
training:
  dataset: cifar10
  epochs: 90
  batch_size: 128
  optimizer: adamw
  loss: cross_entropy
  device: cuda
  num_workers: 4
  mixed_precision: true
```

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `dataset` | string | ✔ | — | Dataset identifier. See supported datasets below. |
| `epochs` | integer | ✔ | — | Maximum training epochs. Pruning may stop experiments before this. |
| `batch_size` | integer | ✔ | — | Batch size. Can also be defined in `search_space` to search over it. |
| `optimizer` | string | ✔ | — | Optimizer to use. See supported optimizers below. |
| `loss` | string | ✔ | — | Loss function. See supported losses below. |
| `device` | string | ✘ | `cuda` | `cuda` or `cpu`. Workers ignore this and use their assigned GPU. |
| `num_workers` | integer | ✘ | `4` | DataLoader worker processes. |
| `mixed_precision` | boolean | ✘ | `false` | Enable `torch.cuda.amp` for faster training on supported GPUs. |

### Supported Datasets

| Value | Description |
|---|---|
| `cifar10` | CIFAR-10 (32×32, 10 classes) |
| `cifar100` | CIFAR-100 (32×32, 100 classes) |
| `imagenet` | ImageNet (requires local path set in `config/paths.yaml`) |
| `mnist` | MNIST (28×28, 10 classes) |
| `custom` | User-defined dataset loaded from `training.dataset_path` |

### Supported Optimizers

| Value | Description |
|---|---|
| `adam` | Adam |
| `adamw` | AdamW (Adam with decoupled weight decay) |
| `sgd` | SGD with optional momentum |
| `rmsprop` | RMSProp |

### Supported Loss Functions

| Value | Description |
|---|---|
| `cross_entropy` | Standard cross-entropy (classification) |
| `mse` | Mean squared error (regression) |
| `bce` | Binary cross-entropy |
| `nll` | Negative log-likelihood |

---

## Section: `search_space`

Defines which hyperparameters FINCH will search over and how to sample them. Each key is a hyperparameter name; the value defines its distribution.

```yaml
search_space:
  learning_rate:
    type: log_uniform
    min: 1e-4
    max: 1e-1
  depth:
    type: categorical
    values: [18, 34, 50, 101]
  dropout:
    type: uniform
    min: 0.1
    max: 0.5
  weight_decay:
    type: log_uniform
    min: 1e-6
    max: 1e-2
```

Any key defined here will override the matching key in `model.params` or `training` at runtime.

---

### Distribution Types

#### `uniform`
Sample a float uniformly between `min` and `max`.

```yaml
dropout:
  type: uniform
  min: 0.1
  max: 0.5
```

| Field | Type | Required | Description |
|---|---|---|---|
| `min` | float | ✔ | Lower bound (inclusive) |
| `max` | float | ✔ | Upper bound (inclusive) |

Use for: dropout rate, momentum, gradient clip value.

---

#### `log_uniform`
Sample a float log-uniformly between `min` and `max`. Equivalent to sampling uniformly in log space, then exponentiating.

```yaml
learning_rate:
  type: log_uniform
  min: 1e-4
  max: 1e-1
```

| Field | Type | Required | Description |
|---|---|---|---|
| `min` | float | ✔ | Lower bound |
| `max` | float | ✔ | Upper bound |

Use for: learning rate, weight decay, epsilon values. Prefer this over `uniform` whenever the parameter spans multiple orders of magnitude.

---

#### `int_uniform`
Sample an integer uniformly between `min` and `max`.

```yaml
hidden_dim:
  type: int_uniform
  min: 64
  max: 512
  step: 64
```

| Field | Type | Required | Description |
|---|---|---|---|
| `min` | integer | ✔ | Lower bound (inclusive) |
| `max` | integer | ✔ | Upper bound (inclusive) |
| `step` | integer | ✘ | Snap to multiples of this value. Useful for dimensions that must be divisible by 64. |

Use for: hidden layer sizes, number of heads, number of layers.

---

#### `categorical`
Sample uniformly from a fixed list of values. Values can be strings, integers, or floats.

```yaml
depth:
  type: categorical
  values: [18, 34, 50, 101]

optimizer:
  type: categorical
  values: [adam, adamw, sgd]
```

| Field | Type | Required | Description |
|---|---|---|---|
| `values` | list | ✔ | List of options to sample from. |

Use for: architecture variants, optimizer choice, activation functions, boolean flags.

---

#### `fixed`
Pin a value without searching over it. Useful when you want a parameter to appear in the experiment record without being part of the search.

```yaml
num_classes:
  type: fixed
  value: 10
```

| Field | Type | Required | Description |
|---|---|---|---|
| `value` | any | ✔ | The fixed value. |

---

## Section: `swarm`

Controls how FINCH manages this experiment family — scheduling policy, mutation, pruning, and time-slicing.

```yaml
swarm:
  population_size: 12
  scheduler: ucb
  ucb_c: 1.4
  mutation_rate: 0.2
  time_slice_steps: 200
  report_interval: 50
  pruning:
    enabled: true
    strategy: asha
    brackets: [100, 300, 900]
    kill_fraction: 0.3
```

### `swarm` Fields

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `population_size` | integer | ✔ | — | Number of experiments alive at any time. As experiments are pruned, mutations fill the gap up to this limit. |
| `scheduler` | string | ✔ | — | Bandit algorithm used by the Rust scheduler engine. See below. |
| `ucb_c` | float | ✘ | `1.4` | Exploration constant for UCB. Higher = more exploration. Only used when `scheduler: ucb`. |
| `mutation_rate` | float | ✘ | `0.2` | Fraction of a hyperparameter's range applied during mutation. `0.2` means ±20% perturbation. |
| `time_slice_steps` | integer | ✔ | — | Number of training steps each experiment runs per GPU time slice before potentially being preempted. |
| `report_interval` | integer | ✘ | `50` | Steps between metric reports sent from worker to Rust scheduler. |

### Supported Schedulers

| Value | Description |
|---|---|
| `ucb` | Upper Confidence Bound. Balances exploitation of high performers with exploration of uncertain experiments. Controlled by `ucb_c`. |
| `thompson` | Thompson Sampling. Samples from a posterior distribution over experiment rewards. More aggressive exploration than UCB early on. |
| `random` | Uniform random allocation. Useful as a baseline to verify the bandit is providing value. |
| `exploit` | Always allocates to the current top performers. No exploration. Use for final-stage refinement only. |

---

### `swarm.pruning`

Controls the early stopping policy applied by the Python controller.

```yaml
pruning:
  enabled: true
  strategy: asha
  brackets: [100, 300, 900]
  kill_fraction: 0.3
```

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `enabled` | boolean | ✘ | `true` | Toggle pruning on or off. |
| `strategy` | string | ✘ | `asha` | Pruning strategy. See below. |
| `brackets` | list[integer] | ✔ (if enabled) | — | Step counts at which pruning is evaluated. Experiments must have reached a bracket to be eligible for pruning at it. |
| `kill_fraction` | float | ✘ | `0.3` | Fraction of eligible experiments to prune at each bracket. `0.3` kills the bottom 30%. |

### Supported Pruning Strategies

| Value | Description |
|---|---|
| `asha` | Asynchronous Successive Halving. Evaluates experiments at each bracket and eliminates the bottom `kill_fraction`. Default and recommended. |
| `median` | Kills any experiment performing below the median at a bracket. |
| `none` | No pruning. All experiments run to completion. |

---

## Full Example — YAML

```yaml
experiment:
  name: resnet-depth-search
  version: 1
  description: "Search over ResNet depth and learning rate on CIFAR-10"
  tags:
    - vision
    - cifar10

model:
  architecture: resnet
  params:
    depth: 34
    num_classes: 10
    dropout: 0.2

training:
  dataset: cifar10
  epochs: 90
  batch_size: 128
  optimizer: adamw
  loss: cross_entropy
  mixed_precision: true
  num_workers: 4

search_space:
  learning_rate:
    type: log_uniform
    min: 1e-4
    max: 1e-1
  depth:
    type: categorical
    values: [18, 34, 50, 101]
  dropout:
    type: uniform
    min: 0.1
    max: 0.5
  weight_decay:
    type: log_uniform
    min: 1e-6
    max: 1e-2

swarm:
  population_size: 12
  scheduler: ucb
  ucb_c: 1.4
  mutation_rate: 0.2
  time_slice_steps: 200
  report_interval: 50
  pruning:
    enabled: true
    strategy: asha
    brackets: [100, 300, 900]
    kill_fraction: 0.3
```

---

## Full Example — JSON

```json
{
  "experiment": {
    "name": "resnet-depth-search",
    "version": 1,
    "description": "Search over ResNet depth and learning rate on CIFAR-10",
    "tags": ["vision", "cifar10"]
  },
  "model": {
    "architecture": "resnet",
    "params": {
      "depth": 34,
      "num_classes": 10,
      "dropout": 0.2
    }
  },
  "training": {
    "dataset": "cifar10",
    "epochs": 90,
    "batch_size": 128,
    "optimizer": "adamw",
    "loss": "cross_entropy",
    "mixed_precision": true,
    "num_workers": 4
  },
  "search_space": {
    "learning_rate": {
      "type": "log_uniform",
      "min": 1e-4,
      "max": 1e-1
    },
    "depth": {
      "type": "categorical",
      "values": [18, 34, 50, 101]
    },
    "dropout": {
      "type": "uniform",
      "min": 0.1,
      "max": 0.5
    },
    "weight_decay": {
      "type": "log_uniform",
      "min": 1e-6,
      "max": 1e-2
    }
  },
  "swarm": {
    "population_size": 12,
    "scheduler": "ucb",
    "ucb_c": 1.4,
    "mutation_rate": 0.2,
    "time_slice_steps": 200,
    "report_interval": 50,
    "pruning": {
      "enabled": true,
      "strategy": "asha",
      "brackets": [100, 300, 900],
      "kill_fraction": 0.3
    }
  }
}
```

---

## Common Mistakes

**Searching over a parameter that is also fixed in `model.params`**
The `search_space` value always wins. If `depth: 34` is in `model.params` and `depth` is also in `search_space`, the sampled value will be used — the fixed value is ignored.

**Setting `brackets` beyond `epochs`**
If your final bracket is step 900 but `epochs` is set such that most experiments never reach 900 steps, pruning at that bracket will never fire. Make sure brackets are reachable within your training budget.

**`kill_fraction` too high early**
A `kill_fraction` of `0.5` at the first bracket means half your population is gone before the bandit has meaningful signal. Start conservative (`0.3`) and increase at later brackets.

**`time_slice_steps` larger than `report_interval`**
The Rust scheduler needs metric reports to update bandit scores. If `time_slice_steps: 200` and `report_interval: 500`, the scheduler makes allocation decisions with stale data. Always keep `report_interval` ≤ `time_slice_steps`.
