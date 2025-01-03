
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
        print("al - z: " + str(al[i] - z))
        print("_y * SR: " + str(_y * SR[i]))
        print("ar + z: " + str(ar[i] + z))
        print("SL * _y * -" + str(SL[i] * _y * (ar[i] + z)))
        print("temp r : " + str(res%p))
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
    return res #% p

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

if __name__ == "__main__":

    v = 44
    lens = 6

    p = 2321554811230297367
    q = 1160777405615148683
    g = 949614091546416552
    h = 1466963809756874025

    vec_g = [936006576136153312, 562089333072243404, 2100206769347857802, 1648441752718476891, 1122909156002190049, 2277864756348908247]
    SL = [1160777405615146765, 1466963809756874025, 2307294256159045561, 1499554366756826301, 1328753870933556460, 2272329651124291436]
    vec_h = [1499554366756826301, 2272329651124291436, 1219142240579132805, 783239516282340099, 518328600787762545, 368566421307526346]
    SR = [949614091546416552, 1966109414838663646, 936006576136153312, 562089333072243404, 1623880455753704159,2100206769347857802 ]

    rho = 1380917867236249887
    gamma = 1989656915129165968
    alpha = 1915953809120944686
    tau1 = 998622983965915544
    tau2 = 466019765628807296

    x = 90674128122773783995181052795706680529257422091107266216195963841335833062463 #61133062294669951655602742753363419500139758125382541498639428591392750155167 #115652887515138899162585127552089451472650926160005882762815374161457880507885
    y = 46155423958541846463797771708411388393694846047192629675802094335562426863355 #21927837593244675618276189758133024960731143978469870926657956032577117639678
    z = 8756591652865634093805800671387648119477879471685707841301941995705999603235 #37310066213429683805310643795342640654894479534136247321171790916159527922660

    al = [0, 0, 1, 1, 0, 1]
    ar = [0] * lens
    for i in range(lens):
        ar[i] = (al[i] - 1) %p
    print(ar)

    ck1 = 0
    ck2 = 0
    ck3 = 0
    _y = 1
    _tw = 1
    for i in range(lens):
        ck1 += al[i] * _tw
        ck2 += al[i] * (ar[i] * _y )
        ck3 += (al[i] - 1 - ar[i]) *_y % p

        _y *= y % p
        _tw *= 2
    print( "ck1: " + str(ck1)) 
    print( "ck2: " + str(ck2)) 
    print( "ck3: " + str(ck3)) 

    V = commit_s(g, v, h, gamma, p)
    print("V")
    print(V)
    A = (pow(h, alpha, p) * commit_v(vec_g, al, vec_h, ar, lens, p)) % p
    

    S = commit_v(vec_g, SL, vec_h, SR, lens, p)
    S = pow(h, rho, p) * S % p

    t1 = get_t1(al, ar, SL, SR, y, z, lens, p )
    _t1 = _get_t1(al, ar, SL, SR, y, z, lens, p )
    t2 = get_t2(SL, SR, y, lens, p)

    T1 = commit_s(g, t1, h, tau1, p)
    T2 = commit_s(g, t2, h, tau2, p)

    vec_a = get_lx(al, SL, x, z, lens, p)
    vec_b = get_rx(ar, SR, x, y, z, lens, p)
    _vec_b = _get_rx(ar, SR, x, y, z, lens, p)
    
    hat_t = inner_product(vec_a, 0, vec_b, 0, lens, p)
    tau_x = (tau2 * x * x + tau1 * x + z * z * gamma ) % p
    mu = (alpha + rho * x) % p


    check_tl = commit_s(g, hat_t, h, tau_x, p)

    delta = get_delta(z, y, lens, p)
    check_tr = (pow(V, z*z, p) * pow(g, delta, p) %p) * (pow(T1, x, p) * pow(T2, x * x, p) %p)
    check_tr = check_tr % p


    _check1 = pow(g, v * z*z, p) * pow(g, delta, p) * pow(g, t1 * x, p) * pow(g, t2*x*x, p) * pow(h, gamma*z*z, p) * pow(h, T1*x, p) * pow(h, T2*x*x, p)
    #_check1 = pow(g, v * z*z) * pow(g, delta) * pow(g, t1 * x) * pow(g, t2*x*x) * pow(h, gamma*z*z) * pow(h, T1*x) * pow(h, T2*x*x)
    _check1 %= p
    print("_check1: " +str(_check1) )
    
    _check2 = pow(pow(g, v, p) * pow(h, gamma, p) , z*z, p) * pow(g, delta, p) * pow(pow(g, t1, p) * pow(h, tau1, p), x, p) * pow(pow(g, t2, p) * pow(h, tau2, p), x*x, p)
    #_check2 = pow(pow(g, v) * pow(h, gamma) , z*z) * pow(g, delta) * pow(pow(g, t1) * pow(h, tau1), x) * pow(pow(g, t2) * pow(h, tau2), x*x)
    _check2 %= p
    print("_check2: " +str(_check2))

    _ch1 = pow(pow(g, v, p) * pow(h, gamma, p) , z*z, p) # (g^v*h^gamma)^{z^2}
    _ch2 = pow(g, v*z*z, p) * pow(h, gamma * z * z, p) % p # g^{v*z^2} * h^{gamma*z^2} 
    print("_ch1: " + str(_ch1))
    print("_ch2: " + str(_ch2))
    _ch3 = pow(pow(g, t1, p) * pow(h, tau1, p), x, p) #  (g^t1*h^tau1)^x
    _ch4 = pow(g, t1 * x, p) * pow(h, tau1*x, p) %p # g^{t1*x}h^{tau1*x}
    print("_ch3: " + str(_ch3))
    print("_ch4: " + str(_ch4))
    _ch5 = pow(pow(g, t2, p) * pow(h, tau2, p), x*x, p) # (g^t2*h^tau2)^x^2
    _ch6 = pow(g, t2*x*x, p) * pow(h, tau2 * x* x, p) % p # g^{t2*x^2}*h^{tau2*x^2}
    print("_ch5: " + str(_ch5))
    print("_ch6: " + str(_ch6))

    _ch01 = _ch1 * _ch3 * _ch5 * pow(g, delta, p)% p
    _ch02 = _ch2 * _ch4 * _ch6 * pow(g, delta, p)% p

    print("_ch01: " + str(_ch01))
    print("_ch02: " + str(_ch02))

    tx = ((v * z*z)%p + delta%p + (t1 * x)%p + (t2 * x * x)%p ) #% p

    _taux = (gamma * z *z + tau1 * x + tau2 * x * x ) % p

    _y = 1
    _tw = 1
    res_l = 0
    res_r = 0
    for i in range(lens):
        res_l += (al[i] - z) * (_y * (ar[i] + z) + z*z *_tw) % p
        _y = _y * y
        _tw = _tw * 2
    res_l %= p
    res_r = (z*z*v + delta) % p
    print("res_l: " + str(res_l))
    print("res_r: " + str(res_r))

    print("t(x): " + str((res_l + t1 *x + t2 * x * x) % p))

    print("V: " +str(V))
    print("A: " +str(A))
    print("S: " +str(S))
    print("t1: " +str(t1))
    print("_t1: " +str(_t1))
    print("t2: " +str(t2))
    print("T1: " +str(T1))
    print("T2: " +str(T2))
    for i in range(lens):
        print("vec_a " +str(vec_a[i]))
        print("vec_b " +str(vec_b[i]))
        print("_vec_b " +str(_vec_b[i]))
    print("hat t: " +str(hat_t))
    print("tx:    " + str(tx))
    print("tau x: "  +str(tau_x))
    print("_tau x: " +str(_taux))
    print("mu: " +str(mu))

    print()
    print("delta: " + str(delta))
    print("check tl: " + str(check_tl))
    print("check tr: " + str(check_tr))


    print("Vz2: " + str(pow(V, z*z, p)))
    print("g delta: " + str(pow(g, delta, p)))
    print("T1x: " + str(pow(T1, x, p)))
    print("T2x2: " + str(pow(T2, x*x, p)))



