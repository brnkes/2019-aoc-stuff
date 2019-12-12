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


def calculate_orbits_q1(objects: Dict):
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


def calculate_orbits_q2(objects: Dict):
    can_reach_from = {}

    def search_upwards(parent_key):
        parent_count = next(can_reach_from[parent_key])
        compute = 1 + parent_count
        while True:
            yield compute

    def search_children(children_of_this_object, target):
        can_reach_from_this_object = 0
        for child_of_this_object in children_of_this_object:
            if target == child_of_this_object:
                can_reach_from_this_object = 1
                break

            can_reach_from_child = next(can_reach_from[child_of_this_object])
            if can_reach_from_child > 0:
                can_reach_from_this_object = can_reach_from_child + 1
                break
        while True:
            yield can_reach_from_this_object

    for key, children in objects.items():
        for child in children:
            can_reach_from[key] = {k: search_children(children, k) for k in ['SAN', 'YOU']}

    diff = set(objects.keys()) - set(can_reach_from.keys())

    for key in diff:
        assert(len(objects.get(key) or set()) == 0), "Incorrect # of leaves"

    return "in progress"

# result = calculate_orbits_q1(process_input())
result = calculate_orbits_q2(process_input())
print(result)
