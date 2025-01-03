package main

import (
	"fmt"
	"os"
	//"io"
	"github.com/Nik-U/pbc"
)

type Group struct {
	pairing *pbc.Pairing
	g1 *pbc.Element
	g2 *pbc.Element
	gt *pbc.Element
}

type PK struct {
	A1 [][]*pbc.Element // k * k+1
	AU1 [][]*pbc.Element // k * 2k+1
	AU2 [][]*pbc.Element // k * 2k+1
	AW1 [][][]*pbc.Element //l1 * k * 2k+1
	AW2 [][][]*pbc.Element //l2 * k * 2k+1
	Ak []*pbc.Element //k
}
	
type MSK struct {
	k_bond []*pbc.Element // k+1
	W1 [][][]*pbc.Element //l1 * k+1 * 2k+1
	W2 [][][]*pbc.Element //l2 * k+1 * 2k+1
	B1 [][]*pbc.Element //2k+1 * k
}

type SKx struct {
	K0 []*pbc.Element
	K1 []*pbc.Element
}

type pkDIPE struct {
	l1 int
	l2 int
	k int
	//pairing *pbc.Pairing
	group Group
	pk PK
	msk MSK
}

type Cy struct {
	C0 []*pbc.Element
	C1 [][]*pbc.Element
	C2 [][]*pbc.Element
	C *pbc.Element
}

func get_new_pkdipe(l1 int, l2 int, k int, filename string) pkDIPE {
	data , err := os.ReadFile(filename)
	if err != nil {panic(err)}
	pairing , err := pbc.NewPairingFromString(string(data))
	if err != nil {panic(err)}

	g1 := pairing.NewG1()
	g2 := pairing.NewG2()
	gt := pairing.NewGT()

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

	k_bond := make([]*pbc.Element, k + 1)
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
	return pkDIPE{l1, l2, k, Group{pairing, g1, g2, gt}, PK{A1, AU1, AU2, AW1, AW2, Ak}, MSK{k_bond, W1, W2, B1}}
}

func (dipe pkDIPE) Setup() {
	pairing := dipe.group.pairing
	g1 := dipe.group.g1.Rand()
	g2 := dipe.group.g2.Rand()
	dipe.group.gt.Pair(g1, g2)
	gt := dipe.group.gt
	
	
	l1, l2, k := dipe.l1, dipe.l2, dipe.k
	// initialize middle variables
	At := make([][]*pbc.Element, k)
	U1 := make([][]*pbc.Element, k + 1)
	U2 := make([][]*pbc.Element, k + 1)
	for i := 0; i < k ; i++ {
		At[i] = make([]*pbc.Element, k + 1)
		for j := 0; j < k + 1; j++ {
			At[i][j] = pairing.NewZr().Rand()
		}
	}
	for i := 0; i < k + 1; i++ {
		dipe.msk.k_bond[i] = pairing.NewZr().Rand()
		U1[i] = make([]*pbc.Element, 2 * k + 1)
		U2[i] = make([]*pbc.Element, 2 * k + 1)
		for j := 0; j < 2 * k + 1; j++ {
			U1[i][j] = pairing.NewZr().Rand()	
			U2[i][j] = pairing.NewZr().Rand()	
		}
	}
	// A1 
	for i := 0; i < k ; i ++ {
		for j := 0; j < k + 1; j ++ {
			dipe.pk.A1[i][j] = pairing.NewG1()
		}
	} // B1
	for i := 0; i < 2 * k + 1; i++ {
		for j := 0; j < k ; j++ {
			dipe.msk.B1[i][j] = pairing.NewZr().Rand()
		}
	} // W1
	for i := 0; i < l1; i ++ {
		for j := 0; j < k + 1; j++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				dipe.msk.W1[i][j][j2] = pairing.NewZr().Rand()
			}
		}
	} // W2
	for i := 0; i < l2; i ++ {
		for j := 0; j < k + 1; j++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				dipe.msk.W2[i][j][j2] = pairing.NewZr().Rand()
			}
		}
	}
	//initialize AU1 AU2 AW1 AW2 Ak
	for i := 0; i < k; i++ {
		dipe.pk.Ak[i] = pairing.NewGT()
		for j := 0; j < 2 * k + 1; j++ {
			dipe.pk.AU1[i][j] = pairing.NewG1()
			dipe.pk.AU2[i][j] = pairing.NewG1()
		}
	}
	for i := 0; i < l1; i++ {
		for j := 0; j < k; j++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				dipe.pk.AW1[i][j][j2] = pairing.NewG1()
			}
		}
	}
	for i := 0; i < l2; i++ {
		for j := 0; j < k; j++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				dipe.pk.AW2[i][j][j2] = pairing.NewG1()
			}
		}
	}

	fmt.Println("AU1 before mat")
	for i := 0; i < k ; i ++ {
		for j := 0; j < 2 * k + 1; j ++ {
			fmt.Printf("%s ", dipe.pk.AU1[i][j])
		}
		fmt.Printf("\n")
	}
	fmt.Println("AU1 %d %d", len(dipe.pk.AU1), len(dipe.pk.AU1[0]))
	fmt.Println("A1 %d %d", len(dipe.pk.A1), len(dipe.pk.A1[0]))
	fmt.Println("U1 %d %d", len(U1), len(U1[0]))
	// matrix multiplication 
	temp := pairing.NewZr()
	temp2 := pairing.NewZr()
	matrix_matmul_exp(dipe.pk.AU1, At, U1, k, k+1, 2*k+1, dipe.group.g1, temp, temp2)
	matrix_matmul_exp(dipe.pk.AU2, At, U2, k, k+1, 2*k+1, dipe.group.g1, temp, temp2)
	for i := 0; i < l1; i++ {
		matrix_matmul_exp(dipe.pk.AW1[i], At, dipe.msk.W1[i], k, k+1, 2*k+1, dipe.group.g1, temp, temp2)
	}	
	for i := 0; i < l2; i++ {
		matrix_matmul_exp(dipe.pk.AW2[i], At, dipe.msk.W2[i], k, k+1, 2*k+1, dipe.group.g1, temp, temp2)
	}
	matrix_mul_vec_exp(dipe.pk.Ak, At, dipe.msk.k_bond, k, k + 1, dipe.group.gt, temp, temp2)
	matrix_power(dipe.pk.A1, At, k, k + 1, dipe.group.g1)
	// exponential of matrix

	fmt.Println("g1 = %s\n", g1)
	fmt.Println("g1 = %s\n", dipe.group.g1)
	fmt.Println("g2 = %s\n", g2)
	fmt.Println("g2 = %s\n", dipe.group.g2)
	fmt.Println("gt = %s\n", gt)
	fmt.Println("gt = %s\n", dipe.group.gt)
	fmt.Println("AU1 after mat")
	for i := 0; i < k ; i ++ {
		for j := 0; j < 2 * k + 1; j ++ {
			fmt.Printf("%s ", dipe.pk.AU1[i][j])
		}
		fmt.Printf("\n")
	}
}

func (dipe pkDIPE) KGen(x1 []*pbc.Element, x2 []*pbc.Element ) SKx {
	pairing := dipe.group.pairing
	//g2 := dipe.group.g2
	l1, l2, k := dipe.l1, dipe.l2, dipe.k

	r_bond := make([]*pbc.Element, k)
	temp := pairing.NewZr()
	for i := 0; i < k; i ++ {
		r_bond[i] = pairing.NewZr().Rand()
	}
	// K1 
	B1r := make([]*pbc.Element, 2 * k + 1)
	K1 := make([]*pbc.Element, 2 * k + 1)
	for i := 0; i < 2 * k + 1 ; i ++ {
		B1r[i] = pairing.NewZr()
		K1[i] = pairing.NewG2()
	}
	matrix_mul_vec(B1r, dipe.msk.B1, r_bond, 2*k+1, k, temp)
	for i := 0; i < 2 * k + 1; i ++ {
		K1[i].PowZn(dipe.group.g2, B1r[i])
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
		matrix_mul_scalar(temp_w, dipe.msk.W1[i], x1[i], k + 1, 2 * k + 1, temp)
	}
	for i := 0; i < l2; i ++ {
		matrix_mul_scalar(temp_w, dipe.msk.W2[i], x2[i], k + 1, 2 * k + 1, temp)
	}
	temp_v := make([]*pbc.Element, k + 1)
	for i := 0; i < k + 1; i ++ {
		temp_v[i] = pairing.NewZr()
	}
	matrix_mul_vec(temp_v, temp_w, B1r, k+1, 2*k+1, temp)
	for i := 0; i < k + 1; i ++ {
		temp_v[i].ThenAdd(dipe.msk.k_bond[i])
		K0[i].PowZn(dipe.group.g2, temp_v[i])
	}
	return SKx{K0, K1}
}

func (dipe pkDIPE) CheckKey(x1 []*pbc.Element, x2 []*pbc.Element, skx SKx) bool {
	pairing := dipe.group.pairing
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
	matrix_power_scalar(temp1, dipe.pk.AW1[0], x1[0], k, 2*k+1)
	for i := 1; i < l1; i ++ {
		matrix_power_scalar(temp2, dipe.pk.AW1[i], x1[i], k, 2*k+1)
		matrix_mat_dotmul(temp1, temp2, k, 2*k+1)
	}
	for i := 0; i < l2; i++ {
		matrix_power_scalar(temp2, dipe.pk.AW2[i], x2[i], k, 2*k+1)
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
	matrix_pairing(temp_t2, dipe.pk.A1, skx.K0, k, k+1, tempT)
	for i := 0; i < k; i ++ {
		temp_t2[i].ThenInvert()
	}
	//3
	for i := 0; i < k; i ++ {
		fmt.Printf("before temp_t1[%d] = %s\n", i, temp_t1[i])
		temp_t1[i].ThenMul(temp_t2[i]).ThenMul(dipe.pk.Ak[i])
		fmt.Printf("temp_t1[%d] = %s\n", i, temp_t1[i])
		if !temp_t1[i].Is1() {
			return false
		}
	}
	//check ...
	/*
	temp_t3 := make([]*pbc.Element, k)
	temp_t4 := make([]*pbc.Element, k + 1)
	for i := 0; i < k; i ++ {
		temp_t3[i] = pairing.NewGT()
	}
	for i := 0; i < k + 1; i ++ {
		temp_t4[i] = pairing.NewG2()
		temp_t4[i].PowZn(dipe.group.g2, dipe.msk.k_bond[i])
	}
	tempT2 := pairing.NewGT()
	matrix_pairing(temp_t3, dipe.pk.A1, temp_t4, k, k+1, tempT2)
	for i := 0; i < k; i ++ {
		//temp_t3[i].ThenInvert().ThenMul(dipe.pk.Ak[i])
		//tempT2.Invert(temp_t3[i])
		//tempT2.ThenMul(dipe.pk.Ak[i])
		//fmt.Printf("check t3[%d] = %s\n", i, temp_t3[i])
		//fmt.Printf("check  = %s\n", tempT2)
		fmt.Printf("t3 %s\n", temp_t3[i])
		fmt.Printf("Ak %s\n", dipe.pk.Ak[i])
	} */

	return true
}

func (dipe pkDIPE) Enc(y1 []*pbc.Element, y2 []*pbc.Element, msg *pbc.Element) Cy {
	l1, l2, k := dipe.l1, dipe.l2, dipe.k
	s_bond := make([]*pbc.Element, k)
	for i := 0; i < k; i++ {
		s_bond[i] = dipe.group.pairing.NewZr()
	}
	//C0
	C0 := make([]*pbc.Element, k+1)
	C1 := make([][]*pbc.Element, l1)
	C2 := make([][]*pbc.Element, l2)
	for i := 0; i < k + 1; i++ {
		C0[i] = dipe.group.pairing.NewG1()
	}
	for i := 0; i < l1; i ++ {
		C1[i] = make([]*pbc.Element, 2*k+1)
		for j := 0; j < 2*k+1; j ++ {
			C1[i][j] = dipe.group.pairing.NewG1()
		}
	}
	for i := 0; i < l2; i ++ {
		C2[i] = make([]*pbc.Element, 2*k+1)
		for j := 0; j < 2*k+1; j ++ {
			C2[i][j] = dipe.group.pairing.NewG1()
		}
	}
	temp := dipe.group.pairing.NewG1()
	matrix_power_vec(C0, dipe.pk.A1, s_bond, k + 1, k, temp)
	//C1
	temp_t1 := make([]*pbc.Element, 2*k+1)
	temp_t2 := make([]*pbc.Element, 2*k+1)
	for i := 0; i < 2*k+1; i++ {
		temp_t1[i] = dipe.group.pairing.NewG1()
		temp_t2[i] = dipe.group.pairing.NewG1()
	}
	for i := 0; i < l1; i ++ {
		matrix_power_vec(temp_t1, dipe.pk.AW1[i], s_bond, 2 * k + 1, k, temp)
		matrix_power_vec(temp_t2, dipe.pk.AU1, s_bond, 2 * k + 1, k, temp)
		for j := 0; j < 2 * k + 1; j ++ {
			temp_t2[j].ThenPowZn(y1[i])
		}
		vec_dotmul(C1[i], temp_t1, temp_t2, 2 * k + 1)
	}
	//C2
	for i := 0; i < l2; i ++ {
		matrix_power_vec(temp_t1, dipe.pk.AW2[i], s_bond, 2 * k + 1, k, temp)
		matrix_power_vec(temp_t2, dipe.pk.AU2, s_bond, 2 * k + 1, k, temp)
		for j := 0; j < 2 * k + 1; j ++ {
			temp_t2[j].ThenPowZn(y2[i])
		}
		vec_dotmul(C2[i], temp_t1, temp_t2, 2 * k + 1)
	}
	//C
	C := dipe.group.pairing.NewGT()
	temp2 := dipe.group.pairing.NewGT()
	for i := 0; i < k; i ++ {
		temp2.PowZn(dipe.pk.Ak[i], s_bond[i])
		C.ThenMul(temp2)
	}
	C.ThenMul(msg)
	return Cy{C0, C1, C2, C}
}

func (dipe pkDIPE) Dec(cy Cy, skx SKx, x1 []*pbc.Element, x2 []*pbc.Element) *pbc.Element {
	l1, l2, k := dipe.l1, dipe.l2, dipe.k
	// 2
	temp1 := make([]*pbc.Element, 2*k+1)
	temp2 := make([]*pbc.Element, 2*k+1)
	for i := 0; i < 2*k+1; i ++ {
		temp1[i] = dipe.group.pairing.NewG1()
		temp2[i] = dipe.group.pairing.NewG1()
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
	tempT := dipe.group.pairing.NewGT()
	temp_t1 := dipe.group.pairing.NewGT()
	vec_pairing(temp_t1, temp1, skx.K1, 2*k+1, tempT)
	// 3
	temp_t2 := dipe.group.pairing.NewGT()
	vec_pairing(temp_t2, cy.C0, skx.K0, k+1, tempT)
	temp_t2.ThenInvert()
	msg_ := dipe.group.pairing.NewGT()
	msg_.ThenMul(cy.C).ThenMul(temp_t1).ThenMul(temp_t2)
	return msg_
}


