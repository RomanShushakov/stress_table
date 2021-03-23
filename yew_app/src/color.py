def proportion(elem_length, elem_range, point_range):
    return elem_length * point_range / elem_range


# def defined_color(a, b, elem_length):
#     elem_range = b - a
#     transition_points = (0, 255, 510, 765, 1020)
#     points_distances = {
#         a: 0,
#     }
#     for i in transition_points:
#         if a < i:
#             point = i
#             point_range = point - a
#             break
#     while point < b:
#         point_dist = proportion(elem_length, elem_range, point_range)
#         points_distances[point] = point_dist
#         point += 255
#         point_range += 255
#     points_distances[b] = elem_length
#     return points_distances


def defined_color(a, b, elem_length):
    elem_range = b - a
    transition_points = (0, 255, 510, 765, 1020)
    points_color = [a]
    points_dist = [0]
    for i in transition_points:
        if a < i:
            point = i
            point_range = point - a
            break
    while point < b:
        point_dist = proportion(elem_length, elem_range, point_range)
        points_color.append(point)
        points_dist.append(point_dist)
        point += 255
        point_range += 255
    points_color.append(b)
    points_dist.append(elem_length)
    return points_color, points_dist


def func(end_points_color, end_points_coord):
    a = end_points_color[0]
    b = end_points_color[1]
    x1 = end_points_coord[0][0]
    x2 = end_points_coord[1][0]
    y1 = end_points_coord[0][1]
    y2 = end_points_coord[1][1]
    z1 = end_points_coord[0][2]
    z2 = end_points_coord[1][2]
    elem_length = ((x2 - x1) ** 2 + (y2 - y1) ** 2 + (z2 - z1) ** 2) ** 0.5
    points_color, points_dist = defined_color(a, b, elem_length)
    point_coord = []
    for point in points_dist:
        x = x1 + (x2 - x1) * point / elem_length
        y = y1 + (y2 - y1) * point / elem_length
        z = z1 + (z2 - z1) * point / elem_length
        point_coord.append([x, y, z])
    return points_color, point_coord


print(func([150, 300], [[0, 0, 0], [100, 100, 100]]))
