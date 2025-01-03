
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

def get_vec_hi(vec_h, y, lens , p, p2):
    vec_hi = [vec_h[0]]
    #y = y % p
    for i in range(1, lens):
        #a = pow(_y, -1, p)
        c = pow(y, i, p)
        a = pow(c, -1, p)
        #print("a y : " + str(a * _y %p))
        b = pow(vec_h[i], a, p)
        vec_hi.append(b)
        #a = pow(vec_h[i], _y, p)
        #vec_hi.append(pow(a, -1, p))
        #_y *= y
        #_y %= p1
    return vec_hi

def get_hp():
    _y = 1
    _tw = 1
    _z = z*z % p
    for i in range(lens):
        z * y
def get_vec_temp(y,z, lens, p, p2):
    vec_temp = []
    _y = 1
    _tw = 1
    for i in range(lens):
        vec_temp.append(z *_y + z*z * _tw)
        _y *= y
        #_y %= p2
        _tw *= 2
    return vec_temp 

def proof_inner_product(vec_g, vec_h, u):
    pass


if __name__ == "__main__":

    v = 44
    lens = 8

    p = 2321554811230297367
    q = 1160777405615148683
    g = 949614091546416552
    h = 1466963809756874025
    p2 = p - 1

    vec_g = [936006576136153312, 562089333072243404, 2100206769347857802, 1648441752718476891, 1122909156002190049, 2277864756348908247 , 1751241068073253848, 1393402950955821549]
    vec_h = [1499554366756826301, 2272329651124291436, 1219142240579132805, 783239516282340099, 518328600787762545, 368566421307526346, 1761047989180823244, 1629005356672022600]
    SL = [1160777405615146765, 1466963809756874025, 2307294256159045561, 1499554366756826301, 1328753870933556460, 2272329651124291436, 1989656915129165968, 1380917867236249887]
    SR = [949614091546416552, 1966109414838663646, 936006576136153312, 562089333072243404, 1623880455753704159,2100206769347857802,1915953809120944686, 998622983965915544 ]

    rho = 1688071933110595193 # 1380917867236249887
    gamma = 466019765628807296 # 1989656915129165968
    alpha = 1222998764048622683 # 1915953809120944686
    tau1 = 1219142240579132805 # 998622983965915544
    tau2 = 1648441752718476891 # 466019765628807296

    x = 33821306132510009352605092301607665923101641412271807644886436820354529081497 #67062475371173391264930613312350525112966446387792512000338263458420090573573 
    #x = 58723778984961479245050443261619876551124665431558016956256697599100143193432
    x = 1065103013158696732442429719408098316438765538043412463891963485416272715705
    y = 63235430145696460546455635519942328829794089584265457776446194573955588997556 # 46155423958541846463797771708411388393694846047192629675802094335562426863355 #21927837593244675618276189758133024960731143978469870926657956032577117639678
    #y = 34147350200811120113668821465061181043527792771202151458040452835475928125549
    y = 19884099381171398033288288191894351531974934333056470943438436136208663438164
    z = 61392441426059077827993276073495633482710160071258476694766835679241029284676 #8756591652865634093805800671387648119477879471685707841301941995705999603235 #37310066213429683805310643795342640654894479534136247321171790916159527922660
    #z = 111592262537559072542996700415924083361256093006263964590781731977339897884753
    z = 42451126273694673097325037023509067891807128219743104310269277428135151674284

    x %= p2
    y %= p2
    z %= p2
    al = [ 0, 0, 1, 1, 0, 1, 0, 0]
    ar = [0] * lens
    for i in range(lens):
        ar[i] = (al[i] - 1) %p2
    print(ar)

    ck1 = 0
    ck2 = 0
    ck3 = 0
    _y = 1
    _tw = 1
    for i in range(lens):
        ck1 += al[i] * _tw
        ck2 += al[i] * (ar[i] * _y )
        ck3 += (al[i] - 1 - ar[i]) *_y % p2

        _y *= y % p2
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

    t1 = get_t1(al, ar, SL, SR, y, z, lens, p2 )
    _t1 = _get_t1(al, ar, SL, SR, y, z, lens, p2 )
    t2 = get_t2(SL, SR, y, lens, p2)

    T1 = commit_s(g, t1, h, tau1, p)
    T2 = commit_s(g, t2, h, tau2, p)

    vec_a = get_lx(al, SL, x, z, lens, p2)
    vec_b = get_rx(ar, SR, x, y, z, lens, p2)
    _vec_b = _get_rx(ar, SR, x, y, z, lens, p2)
    
    hat_t = inner_product(vec_a, 0, vec_b, 0, lens, p2)
    tau_x = (tau2 * x * x + tau1 * x + z * z * gamma ) % p2
    mu = (alpha + rho * x) % p2


    check_tl = commit_s(g, hat_t, h, tau_x, p)

    delta = get_delta(z, y, lens, p2)
    check_tr = (pow(V, z*z%p2, p) * pow(g, delta, p) %p) * (pow(T1, x%p2, p) * pow(T2, x * x%p2, p) %p)
    check_tr = check_tr % p


    _check1 = pow(g, v * z*z, p) * pow(g, delta, p) * pow(g, t1 * x, p) * pow(g, t2*x*x, p) * pow(h, gamma*z*z, p) * pow(h, tau1*x, p) * pow(h, tau2*x*x, p)
    #_check1 = pow(g, v * z*z) * pow(g, delta) * pow(g, t1 * x) * pow(g, t2*x*x) * pow(h, gamma*z*z) * pow(h, T1*x) * pow(h, T2*x*x)
    _check1 %= p
    print("_check1: " +str(_check1) )
    
    _check2 = pow(pow(g, v, p) * pow(h, gamma, p)%p , z*z, p) * pow(g, delta, p) * pow(pow(g, t1, p) * pow(h, tau1, p)%p, x, p) * pow(pow(g, t2, p) * pow(h, tau2, p)%p, x*x, p)
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

    tx = ((v * z*z)%p2 + delta%p2 + (t1 * x)%p2 + (t2 * x * x)%p2 ) % p2

    _taux = (gamma * z *z + tau1 * x + tau2 * x * x ) % p2

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

    
    vec_hi = get_vec_hi(vec_h, y, lens, p, p2)
    print("vec_hi")
    print(vec_hi)
    vec_temp = get_vec_temp(y,z, lens, p, p2)
    print(vec_temp)
    gz = 1
    for i in range(lens):
        a = pow(vec_g[i] , z, p)
        b = pow(a, -1, p)
        c = -z % p

        c2 = -z % p2
        print("vec g z inverse: " + str(b))
        print("c : " +str(c) )
        print(pow(vec_g[i], c, g))
        print("c2 : " +str(c2))
        print(pow(vec_g[i], c2, g))
        gz *= b
        #gz *= pow(vec_g[i], c2, g)
    gz %= p
    hp = 1
    for i in range(lens):
        a = pow(vec_hi[i], vec_temp[i], p)
        hp *= a
    hp %= p
    PL =  A * pow(S, x, p) * gz * hp
    PL %= p

    print("PL")
    print(PL)

    PR = commit_v(vec_g, vec_a, vec_hi, vec_b, lens, p) * pow(h, mu, p) % p

    print("PR")
    print(PR)

    for i in range(lens):
        print("g hi, a b : " + str(i))
        print(vec_g[i])
        print(vec_hi[i])
        print(vec_a[i])
        print(vec_b[i])

    print(pow(g, q, p))
    print(pow(g, 2, p))
    print(pow(g, p-1, p))
    print()
    print(pow(h, q, p))
    print(pow(h, p-1, p))
    print(pow(g*g, q, p))


    l1 = pow(h, (alpha + rho * x), p) # h^mu
    print("l1: " + str(l1))
    print(pow(h, mu, p))
    l2 = commit_v(vec_g, al, vec_h, ar, lens, p) # g^al * h ^ar
    print("l2: " + str(l2))
    l3 = pow(commit_v(vec_g, SL, vec_h, SR, lens, p), x, p) # (g^sl * h ^Sr)^x
    print("l3: " + str(l3));
    l4 = l1 * l2 * l3 * hp * gz % p
    #l4 = l1 * l2 * l3 *gz % p
    print("l4: " + str(l4))



    ltemp = []
    _y = 1
    _tw = 1
    for i in range(lens):
        ltemp.append(z * _y + z * z * _tw )
        _y *= y
        _tw *= 2
    lres = 1
    for i in range(lens):
        lres *= pow(vec_hi[i], ltemp[i], p)
    print("lres: " + str(lres%p))

    rres = 1
    r1 = pow(h, alpha+rho*x , p)
    r2 = 1
    r3 = 1
    r4 = 1
    _y = 1
    _tw = 1
    for i in range(lens):
        #r2 *= pow(vec_g[i], (al[i] + SL[i] * x - z)%p2 , p)
        r2 *= pow(vec_g[i], (al[i] + SL[i] * x -z)%p2 , p)
        #r3 *= pow(vec_hi[i], (_y*(ar[i] + SR[i] * x + z) + z*z*_tw) , p)
        r3 *= pow(vec_h[i], ar[i]+SR[i]*x, p)
        r4 *= pow(vec_hi[i], z*_y+z*z*_tw, p)
        _y *= y
        _tw *= 2
    rres = r1 * r2 * r3 * r4% p
    print(rres)

    _hl = 1
    for i in range(lens):
        _hl *= pow(vec_h[i], ar[i] + SR[i] * x, p)
    print("_hl: " + str(_hl%p))

    _hr = 1
    _y = 1
    _tw = 1
    print(vec_hi)
    print(vec_h)
    for i in range(lens):
        _hr *= pow(vec_hi[i], _y *(ar[i] + SR[i] * x), p)
        print(pow(vec_hi[i], _y, p))
        _y *= y
        _tw *= 2
        
    print("_hr: " + str(_hr%p))



