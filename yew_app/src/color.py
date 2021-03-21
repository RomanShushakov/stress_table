def proportion(elem_length, elem_range, point_range):
    return elem_length * point_range / elem_range


def defined_color(a, b, elem_length):
    elem_range = b - a
    transition_points = (0, 255, 510, 765, 1020)
    points_distances = {
        a: 0,
    }
    for i in range(5):
        if a < transition_points[i]:
            point = transition_points[i]
            point_range = point - a
            break
    while point < b:
        point_dist = proportion(elem_length, elem_range, point_range)
        points_distances[point] = point_dist
        point += 255
        point_range += 255
    points_distances[b] = elem_length
    return points_distances


print(defined_color(150, 800, 100))



