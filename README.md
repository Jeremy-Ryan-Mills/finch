<div align="center">
  <img src="assets/logo.png" width="200"/>
</div>

# finch

**The Rust scheduling core for the FINCH swarm system.**

</div>

---

## Overview

The FINCH scheduler is a Rust service responsible for all real-time GPU allocation decisions. It runs a continuous loop, ingests metrics from Python workers, and emits allocation decisions consumed by the Python controller.

---

## Running

```bash
cd scheduler
cargo build --release
cargo run
```

Runs on `0.0.0.0:8000` by default. Logs are written to `../logs/scheduler_{timestamp}.log`.
