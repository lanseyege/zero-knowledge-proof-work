package main

import (
	"fmt"
	"os"
	"github.com/Nik-U/pbc"
)


func get_new_pkdipe(l1 int, l2 int, k int, filename string) pkDIPE {
	fmt.Println("pair file name: " + filename)
	data , err := os.ReadFile(filename)
	if err != nil {panic(err)}
	pairing , err := pbc.NewPairingFromString(string(data))
	if err != nil {panic(err)}
	return pkDIPE{l1, l2, k, pairing}
}

func (dipe pkDIPE) Setup() (Group, PK, MSK) {
	pairing := dipe.pairing
	g1 := pairing.NewG1().Rand()
	g2 := pairing.NewG2().Rand()
	gt := pairing.NewGT()
	gt.Pair(g1, g2)
	
	l1, l2, k := dipe.l1, dipe.l2, dipe.k
	// initialize structs
	A1 := make([][]*pbc.Element, k)
	AU1 := make([][]*pbc.Element, k)
	AU2 := make([][]*pbc.Element, k)
	AW1 := make([][][]*pbc.Element, l1)
	AW2 := make([][][]*pbc.Element, l2)
	Ak := make([]*pbc.Element, k)

	for i := 0; i < k ; i ++ {
		A1[i] = make([]*pbc.Element, k + 1)
		AU1[i] = make([]*pbc.Element, 2 * k + 1)
		AU2[i] = make([]*pbc.Element, 2 * k + 1)
	}
	for i := 0; i < l1; i++ {
		AW1[i] = make([][]*pbc.Element, k)
		for j := 0; j < k; j++ {
			AW1[i][j] = make([]*pbc.Element, 2 * k + 1)
		}
	}
	for i := 0; i < l2; i++ {
		AW2[i] = make([][]*pbc.Element, k)
		for j := 0; j < k; j++ {
			AW2[i][j] = make([]*pbc.Element, 2 * k + 1)
		}
	}
	K_bond := make([]*pbc.Element, k + 1)
	W1 := make([][][]*pbc.Element, l1)
	W2 := make([][][]*pbc.Element, l2)
	B1 := make([][]*pbc.Element, 2 * k + 1)

	for i := 0; i < l1; i++ {
		W1[i] = make([][]*pbc.Element, k + 1)
		for j := 0; j < k + 1; j++ {
			W1[i][j] = make([]*pbc.Element, 2 * k + 1)
		}
	}
	for i := 0; i < l2; i++ {
		W2[i] = make([][]*pbc.Element, k + 1)
		for j := 0; j < k + 1; j++ {
			W2[i][j] = make([]*pbc.Element, 2 * k + 1)
		}
	}
	for i := 0; i < 2 * k + 1; i++ {
		B1[i] = make([]*pbc.Element, k)
	}

	// initialize middle variables
	At := make([][]*pbc.Element, k)
	U1 := make([][]*pbc.Element, k + 1)
	U2 := make([][]*pbc.Element, k + 1)
	//fmt.Println("A")
	for i := 0; i < k ; i++ {
		At[i] = make([]*pbc.Element, k + 1)
		for j := 0; j < k + 1; j++ {
			At[i][j] = pairing.NewZr().Rand()
			//fmt.Println(At[i][j])
		}
	}
	//fmt.Println("U2")
	for i := 0; i < k + 1; i++ {
		K_bond[i] = pairing.NewZr().Rand()
		U1[i] = make([]*pbc.Element, 2 * k + 1)
		U2[i] = make([]*pbc.Element, 2 * k + 1)
		for j := 0; j < 2 * k + 1; j++ {
			U1[i][j] = pairing.NewZr().Rand()	
			U2[i][j] = pairing.NewZr().Rand()	
			//fmt.Println(U2[i][j])
		}
	}
	// A1 
	for i := 0; i < k ; i ++ {
		for j := 0; j < k + 1; j ++ {
			A1[i][j] = pairing.NewG1()
		}
	} // B1
	for i := 0; i < 2 * k + 1; i++ {
		for j := 0; j < k ; j++ {
			B1[i][j] = pairing.NewZr().Rand()
		}
	} // W1
	for i := 0; i < l1; i ++ {
		for j := 0; j < k + 1; j++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				W1[i][j][j2] = pairing.NewZr().Rand()
			}
		}
	} // W2
	for i := 0; i < l2; i ++ {
		for j := 0; j < k + 1; j++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				W2[i][j][j2] = pairing.NewZr().Rand()
			}
		}
	}
	//initialize AU1 AU2 AW1 AW2 Ak
	for i := 0; i < k; i++ {
		Ak[i] = pairing.NewGT()
		for j := 0; j < 2 * k + 1; j++ {
			AU1[i][j] = pairing.NewG1()
			AU2[i][j] = pairing.NewG1()
		}
	}
	for i := 0; i < l1; i++ {
		for j := 0; j < k; j++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				AW1[i][j][j2] = pairing.NewG1()
			}
		}
	}
	for i := 0; i < l2; i++ {
		for j := 0; j < k; j++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				AW2[i][j][j2] = pairing.NewG1()
			}
		}
	}
	// matrix multiplication 
	temp := pairing.NewZr()
	temp2 := pairing.NewZr()
	matrix_matmul_exp(AU1, At, U1, k, k+1, 2*k+1, g1, temp, temp2)
	matrix_matmul_exp(AU2, At, U2, k, k+1, 2*k+1, g1, temp, temp2)
	for i := 0; i < l1; i++ {
		matrix_matmul_exp(AW1[i], At, W1[i], k, k+1, 2*k+1, g1, temp, temp2)
	}	
	for i := 0; i < l2; i++ {
		matrix_matmul_exp(AW2[i], At, W2[i], k, k+1, 2*k+1, g1, temp, temp2)
	}
	matrix_mul_vec_exp(Ak, At, K_bond, k, k + 1, gt, temp, temp2)
	matrix_power(A1, At, k, k + 1, g1)

	return Group{pairing, g1, g2, gt}, PK{A1, AU1, AU2, AW1, AW2, Ak}, MSK{K_bond, W1, W2, B1} 
}

func (dipe pkDIPE) KGen(msk MSK, x1 []*pbc.Element, x2 []*pbc.Element, g2 *pbc.Element) SKx {
	pairing := dipe.pairing
	//g2 := dipe.group.g2
	l1, l2, k := dipe.l1, dipe.l2, dipe.k

	r_bond := make([]*pbc.Element, k)
	temp := pairing.NewZr()
	//fmt.Println("r_bond")
	for i := 0; i < k; i ++ {
		r_bond[i] = pairing.NewZr().Rand()
		//fmt.Printf("%s\n", r_bond[i])
	}
	// K1 
	B1r := make([]*pbc.Element, 2 * k + 1)
	K1 := make([]*pbc.Element, 2 * k + 1)
	for i := 0; i < 2 * k + 1 ; i ++ {
		B1r[i] = pairing.NewZr()
		K1[i] = pairing.NewG2()
	}
	matrix_mul_vec(B1r, msk.B1, r_bond, 2*k+1, k, temp)
	for i := 0; i < 2 * k + 1; i ++ {
		K1[i].PowZn(g2, B1r[i])
	}
	// K0
	K0 := make([]*pbc.Element, k + 1)
	temp_w := make([][]*pbc.Element, k + 1)
	for i := 0; i < k + 1; i ++ {
		K0[i] = pairing.NewG2()
		temp_w[i] = make([]*pbc.Element, 2 * k + 1)
		for j := 0; j < 2 * k + 1; j ++ {
			temp_w[i][j] = pairing.NewZr()
		}
	}
	for i := 0; i < l1; i ++ {
		matrix_mul_scalar(temp_w, msk.W1[i], x1[i], k + 1, 2 * k + 1, temp)
	}
	for i := 0; i < l2; i ++ {
		matrix_mul_scalar(temp_w, msk.W2[i], x2[i], k + 1, 2 * k + 1, temp)
	}
	temp_v := make([]*pbc.Element, k + 1)
	for i := 0; i < k + 1; i ++ {
		temp_v[i] = pairing.NewZr()
	}
	matrix_mul_vec(temp_v, temp_w, B1r, k+1, 2*k+1, temp)
	for i := 0; i < k + 1; i ++ {
		temp_v[i].ThenAdd(msk.K_bond[i])
		K0[i].PowZn(g2, temp_v[i])
	}
	return SKx{K0, K1}
}

func (dipe pkDIPE) CheckKey(pk PK, x1 []*pbc.Element, x2 []*pbc.Element, skx SKx) bool {
	pairing := dipe.pairing
	l1, l2, k := dipe.l1, dipe.l2, dipe.k
	//1
	temp1 := make([][]*pbc.Element, k)
	temp2 := make([][]*pbc.Element, k)
	for i := 0; i < k; i ++ {
		temp1[i] = make([]*pbc.Element, 2 * k + 1)
		temp2[i] = make([]*pbc.Element, 2 * k + 1)
		for j := 0; j < 2 * k + 1; j ++ {
			temp1[i][j] = pairing.NewG1()
			temp2[i][j] = pairing.NewG1()
		}
	}
	matrix_power_scalar(temp1, pk.AW1[0], x1[0], k, 2*k+1)
	for i := 1; i < l1; i ++ {
		matrix_power_scalar(temp2, pk.AW1[i], x1[i], k, 2*k+1)
		matrix_mat_dotmul(temp1, temp2, k, 2*k+1)
	}
	for i := 0; i < l2; i++ {
		matrix_power_scalar(temp2, pk.AW2[i], x2[i], k, 2*k+1)
		matrix_mat_dotmul(temp1, temp2, k, 2*k+1)
	} 
	tempT := pairing.NewGT()
	temp_t1 := make([]*pbc.Element, k)
	for i := 0; i < k; i ++ {
		temp_t1[i] = pairing.NewGT()
	}
	matrix_pairing(temp_t1, temp1, skx.K1, k, 2*k+1, tempT)
	// 2 
	temp_t2 := make([]*pbc.Element, k)
	for i := 0; i < k; i ++ {
		temp_t2[i] = pairing.NewGT()
	}
	matrix_pairing(temp_t2, pk.A1, skx.K0, k, k+1, tempT)
	for i := 0; i < k; i ++ {
		temp_t2[i].ThenInvert()
	}
	//3
	for i := 0; i < k; i ++ {
		//fmt.Printf("before temp_t1[%d] = %s\n", i, temp_t1[i])
		temp_t1[i].ThenMul(temp_t2[i]).ThenMul(pk.Ak[i])
		//fmt.Printf("temp_t1[%d] = %s\n", i, temp_t1[i])
		if !temp_t1[i].Is1() {
			return false
		}
	}
	return true
}

func (dipe pkDIPE) Enc(pk PK, y1 []*pbc.Element, y2 []*pbc.Element, msg *pbc.Element) (Cy, []*pbc.Element) {
	pairing := dipe.pairing
	l1, l2, k := dipe.l1, dipe.l2, dipe.k
	fmt.Printf("l1 = %d, l2 = %d, k = %d\n", l1, l2, k)
	s_bond := make([]*pbc.Element, k)
	//fmt.Println("s_bond")
	for i := 0; i < k; i++ {
		s_bond[i] = pairing.NewZr().Rand()
		//fmt.Printf("%s\n", s_bond[i])
	}
	//C0
	C0 := make([]*pbc.Element, k+1)
	C1 := make([][]*pbc.Element, l1)
	C2 := make([][]*pbc.Element, l2)
	for i := 0; i < k + 1; i++ {
		C0[i] = pairing.NewG1()
	}
	for i := 0; i < l1; i ++ {
		C1[i] = make([]*pbc.Element, 2*k+1)
		for j := 0; j < 2*k+1; j ++ {
			C1[i][j] = pairing.NewG1()
		}
	}
	for i := 0; i < l2; i ++ {
		C2[i] = make([]*pbc.Element, 2*k+1)
		for j := 0; j < 2*k+1; j ++ {
			C2[i][j] = pairing.NewG1()
		}
	}
	temp := pairing.NewG1()
	matrix_power_vec(C0, pk.A1, s_bond, k + 1, k, temp)
	//C1
	temp_t1 := make([]*pbc.Element, 2*k+1)
	temp_t2 := make([]*pbc.Element, 2*k+1)
	for i := 0; i < 2*k+1; i++ {
		temp_t1[i] = pairing.NewG1()
		temp_t2[i] = pairing.NewG1()
	}
	for i := 0; i < l1; i ++ {
		matrix_power_vec(temp_t1, pk.AW1[i], s_bond, 2 * k + 1, k, temp)
		matrix_power_vec(temp_t2, pk.AU1, s_bond, 2 * k + 1, k, temp)
		for j := 0; j < 2 * k + 1; j ++ {
			temp_t2[j].ThenPowZn(y1[i])
		}
		vec_dotmul(C1[i], temp_t1, temp_t2, 2 * k + 1)
	}
	//C2
	for i := 0; i < l2; i ++ {
		matrix_power_vec(temp_t1, pk.AW2[i], s_bond, 2 * k + 1, k, temp)
		matrix_power_vec(temp_t2, pk.AU2, s_bond, 2 * k + 1, k, temp)
		for j := 0; j < 2 * k + 1; j ++ {
			temp_t2[j].ThenPowZn(y2[i])
		}
		vec_dotmul(C2[i], temp_t1, temp_t2, 2 * k + 1)
	}
	//C
	C := pairing.NewGT()
	temp2 := pairing.NewGT()
	for i := 0; i < k; i ++ {
		temp2.PowZn(pk.Ak[i], s_bond[i])
		C.ThenMul(temp2)
	}
	C.ThenMul(msg)
	return Cy{C0, C1, C2, C}, s_bond
}

func (dipe pkDIPE) Dec(cy Cy, skx SKx, x1 []*pbc.Element, x2 []*pbc.Element) *pbc.Element {
	pairing := dipe.pairing
	l1, l2, k := dipe.l1, dipe.l2, dipe.k
	// 2
	temp1 := make([]*pbc.Element, 2*k+1)
	temp2 := make([]*pbc.Element, 2*k+1)
	for i := 0; i < 2*k+1; i ++ {
		temp1[i] = pairing.NewG1()
		temp2[i] = pairing.NewG1()
	}
	vec_power_scalar(temp1, cy.C1[0], x1[0], 2*k+1)
	for i := 1; i < l1; i ++ {
		vec_power_scalar(temp2, cy.C1[i], x1[i], 2*k+1)
		vec_dotmul_self(temp1, temp2, 2*k+1)
	}
	for i := 0; i < l2; i++ {
		vec_power_scalar(temp2, cy.C2[i], x2[i], 2*k+1)
		vec_dotmul_self(temp1, temp2, 2*k+1)
	} 
	tempT := pairing.NewGT()
	temp_t1 := pairing.NewGT()
	vec_pairing(temp_t1, temp1, skx.K1, 2*k+1, tempT)
	// 3
	temp_t2 := pairing.NewGT()
	vec_pairing(temp_t2, cy.C0, skx.K0, k+1, tempT)
	temp_t2.ThenInvert()
	msg_ := pairing.NewGT()
	msg_.ThenMul(cy.C).ThenMul(temp_t1).ThenMul(temp_t2)
	return msg_
}


