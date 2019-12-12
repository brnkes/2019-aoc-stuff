'''
It is a six-digit number.
The value is within the range given in your puzzle input.
Two adjacent digits are the same (like 22 in 122345).
Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679)
'''

boundLower = "382345"
boundUpper = "843167"


# todo : bounds
def generate(current, remaining_digit_count, bounds={}, last_adjacent_digit_group=None, adjacency_satisfied=0):
    if remaining_digit_count < 1:
        return 1 if adjacency_satisfied > 0 else 0

    acc = 0
    for next_digit_candidate in range((current and int(current)) or 1, 10):
        new_bounds = {}

        if bounds.get('lower'):
            lower_bound = int(bounds['lower'][0])
            if lower_bound > next_digit_candidate:
                continue
            if lower_bound == next_digit_candidate:
                new_bounds['lower'] = (bounds['lower'][1:])

        if bounds.get('upper'):
            upper_bound = int(bounds['upper'][0])
            if upper_bound < next_digit_candidate:
                continue
            if upper_bound == next_digit_candidate:
                new_bounds['upper'] = (bounds['upper'][1:])

        last_adjacent_digit_group_new = last_adjacent_digit_group
        adjacency_satisfied_new = adjacency_satisfied

        if adjacency_satisfied != 2:
            if next_digit_candidate == current:
                if last_adjacent_digit_group == next_digit_candidate:
                    adjacency_satisfied_new = 0
                else:
                    last_adjacent_digit_group_new = next_digit_candidate
                    adjacency_satisfied_new = 1
            elif adjacency_satisfied == 1:
                adjacency_satisfied_new = 2

        acc += generate(
            next_digit_candidate, remaining_digit_count - 1, new_bounds,
            last_adjacent_digit_group_new, adjacency_satisfied_new
        )

    return acc


def run():
    print(generate(None, 6, {'lower': boundLower, 'upper': boundUpper}))


run()
