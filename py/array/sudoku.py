



region_size = 3


l = [ [
    [a, b]
    for a in range(region_size * I, region_size * (I + 1))
    for b in range(region_size * J, region_size * (J + 1))
] for I in range(region_size) for J in range(region_size)]


print(l)