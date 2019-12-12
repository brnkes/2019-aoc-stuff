'''
It is a six-digit number.
The value is within the range given in your puzzle input.
Two adjacent digits are the same (like 22 in 122345).
Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679)
'''

boundLower = "382345"
boundUpper = "843167"


# todo : bounds
def generate(current, remaining_digit_count, bounds={}, had_adjacent=False):
    if remaining_digit_count < 1:
        return 1 if had_adjacent else 0

    acc = 0
    for k in range(int(current), 10):
        new_bounds = {}

        if bounds.get('lower'):
            lower_bound = int(bounds['lower'][0])
            if lower_bound > k:
                continue
            if lower_bound == k:
                new_bounds['lower'] = (bounds['lower'][1:])

        if bounds.get('upper'):
            upper_bound = int(bounds['upper'][0])
            if upper_bound < k:
                continue
            if upper_bound == k:
                new_bounds['upper'] = (bounds['upper'][1:])

        acc += generate(k, remaining_digit_count - 1, new_bounds, had_adjacent or k == current)

    return acc


def run():
    print(generate(1, 6, {'lower': boundLower, 'upper': boundUpper}))


run()
