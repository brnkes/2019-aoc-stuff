from typing import Set, Dict


def process_input():
    objects = {}

    with open("./input.txt", 'r') as fr:
        for line in fr:
            [center, orbiting] = line.strip().split(")")
            objects[center] = (objects.get(center) or set())
            objects[center].add(orbiting)

    # print(objects)

    return objects


def calculate_orbits(objects: Dict):
    object_orbiting = {}

    def computation(parent_key):
        parent_count = next(object_orbiting[parent_key])
        compute = 1 + parent_count
        while True:
            yield compute

    for key, children in objects.items():
        for child in children:
            object_orbiting[child] = computation(key)

    diff = set(objects.keys()) - set(object_orbiting.keys())

    def root_object_computation():
        while True:
            yield 0

    for non_orbiting in diff:
        object_orbiting[non_orbiting] = root_object_computation()

    return sum([next(compute_orbit) for _, compute_orbit in object_orbiting.items()])


result = calculate_orbits(process_input())
print(result)
