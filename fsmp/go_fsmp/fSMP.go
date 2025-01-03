package main

import (
	"fmt"
	"os"
	"math/rand"
	//"crypto/sha256"
	"github.com/Nik-U/pbc"
	"github.com/BurntSushi/toml"
)


type fSMP struct {
	conf Config
	witness Witness
	tau Tau
	sme SME
}

func get_new_fsmp(config_name string, witness_name string) fSMP {
	var conf Config
	if _ , err := toml.DecodeFile(config_name, &conf); err != nil {
		panic(err)
	}
	if conf.Seed > 0 {
		pbc.SetRandRandom(rand.New(rand.NewSource(conf.Seed + 2)))
	}
	var witness Witness 
	if _, err := toml.DecodeFile(witness_name, &witness); err != nil {
		panic(err)
	}
	conf.L1 = len(conf.Phi)
	sme := get_new_sme(conf.L1, conf.L2, conf.K, conf.Pair_file)
	Tau_f := make([]*pbc.Element, conf.L2)
	Tau_p := make([]*pbc.Element, conf.L2)
	for i := 0; i < conf.L2; i ++ {
		Tau_f[i] = sme.dipe.pairing.NewZr()
		Tau_p[i] = sme.dipe.pairing.NewZr()
	}
	get_tau(Tau_f, Tau_p, conf.L2)
	return fSMP{conf, witness, Tau{Tau_f, Tau_p}, sme}
}

func (fsmp fSMP) Setup() (CRS) {
	//l1:= fsmp.conf.L1
	pairing := fsmp.sme.dipe.pairing
	Random_g := pairing.NewG1().Rand()
	Random_h := pairing.NewG1().Rand()

	return CRS{Random_g, Random_h} 
}

func (fsmp fSMP) UKGen(crs CRS) (Group, PK, MSK) {
	return fsmp.sme.Setup()
}

func (fsmp fSMP) FKGen(crs CRS, pk PK, sk MSK, S []int, tau_f []*pbc.Element, g2 *pbc.Element) SKx {
	return fsmp.sme.KGen(pk, sk, S, tau_f, g2)
}

func (fsmp fSMP) CheckKey(crs CRS, pk PK, S []int, tau_f []*pbc.Element, skx SKx) bool {
	return fsmp.sme.CheckKey(pk, S, tau_f, skx)
}

func get_tau(tau_f []*pbc.Element, tau_p []*pbc.Element, l2 int) {
	for i := 0; i < l2; i ++ {
		if i % 2 == 0 {
			tau_f[i].Set0()
			tau_p[i].Rand()
		} else {
			tau_p[i].Set0()
			tau_f[i].Rand()
		}
	}
}

func fsmp_keys_procedure(conf_name string, witness_name string) {
	fsmp := get_new_fsmp(conf_name, witness_name)
	crs := fsmp.Setup()
	group, pk, sk := fsmp.UKGen(crs)
	skx := fsmp.FKGen(crs, pk, sk, fsmp.conf.S, fsmp.tau.Tau_f, group.g2)
	flag := fsmp.CheckKey(crs, pk, fsmp.conf.S, fsmp.tau.Tau_f, skx)
	fmt.Printf("checkKey result: %t\n", flag)
	if !flag {
		fmt.Println("key check failed ... ")
		os.Exit(3)
	}
	write_to_string_to_file(fsmp.conf.Keys_path + "crs_new" , crs)
	write_to_string_to_file(fsmp.conf.Keys_path + "pk" , pk)
	write_to_string_to_file(fsmp.conf.Keys_path + "sk" , sk)
	write_to_string_to_file(fsmp.conf.Keys_path + "skx" , skx)
	write_to_string_to_file(fsmp.conf.Keys_path + "tau" , fsmp.tau)
	file , err := os.Create(fsmp.conf.Keys_path + "m_dum")
	if (err != nil) {fmt.Println(err)}
	enc, err := file.WriteString(fsmp.sme.dipe.pairing.NewGT().Rand().String())
	if (err != nil) {fmt.Println(err)}
	fmt.Println(enc)
	file.Close()
}

func main() {
	fmt.Println("fSMP.go ...")
	args := os.Args
	if len(args) != 3 {
		fmt.Println("number of args is not 3, panic ... \n it should be like this: ./fsmp_prover config_path witness_path \n")
		os.Exit(3)
	}
	fmt.Println(args[1])
	fsmp_keys_procedure(args[1], args[2])
}

