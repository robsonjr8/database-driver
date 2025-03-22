import asyncio
import json
import os

from dotenv import load_dotenv

from db_driver.db_driver import execute


def main():
    asyncio.run(fetch())


async def fetch():
    load_dotenv()
    data = await execute(os.environ.get("DB_URI", "db-uri-undefined"), 18)
    print(data)
    print(type(data))
    json_data = json.loads(data.decode("utf-8"))
    print(json_data)
    print(type(json_data))
    print(type(json_data[0]))


if __name__ == "__main__":
    main()
