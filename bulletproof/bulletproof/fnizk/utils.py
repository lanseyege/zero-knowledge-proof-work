import gmpy2

def get_msg_phug(_p, _u, _vec_g, _vec_h):
    msg = ""
    msg += "".join(hex(l)[2:] for l in _vec_g)
    msg += "".join(hex(l)[2:] for l in _vec_h)
    msg += hex(_p)[2:]
    msg += hex(_u)[2:]
    return msg


def get_delta(z, y, lens, p):
    z2 = z * z #% p
    z3 = z2 * z #% p
    _y = 1
    _tw = 1
    res1 = 0
    res2 = 0
    for i in range(lens):
        res1 = res1 + _y
        res2 = res2 + _tw

        _y = _y * y #% p
        _tw *= 2
    return ((z-z2) * res1 - z3 * res2) % p
    

def get_lx(al, SL, x, z, lens, p):
    res = [0] * lens
    for i in range(lens):
        res[i] = (al[i] - z + SL[i] * x) % p
    return res

def get_rx(ar, SR, x, y, z, lens, p):
    res = [0] * lens
    _y = 1
    _two = 1
    for i in range(lens):
        res[i] = (_y * (ar[i] + z + SR[i] * x)%p + (z * z)%p  * _two) % p
        _y = _y * y % p
        _two *= 2
    return res

def _get_rx(ar, SR, x, y, z, lens, p):
    res = [0] * lens
    _y = 1
    _two = 1
    rl = [0] * lens
    for i in range(lens):
        rl[i] = ar[i] + z + SR[i] * x

    for i in range(lens):
        res[i] = (_y * rl[i] + z*z * _two) % p

        _y = _y * y % p
        _two = _two * 2
    return res
    

def get_t1(al, ar, SL, SR, y, z, lens, p):
    res = 0
    _y = 1
    _tw = 1
    for i in range(lens):
        res += ((al[i] - z) * (_y * SR[i]) + SL[i] *( _y * (ar[i] + z) + z*z*_tw)) % p

        _y = (_y * y) % p
        _tw *= 2

    return res % p

def _get_t1(al, ar, SL, SR, y, z, lens, p):
    res = 0
    _y = 1
    _tw = 1
    temp1 = [0] * lens
    temp2 = [0] * lens
    temp3 = [0] * lens
    temp4 = [0] * lens
    for i in range(lens):
        temp1[i] = al[i] - z
        temp2[i] = _y * SR[i]
        temp3[i] = ar[i] + z
        temp4[i] = _y * temp3[i] + z*z *_tw
        #res += ((al[i] - z) * (_y * SR[i]) + SL[i] * _y * (ar[i] + z)) % p
        _y = _y * y % p
        _tw *= 2
    
    for i in range(lens):
        res += temp1[i] * temp2[i]  % p
        res += SL[i] * temp4[i]  % p
    return res  % p

def get_t2(SL, SR, y, lens, p):
    res = 0
    _y = 1
    for i in range(lens):
        res += SL[i] * _y * SR[i] % p
        _y = _y * y % p
    return res % p

def hadamard(left, start1, right, start2, lens):
    res = []
    for i in range(lens):
        res.append(left[i+start1] * right[i+start2] % p)
    return res

def inner_product(left, start1, right, start2, lens, p):
    res = 0
    for i in range(lens):
        res += left[i+start1] * right[i+start2] 
    return res % p

def commit_s(base1, inx1, base2, inx2, p):
    res1 = pow(base1, inx1, p)
    res2 = pow(base2, inx2, p)
    return res1 * res2 % p

def commit_v(vec1, inx1, vec2, inx2, lens, p) :
    res = 1
    for i in range(lens):
        res = res * pow(vec1[i], inx1[i], p)
        res = res * pow(vec2[i], inx2[i], p)
        res = res % p
    return res

def get_vec_hi(vec_h, y, lens , p , q):
    vec_hi = [vec_h[0]]
    #y = y % p
    for i in range(1, lens):
        c = pow(y, i, p)
        a = pow(c, -1, p)
        b = pow(vec_h[i], a, q)
        vec_hi.append(b)
    return vec_hi

def get_hp():
    _y = 1
    _tw = 1
    _z = z*z % p
    for i in range(lens):
        z * y

def get_vec_temp(y,z, lens, p):
    vec_temp = []
    _y = 1
    _tw = 1
    for i in range(lens):
        vec_temp.append(z *_y + z*z * _tw)
        _y *= y
        #_y %= p2
        _tw *= 2
    return vec_temp 


