import subprocess
import asyncio
import time

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
    num_iterations = 20
    batch_size = 10

    queue = asyncio.Queue()
    models = [
        ("python/synthetic_environments/examples", "isle_of_dogs"),
        ("python/synthetic_environments/examples", "isle_of_wight"),
        ("python/synthetic_environments/examples", "greater_manchester"),
        ("python/synthetic_environments/output", "devon"),
        ("python/synthetic_environments/output", "wales"),
        ("python/synthetic_environments/output", "london_se_commuter_ring"),
    ]

    for env_dir, model_name in models:
        for iteration in range(num_iterations):
            queue.put_nowait(
                run(f'.\\target\\release\\outbreak-sim.exe {env_dir} {model_name} {str(iteration)}'))

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

    print(f"Took {total_run_time} for {num_iterations} iterations in batches of {batch_size} for the following inputs:")
    print(models)

if __name__ == "__main__":
    asyncio.run(main())
