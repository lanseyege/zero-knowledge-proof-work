package main

import (
	"fmt"
	"os"
	"math/rand"
	"crypto/sha256"
	"github.com/Nik-U/pbc"
	"github.com/BurntSushi/toml"
)

type fSMP_Verifier struct {
	conf Config
	crs CRS
	sme SME
	cy Cy
	pi PI
	tau Tau
	m_dum *pbc.Element
	pk PK
	skx SKx
	com *pbc.Element
}

func get_new_fsmp_verifier(config_name string) fSMP_Verifier {
	var conf Config
	if _ , err := toml.DecodeFile(config_name, &conf); err != nil {
		panic(err)
	}
	conf.L1 = len(conf.Phi) 	
	if conf.Seed > 0 {
		pbc.SetRandRandom(rand.New(rand.NewSource(conf.Seed + 3)))
	}
	sme := get_new_sme(conf.L1, conf.L2, conf.K, conf.Pair_file)
	content := read_from_file_to_string(conf.Keys_path + "crs_new")
	crs := string_to_struct_crs(content, sme.dipe.pairing)
	content = read_from_file_to_string(conf.Keys_path + "pk")
	pk := string_to_struct_pk(content, sme.dipe.pairing, conf.L1, conf.L2, conf.K)
	content = read_from_file_to_string(conf.Data_path + "Cy")
	cy := string_to_struct_cy(content, sme.dipe.pairing, conf.L1, conf.L2, conf.K)
	content = read_from_file_to_string(conf.Data_path + "PI")
	pi := string_to_struct_pi(content, sme.dipe.pairing, conf.L1, conf.L2, conf.K)
	content = read_from_file_to_string(conf.Keys_path + "tau")
	tau := string_to_struct_tau(content, sme.dipe.pairing, conf.L2)
	content = read_from_file_to_string(conf.Keys_path + "skx")
	skx := string_to_struct_skx(content, sme.dipe.pairing, conf.L1, conf.L2, conf.K)
	content = read_from_file_to_string(conf.Keys_path + "m_dum")
	m_dum := sme.dipe.pairing.NewGT()
	m_dum.SetString(content[0], 10)

	content = read_from_file_to_string(conf.Keys_path + "pedersen")
	com := sme.dipe.pairing.NewG1()
	com.SetString(content[0], 10)

	return fSMP_Verifier{conf, crs, sme, cy, pi, tau, m_dum, pk, skx, com}
}

func (fsmp_verifier fSMP_Verifier) NIZK_Verify(crs CRS, c Cy, Phi []string, m_dum *pbc.Element, pk PK) {
	l1, l2, k := fsmp_verifier.conf.L1, fsmp_verifier.conf.L2, fsmp_verifier.conf.K
	pairing := fsmp_verifier.sme.dipe.pairing
	c_bond_i := fsmp_verifier.pi.C_bond_i
	z_com_i  := fsmp_verifier.pi.Z_com_i
	z_bond_t := fsmp_verifier.pi.Z_bond_t
	tau_p := fsmp_verifier.pi.Tau_p
	a_com := make([]*pbc.Element, l1)
	a_bond_C0 := make([][]*pbc.Element, l1)
	a_bond_C1 := make([][][]*pbc.Element, l1)
	a_bond_C2 := make([][][]*pbc.Element, l1)
	a_bond_C := make([]*pbc.Element, l1)

	for i := 0; i < l1; i ++ {
		a_bond_C[i] = pairing.NewGT()
		a_com[i] = pairing.NewG1()
		//step 2, a_com
		g_phi := pairing.NewG1()
		ss := fmt.Sprintf("%x", sha256.Sum256([]byte(Phi[i])))
		ss_ , _ := pairing.NewZr().SetString(ss , 16)
		g_phi.PowZn(crs.Random_g, ss_)
		g_phi.Invert(g_phi)
		//g_phi.Mul(crs.Com[i], g_phi)
		g_phi.Mul(fsmp_verifier.com, g_phi)
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
		// step 3, a_bond_C1
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
	res := get_strings(crs, fsmp_verifier.com, tau_p, c, pk, m_dum, Phi, a_com, a_bond_C1, a_bond_C2, a_bond_C)
	ss := fmt.Sprintf("%x", sha256.Sum256([]byte(res)))
	c_hash, _ := pairing.NewZr().SetString(ss, 16)
	cxor_ := pairing.NewZr()
	for i := 0; i < l1; i ++ {
		cxor_.Add(cxor_, c_bond_i[i])
	}
	
	fmt.Printf("c_hash : %s\n", c_hash)
	fmt.Printf("cxor : %s\n", cxor_)
	if c_hash.Equals(cxor_) {
		fmt.Println("Verify Sucessfully Pass!!!")
	} else {
		fmt.Println("Verify Failed!!!")
	}
}

func (fsmp_verifier fSMP_Verifier) Verify(crs CRS, cy Cy, Phi []string, m_dum *pbc.Element, pk PK) {
	fsmp_verifier.NIZK_Verify(crs , cy , Phi , m_dum , pk )
}

func (fsmp_verifier fSMP_Verifier) Extract(cy Cy, skx SKx, S []int, tau_f []*pbc.Element, msg *pbc.Element) bool {
	return fsmp_verifier.sme.Query(cy, skx, S, tau_f, msg)
}
/*
func get_subsets(subset_path string) SubSets {
	var subsets SubSets
	if _ , err := toml.DecodeFile(subset_path, &subsets); err != nil {
		panic(err)
	}
	return subsets
}*/

func fsmp_verifier_procedure(conf_name string) {
	fmt.Println("get new fsmp verifier")
	fsmp_verifier := get_new_fsmp_verifier(conf_name)
	fsmp_verifier.Verify(fsmp_verifier.crs, fsmp_verifier.cy, fsmp_verifier.conf.Phi, fsmp_verifier.m_dum, fsmp_verifier.pk)
	//subsets := get_subsets(subset_path)
	//lens := len(subsets.Subsets)
	//for i := 0; i < lens; i ++ {
		flag := fsmp_verifier.Extract(fsmp_verifier.cy, fsmp_verifier.skx, fsmp_verifier.conf.S, fsmp_verifier.tau.Tau_f, fsmp_verifier.m_dum)
		fmt.Printf("subsets = ")
		fmt.Println(fsmp_verifier.conf.S)
		fmt.Printf("The output of extract is: %t\n" , flag)
	//}
	// Extract ... 
}

func main() {
	fmt.Println("fSMP_Verifier.go ...")
	args := os.Args
	if len(args) != 2 {
		fmt.Println("number of args is not 2, panic ... \n it should be like this: ./fsmp_verifier config_path \n")
		os.Exit(3)
	}
	fmt.Println(args[1])
	//fmt.Println(args[2])
	fsmp_verifier_procedure(args[1])

}
