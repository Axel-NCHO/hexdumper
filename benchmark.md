## Performance

Benchmarked on Fedora Linux using a 20 MiB file (`data/random.bin`), with output redirected to `/dev/null`. The file was generated using the following command: 

```shell dd if=/dev/urandom of=random.bin bs=1M count=20 status=progress```  

Results are from 10 runs after 2 warmup runs, measured with [`hyperfine`](https://github.com/sharkdp/hyperfine).

| Command | Mean runtime | Standard deviation |
|---|---:|---:|
| `hexdumper -C` | 422.5 ms | ± 5.3 ms |
| `hexdump -C` | 2.325 s | ± 6 ms |

**`hexdumper` is 5.50× faster** than `hexdump` in this benchmark.

The memory use is theorically constant for both tools.  

The runtime performance improvements come from buffered I/O, reusable formatting buffers, and batching output.

*Benchmark results are specific to this system and workload; actual performance may vary depending on the hardware, file size, and output destination.*
