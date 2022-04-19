import subprocess
import asyncio
import time
from textwrap import dedent


async def run(cmd):
    print(f"running: {cmd}")
    proc = await asyncio.create_subprocess_shell(
        cmd,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE)

    stdout, stderr = await proc.communicate()

    print(f'[{cmd!r} exited with {proc.returncode}]')
    # if stdout:
    #     print(f'[stdout]\n{stdout.decode()}')
    # if stderr:
    #     print(f'[stderr]\n{stderr.decode()}')


async def worker(queue):
    while True:
        # Get a "work item" out of the queue.
        task = await queue.get()

        await task

        # Notify the queue that the "work item" has been processed.
        queue.task_done()


async def main():
    num_iters_per_combo = 30
    batch_size = 10
    # sim_length_days = 100

    queue = asyncio.Queue()
    models = [
        # ("python/synthetic_environments/examples", "isle_of_dogs"),
        # ("python/synthetic_environments/examples", "isle_of_wight"),
        # ("python/synthetic_environments/examples", "greater_manchester"),
        # ("python/synthetic_environments/output", "devon"),
        # ("python/synthetic_environments/output", "wales"),
        # ("python/synthetic_environments/output", "london_s_commuter_ring"),
        ("python/synthetic_environments/output", "cambridge"),
    ]
    time_steps = [
        24,
        # 48,  # half an hour
        # 96,  # quarter of an hour
        # 720,  # two minutes
        # 1440  # minute
    ]
    infection_chances = [
        # 0.1,
        # 0.01,
        0.001,
        # 0.0001,
        # 0.00001,
        # 0.000001
    ]

    sim_lengths = [
        # 15,
        # 30,
        60,
        # 90,
        # 180,
        # 360
    ]

    for env_dir, model_name in models:
        for (idx, sim_length) in enumerate(sim_lengths):
            start_idx = idx * num_iters_per_combo
            for iteration in range(start_idx, start_idx + num_iters_per_combo):
                run_cmd = f'''\
                                .\\target\\release\\outbreak-sim.exe {env_dir} {model_name} {str(iteration)}
                                --time-steps-per-day={24}
                                --sim-length-days={sim_length}
                                --iterations-per-render={24 * sim_length}
                                --seed-infection-chance={0.001}
                                '''
                run_cmd = ' '.join(dedent(run_cmd).splitlines())
                queue.put_nowait(run(run_cmd))

    # for env_dir, model_name in models:
    #     for (ts_idx, time_steps_per_day) in enumerate(time_steps):
    #         for (inf_idx, infection_chance) in enumerate(infection_chances):
    #             start_idx = num_iters_per_combo * (ts_idx * len(infection_chances) + inf_idx)
    #             for iteration in range(start_idx, start_idx + num_iters_per_combo):
    #                 run_cmd = f'''\
    #                                 .\\target\\release\\outbreak-sim.exe {env_dir} {model_name} {str(iteration)}
    #                                 --time-steps-per-day={time_steps_per_day}
    #                                 --sim-length-days={sim_length_days}
    #                                 --iterations-per-render={time_steps_per_day * sim_length_days}
    #                                 --seed-infection-chance={infection_chance}
    #                                '''
    #                 run_cmd = ' '.join(dedent(run_cmd).splitlines())
    #                 queue.put_nowait(run(run_cmd))

    tasks = []
    for _ in range(batch_size):
        task = asyncio.create_task(worker(queue))
        tasks.append(task)

    # Wait until the queue is fully processed.
    started_at = time.monotonic()
    await queue.join()
    total_run_time = time.monotonic() - started_at

    # Cancel our worker tasks.
    for task in tasks:
        task.cancel()
    # Wait until all worker tasks are cancelled.
    await asyncio.gather(*tasks, return_exceptions=True)

    print(
        f"Took {total_run_time} for {num_iters_per_combo} iterations in batches of {batch_size} for the following inputs:")
    print(models)


if __name__ == "__main__":
    asyncio.run(main())
