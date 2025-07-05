import os
import asyncio
import pyidsm.pyidsm as pyidsm


ROOT = os.path.dirname(os.path.dirname(__file__))
TMP = os.path.join(ROOT, 'tmp')
INPUTS = os.path.join(TMP, 'input')


async def main():
    # Add dataset to the store
    await pyidsm.store("yolo", "1", "yolo-1-1", os.path.join(INPUTS, "drone-man-yolo"))
    await pyidsm.store("yolo", "2", "yolo-1-1", os.path.join(INPUTS, "drone-man-yolo"))
    await pyidsm.store("coco", "1", "coco-1-0", os.path.join(INPUTS, "drone-man-coco"))

    # Create a brand new merged set
    await pyidsm.new_merge("mergy", "1", os.path.join(TMP, "mapping.toml"), [
        ("yolo", "1"),
        ("coco-train", "1")
    ])

    # Take a merge set an create a new version adding datasets to it
    await pyidsm.merge_on("mergy", "1", "lol", "2", None, [
        ("yolo", "2"),
    ])

    for sets in await pyidsm.list():
        print(sets.metadata.name, sets.metadata.version)


asyncio.run(main())