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

    def search_children(children_of_this_object, target_object):
        can_reach_from_this_object = None
        for child_of_this_object in children_of_this_object:
            if target == child_of_this_object:
                can_reach_from_this_object = 0
                break

            reachability = can_reach_from.get(child_of_this_object)
            if reachability is None:
                break

            can_reach_from_child = next(reachability[target_object])
            if can_reach_from_child is not None:
                can_reach_from_this_object = can_reach_from_child + 1
                break
        while True:
            yield can_reach_from_this_object

    for key, children in objects.items():
        can_reach_from[key] = {k: search_children(children, k) for k in ['SAN', 'YOU']}

    minimum_so_far = None
    for object_in_center, target_gens in can_reach_from.items():
        total = None
        for target, gen in target_gens.items():
            required_steps = next(gen)
            if required_steps is None:
                total = None
                break
            total = total + required_steps if total else required_steps
        if total is not None:
            if minimum_so_far is None:
                minimum_so_far = total
            else:
                minimum_so_far = min(total, minimum_so_far)

    return minimum_so_far


# result = calculate_orbits_q1(process_input())
result = calculate_orbits_q2(process_input())
print(result)
