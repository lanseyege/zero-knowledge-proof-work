
p = 1160777405615148683 
q = 2321554811230297367  
g = 2307294256159045561 
h = 1328753870933556460 

y = 1074275676273622917
#y = y % (p-1)

vec_h = [1009675229564370393, 783239516282340099, 518328600787762545, 2277864756348908247, 1807296506365810685, 836110136633941117, 60393740024100466, 936385913323286173]

print(pow(g, p, q))

def get_vec_hi(vec_h, y, lens , p, q):
    vec_hi = [vec_h[0]]
    #y = y % p
    for i in range(1, lens):
        #a = pow(_y, -1, p)
        c = pow(y, i, p)
        a = pow(c, -1, p)
        #print("a y : " + str(a * _y %p))
        b = pow(vec_h[i], a, q)
        vec_hi.append(b)
        #a = pow(vec_h[i], _y, p)
        #vec_hi.append(pow(a, -1, p))
        #_y *= y
        #_y %= p1
    return vec_hi

def get_vec_hi2(vec_h, y, lens , p, q):
    vec_hi = [vec_h[0]]
    _y = y 
    for i in range(1, lens):
        a = pow(_y, -1, p)
        b = pow(vec_hi[i], a, q)
        vec_hi.append(b)
        _y *= y
        #_y %= p1
    return vec_hi

def rever(vec_hi, y, lens, p, q):
    vec_rev = [vec_hi[0]]
    _y = y
    for i in range(1, lens):
        vec_rev.append(pow(vec_hi[i], _y, q))
        _y *= y
    return vec_rev

print(vec_h)

vec_hi1 = get_vec_hi(vec_h, y, 8, p, q)
vec_hi2 = get_vec_hi(vec_h, y, 8, p, q)

print(vec_hi1)
print(vec_hi2)

vec_rev1 = rever(vec_hi1, y, 8, p, q)
vec_rev2 = rever(vec_hi2, y, 8, p, q)

print(vec_rev1)
print(vec_rev2)

