import time
import asyncio
import sys
import subprocess
import pkg_resources


async def main():
    print("builtin modules:")
    [print(item) for item in sys.builtin_module_names]
    print()

    print(sys.flags)
    print()

    print("base prefix:")
    print(sys.base_prefix)
    print()

    print("meta path:")
    [print(item) for item in sys.meta_path]
    print()

    print("path")
    [print(item) for item in sys.path]
    print()

    [print(item) for item in sorted(pkg_resources.working_set)]
    print()

    print(f"sys.executable: {sys.executable}")
    print(f"sys.exec_prefix: {sys.exec_prefix}")
    subprocess.run(["whoami"])
    subprocess.run(["pwd"])
    subprocess.run(["ls", "-alh", "./python/build"])
    print()

    return


if __name__ == "__main__":
    asyncio.run(main(), debug=False)
