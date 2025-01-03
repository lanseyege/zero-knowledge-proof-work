


p = 2321554811230297367
q = 1160777405615148683
g = 949614091546416552
h = 1466963809756874025

print(p)
print(g)
print(h)
print()

a = pow(h, 3 * p + 2, p)
print(a)

a = pow(h, (3 * p + 2) % p, p)
print(a)

a = pow(h, (3 * p + 2) % (p-1), p)
print(a)



b = pow(h, 2, p)
print(b)

b = pow(h, 2 * p -2, p)
print(b)

b = pow(h, q, p)
print(b)
