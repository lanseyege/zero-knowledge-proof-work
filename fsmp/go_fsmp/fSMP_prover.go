package main

import (
	"fmt"
	"os"
	//"crypto/rand"
	"math/rand"
	"crypto/sha256"
	"github.com/Nik-U/pbc"
	"github.com/BurntSushi/toml"
)

type fSMP_Prover struct {
	conf Config
	tau Tau
	sme SME
	pk PK
	msk MSK
	m_dum *pbc.Element
	crs CRS
	witness Witness
	com *pbc.Element
	r_com *pbc.Element
}

func get_new_fsmp_prover(config_name string, witness_name string) fSMP_Prover {
	var conf Config
	if _ , err := toml.DecodeFile(config_name, &conf); err != nil {
		panic(err)
	}
	conf.L1 = len(conf.Phi)
	if conf.Seed > 0 {
		pbc.SetRandRandom(rand.New(rand.NewSource(conf.Seed + 1)))
	}
	var witness Witness 
	if _, err := toml.DecodeFile(witness_name, &witness); err != nil {
		panic(err)
	} 
	sme := get_new_sme(conf.L1, conf.L2, conf.K, conf.Pair_file)
	content := read_from_file_to_string(conf.Keys_path + "crs_new")
	crs := string_to_struct_crs(content, sme.dipe.pairing)
	content = read_from_file_to_string(conf.Keys_path + "pk")
	pk := string_to_struct_pk(content, sme.dipe.pairing, conf.L1, conf.L2, conf.K)
	content = read_from_file_to_string(conf.Keys_path + "sk")
	sk := string_to_struct_msk(content, sme.dipe.pairing, conf.L1, conf.L2, conf.K)
	content = read_from_file_to_string(conf.Keys_path + "tau")
	tau := string_to_struct_tau(content, sme.dipe.pairing, conf.L2)

	content = read_from_file_to_string(conf.Keys_path + "m_dum")
	m_dum := sme.dipe.pairing.NewGT()
	m_dum.SetString(content[0], 10)
	
	content = read_from_file_to_string(conf.Keys_path + "pedersen")
	com := sme.dipe.pairing.NewG1()
	com.SetString(content[0], 10)

	content = read_from_file_to_string(conf.Keys_path + "r_com")
	r_com := sme.dipe.pairing.NewZr()
	r_com.SetString(content[0], 10)

	return fSMP_Prover{conf, tau, sme, pk, sk, m_dum, crs, witness, com, r_com}
}

func (fsmp_prover fSMP_Prover) NIZK_Prove(crs CRS, tau_p []*pbc.Element, c Cy, pk PK, Phi []string, w string, m_dum *pbc.Element, s_bond []*pbc.Element) PI {
	l1, l2, k := fsmp_prover.conf.L1, fsmp_prover.conf.L2, fsmp_prover.conf.K
	pairing := fsmp_prover.sme.dipe.pairing
	a_com := make([]*pbc.Element, l1)
	a_bond_C0 := make([][]*pbc.Element, l1)
	a_bond_C1 := make([][][]*pbc.Element, l1)
	a_bond_C2 := make([][][]*pbc.Element, l1)
	a_bond_C := make([]*pbc.Element, l1)
	z_bond_t := make([][]*pbc.Element, l1)
	z_com_i := make([]*pbc.Element, l1)
	c_bond_i := make([]*pbc.Element, l1)
	inx := -1
	t_com_mu := pairing.NewZr().Rand()
	t_bond := make([]*pbc.Element, k)
	for i := 0; i < l1; i++ {
		a_bond_C[i] = pairing.NewGT()
		a_com[i] = pairing.NewG1()
		c_bond_i[i] = pairing.NewZr().Rand()
		z_com_i[i] = pairing.NewZr().Rand()
		z_bond_t[i] = make([]*pbc.Element, k)
		for j := 0; j < k; j++ {
			z_bond_t[i][j] = pairing.NewZr().Rand()
		}
		if w == Phi[i] {
			inx = i
			a_com[i].PowZn(crs.Random_h, t_com_mu)
			// step 2 , a_bond_C0
			for j := 0; j < k; j ++ {
				t_bond[j] = pairing.NewZr().Rand()
			}
			a_bond_C0[i] = make([]*pbc.Element, k+1)
			for j := 0; j < k+1; j++ {
				a_bond_C0[i][j] = pairing.NewG1()
			}
			temp := pairing.NewG1()
			matrix_power_vec(a_bond_C0[i], pk.A1, t_bond, k + 1, k, temp)
			// step 2 , a_bond_C1 , i
			a_bond_C1[i] = make([][]*pbc.Element, l1)
			for j := 0; j < l1; j ++ {
				a_bond_C1[i][j] = make([]*pbc.Element, 2*k+1)
				for j2 := 0; j2 < 2*k+1; j2 ++ {
					a_bond_C1[i][j][j2] = pairing.NewG1()
				}
			}
			temp_t1 := make([]*pbc.Element, 2*k+1)
			temp_t2 := make([]*pbc.Element, 2*k+1)
			for j := 0; j < 2*k+1; j++ {
				temp_t1[j] = pairing.NewG1()
				temp_t2[j] = pairing.NewG1()
			}
				matrix_power_vec(temp_t1, pk.AW1[i], t_bond, 2 * k + 1, k, temp)
				matrix_power_vec(temp_t2, pk.AU1, t_bond, 2 * k + 1, k, temp)
				vec_dotmul(a_bond_C1[i][i], temp_t1, temp_t2, 2 * k + 1)
			//step 3 , a_bond_C1, != i
			for j := 0; j < l1 ; j ++ {
				if j != i {
					matrix_power_vec(a_bond_C1[i][j], pk.AW1[j], t_bond, 2 * k + 1, k, temp)
				}
			}
			//step 4, a_bond_C2
			a_bond_C2[i] = make([][]*pbc.Element, l2)
			for j := 0; j < l2; j ++ {
				a_bond_C2[i][j] = make([]*pbc.Element, 2*k+1)
				for j2 := 0; j2 < 2*k+1; j2 ++ {
					a_bond_C2[i][j][j2] = pairing.NewG1()
				}
			}
			for j := 0; j < l2; j ++ {
				matrix_power_vec(temp_t1, pk.AW2[j], t_bond, 2 * k + 1, k, temp)
				matrix_power_vec(temp_t2, pk.AU2, t_bond, 2 * k + 1, k, temp)
				for j2 := 0; j2 < 2 * k + 1; j2 ++ {
					temp_t2[j2].ThenPowZn(tau_p[j])
				}
				vec_dotmul(a_bond_C2[i][j], temp_t1, temp_t2, 2 * k + 1)
			}
			//step 5, a_bond_C
			temp2 := pairing.NewGT()
			for j := 0; j < k; j ++ {
				temp2.PowZn(pk.Ak[j], t_bond[j])
				a_bond_C[i].ThenMul(temp2)
			}
		} else {
			//step 1, 
			g_phi := pairing.NewG1()
			ss := fmt.Sprintf("%x", sha256.Sum256([]byte(Phi[i])))
			ss_ , _ := pairing.NewZr().SetString(ss, 16)
			g_phi.PowZn(crs.Random_g, ss_)
			g_phi.Invert(g_phi)
			//g_phi.Mul(crs.Com[i], g_phi)
			g_phi.Mul(fsmp_prover.com, g_phi)
			g_phi.PowZn(g_phi, c_bond_i[i])
			tp1 := pairing.NewG1()
			tp1.PowZn(crs.Random_h, z_com_i[i])
			a_com[i].Mul(tp1, g_phi)
			//step 2, a_bond_C0
			a_bond_C0[i] = make([]*pbc.Element, k+1)
			for j := 0; j < k+1; j++ {
				a_bond_C0[i][j] = pairing.NewG1()
			}
			temp := pairing.NewG1()
			temp_t1 := make([]*pbc.Element, k+1)
			temp_t2 := make([]*pbc.Element, k+1)
			for j := 0; j < k+1; j++ {
				temp_t1[j] = pairing.NewG1()
				temp_t2[j] = pairing.NewG1()
			}
			matrix_power_vec(temp_t1, pk.A1, z_bond_t[i], k + 1, k, temp)
			vec_power_scalar(temp_t2, c.C0, c_bond_i[i], k+1)
			vec_dotmul(a_bond_C0[i], temp_t1, temp_t2, k+1)
			//step 3, a_bond_C1
			temp_t3 := make([]*pbc.Element, 2*k+1)
			temp_t4 := make([]*pbc.Element, 2*k+1)
			temp_t5 := make([]*pbc.Element, 2*k+1)
			for j := 0; j < 2*k+1; j++ {
				temp_t3[j] = pairing.NewG1()
				temp_t4[j] = pairing.NewG1()
				temp_t5[j] = pairing.NewG1()
			}
			a_bond_C1[i] = make([][]*pbc.Element, l1)
			for j := 0; j < l1; j ++ {
				a_bond_C1[i][j] = make([]*pbc.Element, 2*k+1)
				for j2 := 0; j2 < 2*k+1; j2 ++ {
					a_bond_C1[i][j][j2] = pairing.NewG1()
				}
			}
			// a_bond_C1 , i = j
			matrix_power_vec(temp_t3, pk.AW1[i], z_bond_t[i], 2*k + 1, k, temp)
			matrix_power_vec(temp_t5, pk.AU1, z_bond_t[i], 2*k + 1, k, temp)
			vec_dotmul(a_bond_C1[i][i], temp_t3, temp_t5, 2*k+1)
			vec_power_scalar(temp_t4, c.C1[i], c_bond_i[i], 2*k+1)
			vec_dotmul(a_bond_C1[i][i], a_bond_C1[i][i], temp_t4, 2*k+1)
			// a_bond_C1, i != j
			for j := 0; j < l1; j ++ {
				if j != i {
					matrix_power_vec(temp_t3, pk.AW1[j], z_bond_t[i], 2*k + 1, k, temp)
					vec_power_scalar(temp_t4, c.C1[j], c_bond_i[i], 2*k+1)
					vec_dotmul(a_bond_C1[i][j], temp_t3, temp_t4, 2*k+1)
				}
			}
			//step 4, a_bond_C2
			a_bond_C2[i] = make([][]*pbc.Element, l2)
			for j := 0; j < l2; j ++ {
				a_bond_C2[i][j] = make([]*pbc.Element, 2*k+1)
				for j2 := 0; j2 < 2*k+1; j2 ++ {
					a_bond_C2[i][j][j2] = pairing.NewG1()
				}
			}
			for j := 0; j < l2; j ++ {
				matrix_power_vec(temp_t3, pk.AW2[j], z_bond_t[i], 2 * k + 1, k, temp)
				matrix_power_vec(temp_t4, pk.AU2, z_bond_t[i], 2 * k + 1, k, temp)
				for j2 := 0; j2 < 2 * k + 1; j2 ++ {
					temp_t4[j2].ThenPowZn(tau_p[j])
				}
				vec_dotmul(a_bond_C2[i][j], temp_t3, temp_t4, 2 * k + 1)
				vec_power_scalar(temp_t3, c.C2[j], c_bond_i[i], 2*k+1)
				vec_dotmul(a_bond_C2[i][j], a_bond_C2[i][j], temp_t3, 2 * k + 1)
			}
			//step 5, a_bond_C
			temp2 := pairing.NewGT()
			temp3 := pairing.NewGT()
			for j := 0; j < k; j ++ {
				temp2.PowZn(pk.Ak[j], z_bond_t[i][j])
				a_bond_C[i].ThenMul(temp2)
			}
			temp3.Invert(m_dum)
			temp3.Mul(c.C, temp3)
			temp3.PowZn(temp3, c_bond_i[i])
			a_bond_C[i].Mul(a_bond_C[i] , temp3)
		}
	}
	// c_hash
	res := get_strings(crs, fsmp_prover.com, tau_p, c, pk, m_dum, Phi, a_com, a_bond_C1, a_bond_C2, a_bond_C)
	ss := fmt.Sprintf("%x", sha256.Sum256([]byte(res)))
	c_hash, _ := pairing.NewZr().SetString(ss, 16)
	fmt.Printf("c_hash : %s\n", c_hash)
	// c_inx
	cxor := pairing.NewZr()
	for i := 0; i < l1; i ++ {
		if i != inx {
			cxor.Add(cxor, c_bond_i[i])
		}
	}
	c_bond_i[inx].Sub(c_hash, cxor)

	tm1 := pairing.NewZr().Mul(c_bond_i[inx], fsmp_prover.r_com)
	z_com_i[inx].Sub(t_com_mu , tm1)

	temp_z := pairing.NewZr()
	for i := 0; i < k; i ++ {
		temp_z.Mul(c_bond_i[inx], s_bond[i])
		z_bond_t[inx][i].Sub(t_bond[i], temp_z)
	}
	return PI{tau_p, c_bond_i, z_com_i, z_bond_t}
}

func (fsmp_prover fSMP_Prover) Prove(crs CRS, pk PK, tau_p []*pbc.Element, Phi []string, w string, m_dum *pbc.Element) {
	fmt.Println("fsmp_prover sme enc")
	c , s_bond := fsmp_prover.sme.Enc(pk, Phi, tau_p, w, m_dum)
	fmt.Println("nizk prove")
	//pi := fsmp_prover.NIZK_Prove(crs, tau_p, com, c, pk, Phi, w, r_com, m_dum, s_bond)
	pi := fsmp_prover.NIZK_Prove(crs, tau_p, c, pk, Phi, w, m_dum, s_bond)
	
	write_to_string_to_file(fsmp_prover.conf.Data_path + "PI" , pi)
	write_to_string_to_file(fsmp_prover.conf.Data_path + "Cy" , c)
}

func fsmp_prover_procedure(conf_name string, witness_name string) {
	fmt.Println("get new fmsp prover")
	fsmp_prover := get_new_fsmp_prover(conf_name, witness_name)
	fmt.Println("prove ... ")
	fsmp_prover.Prove(fsmp_prover.crs, fsmp_prover.pk, fsmp_prover.tau.Tau_p, fsmp_prover.conf.Phi, fsmp_prover.witness.W, fsmp_prover.m_dum)
}

func main() {
	fmt.Println("fSMP_prover.go ...")
	args := os.Args
	if len(args) != 3 {
		fmt.Println("number of args is not 3, panic ... \n it should be like this: ./fsmp_prover config_path witness_path \n")
		os.Exit(3)
	}
	fmt.Println(args[1], args[2])
	fsmp_prover_procedure(args[1], args[2])

}

