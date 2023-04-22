import time
import websockets
import asyncio


async def test_function():
    print(f"client connected")
    return


async def main():

    async with websockets.serve(test_function, host=None, port=2233, ping_timeout=None):
        print(f"current time is: {time.asctime()}")

    return


if __name__ == "__main__":
    asyncio.run(main(), debug=False)
