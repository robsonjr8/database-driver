import asyncio
import json

from db_driver import execute


def main():
    loop = asyncio.get_event_loop()  # TODO: DeprecationWarning: There is no current event loop
    loop.run_until_complete(create_tasks_func())
    loop.close()


async def fetch():
    data = await execute("mysql://user:password@host:port/db", 18)
    print(data)
    print(type(data))
    json_data = json.loads(data.decode("utf-8"))
    print(json_data)
    print(type(json_data))
    print(type(json_data[0]))


async def create_tasks_func():
    tasks = [asyncio.create_task(fetch())]
    await asyncio.wait(tasks)


if __name__ == "__main__":
    main()
