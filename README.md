> WIP

## A bare-bones instrumentation profiler for quick and dirty profiling

## Usage

```rust
use profilum::*;

define_slots!(Task {
    ReadData,
    ProcessData,
    OutputData
});


fn main() -> std::io::Result<()> {
    start_profiler!();

    // wrap each block to profile 

    let data = profile!(Task::ReadFile, { std::fs::read(FILE_PATH)? });

    let numbers_count = profile!(Task::ProcessData, { 
        data
            .lines()
            .filter_map(|line| {
                line.ok()
                    .and_then(|line| line.parse::<usize>().ok())
            })
            .count()
    });

    profile!(Task::OutputData, {
        std::fs::write("./output", format!("{}", valid_num_counts))?;
    })

    end_profiler!();

    // output measurements per task
    println!(get_profiler_metrics!());
}

```


## Example output

```
======Profiler Metrics======
Total time: 3.46, CPU Freq: 399999900
Total elapsed: 13840760140
Slots:
    LookupAndConvert        : (25.25%), hits = 1, elapsed = 3494913208
    ParseHaversinePairs     : (97.02%), hits = 1, elapsed = 13428869368
    ReadFile                : (0.62%), hits = 1, elapsed = 86255344
    ComputeHaversine        : (2.35%), hits = 1, elapsed = 325592432
======================
```
