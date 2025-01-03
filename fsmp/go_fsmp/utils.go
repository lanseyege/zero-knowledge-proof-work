package main

import (
	"fmt"
	"os"
	"github.com/Nik-U/pbc"
	"github.com/BurntSushi/toml"
	"reflect"
	"strings"
	"crypto/rand"
	"math/big"
)
// n \times m  mat_mul  m \times l
func matrix_matmul(res [][]*pbc.Element, A [][]*pbc.Element, B [][]*pbc.Element, n int, m int, l int, temp *pbc.Element) {
	for i := 0; i < n; i++ {
		for j := 0; j < l; j++ {
			for j2 := 0; j2 < m ; j2 ++ {
				//res[i][j] += A[i][j2] * B[j2][j]
				temp.Mul(A[i][j2], B[j2][j])
				res[i][j].ThenAdd(temp)
			}
		}
	}
}

func matrix_matmul_exp(res [][]*pbc.Element, A [][]*pbc.Element, B [][]*pbc.Element, n int, m int, l int, generator *pbc.Element, temp *pbc.Element, temp2 *pbc.Element) {
	for i := 0; i < n; i++ {
		for j := 0; j < l; j++ {
			for j2 := 0; j2 < m ; j2 ++ {
				temp.Mul(A[i][j2], B[j2][j])
				temp2.ThenAdd(temp)
			}
			res[i][j].PowZn(generator, temp2)
			temp2.Set0()
		}
	}
	temp.Set0()
}

func matrix_mul_vec_exp(res []*pbc.Element, A [][]*pbc.Element, B []*pbc.Element, n int, m int, generator *pbc.Element, temp *pbc.Element, temp2 *pbc.Element) {
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			temp.Mul(A[i][j], B[j])
			//res[i].ThenAdd(temp)
			temp2.ThenAdd(temp)
		}
		res[i].PowZn(generator, temp2)
		temp2.Set0()
	}
	temp.Set0()
}

func matrix_mul_vec(res []*pbc.Element, A [][]*pbc.Element, B []*pbc.Element, n int, m int, temp *pbc.Element) {
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			temp.Mul(A[i][j], B[j])
			res[i].ThenAdd(temp)
		}
	}
	temp.Set0()
}

func matrix_power(res [][]*pbc.Element, A [][]*pbc.Element, n int, m int, generator *pbc.Element) {
	for i := 0; i < n; i ++ {
		for j := 0; j < m; j ++ {
			res[i][j].PowZn(generator, A[i][j])
		}
	}
}

func matrix_power_scalar(res [][]*pbc.Element, A [][]*pbc.Element, a *pbc.Element, n int, m int) {
	for i := 0; i < n; i ++ {
		for j := 0; j < m; j ++ {
			res[i][j].PowZn(A[i][j], a)
		}
	}
}

func matrix_power_scalar_self(res [][]*pbc.Element, a *pbc.Element, n int, m int) {
	for i := 0; i < n; i ++ {
		for j := 0; j < m; j ++ {
			res[i][j].ThenPowZn(a)
		}
	}
}

func matrix_mul_scalar(res [][]*pbc.Element, A [][]*pbc.Element, B *pbc.Element, n int, m int, temp *pbc.Element) {
	for i := 0; i < n; i ++ {
		for j := 0; j < m; j++ {
			temp.Mul( A[i][j], B)
			res[i][j].ThenAdd(temp)
		}
	}
}

func matrix_mat_dotmul(A [][]*pbc.Element, B [][]*pbc.Element, n int, m int) {
	for i := 0; i < n; i ++ {
		for j := 0; j < m; j ++ {
			A[i][j].ThenMul(B[i][j])
		}
	}
}

func vec_dotmul(res []*pbc.Element, A []*pbc.Element, B []*pbc.Element, n int) {
	for i := 0; i < n; i ++ {
		res[i].Mul(A[i], B[i])
	}
}

func vec_power_scalar(res []*pbc.Element, A []*pbc.Element, b *pbc.Element, n int) {
	for i := 0; i < n; i ++ {
		res[i].PowZn(A[i], b)
	}
}
func vec_dotmul_self(res []*pbc.Element, A []*pbc.Element, n int) {
	for i := 0; i < n; i++ {
		res[i].ThenMul(A[i])
	}
}

func matrix_pairing(res []*pbc.Element, A [][]*pbc.Element, B []*pbc.Element, n int, m int, temp *pbc.Element) {
	for i := 0; i < n; i ++ {
		for j := 0; j < m; j ++ {
			temp.Pair(A[i][j], B[j])
			res[i].ThenMul(temp)
		}
	}
}

func vec_pairing(res *pbc.Element, A []*pbc.Element, B []*pbc.Element, n int, temp *pbc.Element) {
	for i := 0; i < n; i++ {
		temp.Pair(A[i], B[i])
		res.ThenMul(temp)
	}
}

func matrix_power_vec(res []*pbc.Element, A [][]*pbc.Element, B []*pbc.Element, n int, m int, temp *pbc.Element) {
	for i := 0; i < n; i ++ {
		res[i].Set1()
		for j := 0; j < m; j ++ {
			temp.PowZn(A[j][i], B[j])
			res[i].ThenMul(temp)
		}
	}
}


func EncodeS(IS []*pbc.Element, l1 int, S []int) {
	for i := 0; i < l1; i ++ {
		IS[i].Set1()
	}
	for i := 0; i < len(S); i ++ {
		IS[S[i] - 1].Set0()
	}
}

func EncodeW(IW []*pbc.Element, l1 int, w string, Phi []string) {
	for i := 0; i < l1; i ++ {
		IW[i].Set0()
		if w == Phi[i] {
			IW[i].Set1()
		}
	}
}

func write_config_to_toml_file(file_name string, class interface{}) {
	fmt.Println(file_name)
	//fmt.Println(class)
	file1 , err := os.Create(file_name)
	if err != nil {
		fmt.Println(err)
	}
	if err := toml.NewEncoder(file1).Encode(class); err != nil {
		fmt.Println(err)
	}
	if err := file1.Close() ; err != nil {
		fmt.Println(err)
	}
}

func write_to_string_to_file(file_name string, class interface{}) {
	n := reflect.TypeOf(class).NumField()
	result := ""
	for i := 0 ; i < n; i ++ {
		fd1 := reflect.ValueOf(class).Field(i)
		//fmt.Println(fd1)
		if  fd1.Type().String() != "*pbc.Element" {
			for j1 := 0; j1 < fd1.Len(); j1 ++ {
				fd2 := fd1.Index(j1)
				if  fd2.Type().String() != "*pbc.Element" {
					for j2 := 0; j2 < fd2.Len(); j2 ++ {
						fd3 := fd2.Index(j2)
						if  fd3.Type().String() != "*pbc.Element" {
							for j3 := 0; j3 < fd3.Len(); j3 ++ {
								if  fd3.Index(j3).Type().String() != "*pbc.Element" {
									fmt.Println("something goes wrong ... ")
								} else {
									result = result + fmt.Sprintf("%s", fd3.Index(j3)) + "\n"
								}
							}
						} else {
							result = result + fmt.Sprintf("%s", fd2.Index(j2)) + "\n"
						}
					}
				} else {
					result = result + fmt.Sprintf("%s", fd1.Index(j1)) + "\n"
				}
			}
		} else {
			result = result + fmt.Sprintf("%s", fd1) + "\n"
		}
	}
	file, err := os.Create(file_name)
	if (err!= nil) {fmt.Println(err)}
	_, err = file.WriteString(result)
	if (err != nil) {fmt.Println(err)}
	file.Close()
}


func read_from_file_to_string(file_name string) []string {
	bits , err := os.ReadFile(file_name)
	if err != nil {fmt.Println(err)}
	content := strings.Split(string(bits), "\n")
	return content
}

func string_to_struct_pk(content []string, pairing *pbc.Pairing, l1 int, l2 int, k int) PK {
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
	for i := 0; i < k ; i ++ {
		for j := 0; j < k + 1; j ++ {
			A1[i][j] = pairing.NewG1()
		}
	} 	//initialize AU1 AU2 AW1 AW2 Ak
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
	// set value
	inx := 0
	for i := 0; i < k; i ++ {
		for j := 0; j < k + 1; j ++ {
			A1[i][j].SetString(content[inx], 10)
			inx++
		}
	}
	for i := 0; i < k; i ++ {
		for j := 0; j < 2 * k + 1; j ++ {
			AU1[i][j].SetString(content[inx], 10)
			inx++
		}
	}
	for i := 0; i < k; i ++ {
		for j := 0; j < 2 * k + 1; j ++ {
			AU2[i][j].SetString(content[inx], 10)
			inx++
		}
	}
	for i := 0; i < l1; i ++ {
		for j := 0; j < k ; j ++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				AW1[i][j][j2].SetString(content[inx], 10)
				inx++
			}
		}
	}
	for i := 0; i < l2; i ++ {
		for j := 0; j < k ; j ++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				//fmt.Println(content[inx])
				AW2[i][j][j2].SetString(content[inx], 10)
				inx++
			}
		}
	}
	for i := 0; i < k; i ++ {
		Ak[i].SetString(content[inx], 10)
		inx++
	}
	return PK{A1, AU1, AU2, AW1, AW2, Ak}
}

func string_to_struct_msk(content []string, pairing *pbc.Pairing, l1 int, l2 int, k int) MSK {
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
	} //K_bond
	for i := 0; i < k + 1; i++ {
		K_bond[i] = pairing.NewZr()
	}
	// B1
	for i := 0; i < 2 * k + 1; i++ {
		for j := 0; j < k ; j++ {
			B1[i][j] = pairing.NewZr()
		}
	} // W1
	for i := 0; i < l1; i ++ {
		for j := 0; j < k + 1; j++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				W1[i][j][j2] = pairing.NewZr()
			}
		}
	} // W2
	for i := 0; i < l2; i ++ {
		for j := 0; j < k + 1; j++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				W2[i][j][j2] = pairing.NewZr()
			}
		}
	}
	// set value
	inx := 0
	for i := 0; i < k + 1; i ++ {
		K_bond[i].SetString(content[inx], 10)
		inx++
	} // W1
	for i := 0; i < l1; i ++ {
		for j := 0; j < k + 1; j++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				W1[i][j][j2].SetString(content[inx], 10)
				inx++
			}
		}
	} // W2
	for i := 0; i < l2; i ++ {
		for j := 0; j < k + 1; j++ {
			for j2 := 0; j2 < 2 * k + 1; j2 ++ {
				W2[i][j][j2].SetString(content[inx], 10)
				inx++
			}
		}
	} // B1
	for i := 0; i < 2 * k + 1; i++ {
		for j := 0; j < k ; j++ {
			B1[i][j].SetString(content[inx], 10)
			inx++
		}
	} 
	return MSK{K_bond, W1, W2, B1}
}
func string_to_struct_skx(content []string, pairing *pbc.Pairing, l1 int, l2 int, k int) SKx { 
	K0 := make([]*pbc.Element, k + 1)
	K1 := make([]*pbc.Element, 2 * k + 1)
	for i := 0; i < k + 1; i ++ {
		K0[i] = pairing.NewG2()
	}
	for i := 0; i < 2 * k + 1; i ++ {
		K1[i] = pairing.NewG2()
	}
	// set 
	inx := 0
	for i := 0; i < k + 1; i ++ {
		K0[i].SetString(content[inx], 10)
		inx++
	}
	for i := 0; i < 2 * k + 1; i ++ {
		K1[i].SetString(content[inx], 10)
		inx++
	}
	return SKx{K0, K1}
}
func string_to_struct_tau(content []string, pairing *pbc.Pairing, l2 int) Tau {
	Tau_f := make([]*pbc.Element, l2)
	Tau_p := make([]*pbc.Element, l2)
	for i := 0; i < l2; i ++ {
		Tau_f[i] = pairing.NewZr()
		Tau_p[i] = pairing.NewZr()
	}
	// set 
	inx := 0
	for i := 0; i < l2; i ++ {
		Tau_f[i].SetString(content[inx], 10)
		inx++
	}
	for i := 0; i < l2; i ++ {
		Tau_p[i].SetString(content[inx], 10)
		inx++
	}
	return Tau{Tau_f, Tau_p}
}

/*
func generator_yield(Bit_length int, p *big.Int, q *big.Int) *big.Int {
	g, e := rand.Prime(rand.Reader, Bit_length)
	if (e != nil) {fmt.Println(e)}
	one, two := big.NewInt(1), big.NewInt(2)
	temp1, temp2 := big.NewInt(1), big.NewInt(1)
	for {
		temp1.Exp(g, p, q)
		temp2.Exp(g, two, q)
		if temp1.Cmp(one) == 0 && temp2.Cmp(one) != 0 && g.Cmp(p) != 0 {
			break
		} else {
			g, e = rand.Prime(rand.Reader, Bit_length)
			if (e != nil) {fmt.Println(e)}
		}
	}
	return g
} */

func get_strings(crs CRS, com *pbc.Element, tau_p []*pbc.Element, c Cy, pk PK, m_dum *pbc.Element, Phi []string, a_com []*pbc.Element, a_bond_C1 [][][]*pbc.Element, a_bond_C2 [][][]*pbc.Element, a_bond_C []*pbc.Element)  string {
	res := ""
	//res += Random_g.String()
	res += crs.Random_g.String()
	res += crs.Random_h.String()
	for i := 0; i < len(tau_p); i ++ {
		res += tau_p[i].String()
	}/*
	for i := 0; i < len(crs.Com); i++ {
		res += crs.Com[i].String()
	}*/
	res += com.String()
	for i := 0; i < len(c.C0); i++ {
		res += c.C0[i].String()
	}
	for i := 0; i < len(c.C1); i++ {
		for j := 0; j < len(c.C1[0]); j ++ {
			res += c.C1[i][j].String()
		}
	}
	for i := 0; i < len(c.C2); i++ {
		for j := 0; j < len(c.C2[0]); j ++ {
			res += c.C2[i][j].String()
		}
	}
	res += c.C.String()
	for i := 0; i < len(pk.A1) ; i ++ {
		for j := 0; j < len(pk.A1[0]); j ++ {
			res += pk.A1[i][j].String()
		}
	}
	for i := 0; i < len(pk.AU1) ; i ++ {
		for j := 0; j < len(pk.AU1[0]); j ++ {
			res += pk.AU1[i][j].String()
		}
	}
	for i := 0; i < len(pk.AU2) ; i ++ {
		for j := 0; j < len(pk.AU2[0]); j ++ {
			res += pk.AU2[i][j].String()
		}
	}
	for i := 0; i < len(pk.AW1) ; i ++ {
		for j := 0; j < len(pk.AW1[0]); j ++ {
			for j2 := 0; j2 < len(pk.AW1[0][0]); j2 ++ {
				res += pk.AW1[i][j][j2].String()
			}
		}
	}
	for i := 0; i < len(pk.AW2) ; i ++ {
		for j := 0; j < len(pk.AW2[0]); j ++ {
			for j2 := 0; j2 < len(pk.AW2[0][0]); j2 ++ {
				res += pk.AW1[i][j][j2].String()
			}
		}
	}
	for i := 0; i < len(pk.Ak) ; i ++ {
		res += pk.Ak[i].String()
	}
	res += m_dum.String()
	for i := 0; i < len(Phi) ; i ++ {
		res += Phi[i]
	}
	for i := 0; i < len(a_com) ; i ++ {
		res += a_com[i].String()
	}
	for i := 0; i < len(a_bond_C1); i ++ {
		for j := 0; j < len(a_bond_C1[0]) ; j ++ {
			for j2 := 0; j2 < len(a_bond_C1[0][0]) ; j2 ++ {
				res += a_bond_C1[i][j][j2].String()
			}
		}
	}
	for i := 0; i < len(a_bond_C2); i ++ {
		for j := 0; j < len(a_bond_C2[0]) ; j ++ {
			for j2 := 0; j2 < len(a_bond_C2[0][0]) ; j2 ++ {
				res += a_bond_C2[i][j][j2].String()
			}
		}
	}
	for i := 0; i < len(a_bond_C); i ++ {
		res += a_bond_C[i].String()
	}
	return res
}

func string_to_struct_crs(content []string, pairing *pbc.Pairing) CRS { 
	Random_g := pairing.NewG1()
	Random_h := pairing.NewG1()
	inx := 0
	Random_g.SetString(content[inx], 10)
	inx ++
	Random_h.SetString(content[inx], 10)
	inx ++
	return CRS{Random_g, Random_h} //, r_com, com}
}
func string_to_struct_cy(content []string, pairing *pbc.Pairing, l1 int, l2 int, k int) Cy {
	C0 := make([]*pbc.Element, k+1)
	C1 := make([][]*pbc.Element, l1)
	C2 := make([][]*pbc.Element, l2)
	C := pairing.NewGT()
	for i := 0; i < k + 1; i ++ {
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
	inx := 0
	for i := 0; i < k + 1; i ++ {
		C0[i].SetString(content[inx], 10)
		inx ++
	}
	for i := 0; i < l1; i ++ {
		for j := 0; j < 2*k+1; j ++ {
			C1[i][j].SetString(content[inx], 10)
			inx ++
		}
	}
	for i := 0; i < l2; i ++ {
		for j := 0; j < 2*k+1; j ++ {
			C2[i][j].SetString(content[inx], 10)
			inx ++
		}
	}
	C.SetString(content[inx], 10)
	return Cy{C0, C1, C2, C}
}

func string_to_struct_pi(content []string, pairing *pbc.Pairing, l1 int, l2 int, k int) PI {
	Tau_p := make([]*pbc.Element, l2)
	C_bond_i := make([]*pbc.Element, l1)
	Z_com_i := make([]*pbc.Element, l1)
	Z_bond_t := make([][]*pbc.Element, l1)
	for i := 0; i < l2; i ++ {
		Tau_p[i] = pairing.NewZr()
	}
	for i := 0; i < l1; i ++ {
		C_bond_i[i] = pairing.NewZr()
		Z_com_i[i] = pairing.NewZr()
		Z_bond_t[i] = make([]*pbc.Element, k)
		for j := 0; j < k; j ++ {
			Z_bond_t[i][j] = pairing.NewZr()
		}
	}
	inx := 0
	for i := 0; i < l2; i ++ {
		Tau_p[i].SetString(content[inx], 10)
		inx ++
	}
	for i := 0; i < l1; i ++ {
		C_bond_i[i].SetString(content[inx], 10)
		inx ++
	}
	for i := 0; i < l1; i ++ {
		Z_com_i[i].SetString(content[inx], 10)
		inx ++
	}
	for i := 0; i < l1; i ++ {
		for j := 0; j < k; j ++ {
			Z_bond_t[i][j].SetString(content[inx], 10)
			inx ++
		}
	}
	return PI{Tau_p, C_bond_i, Z_com_i, Z_bond_t}
}

func generate_random(rando *pbc.Element, prime *big.Int) *pbc.Element {
	random_ , err := rand.Int(rand.Reader, prime)
	if err != nil {fmt.Println(err)}
	return rando.SetBig(random_)
}

