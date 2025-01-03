package main

import (
	"fmt"
	"os"
	"crypto/sha256"
	"github.com/Nik-U/pbc"
	"github.com/BurntSushi/toml"
)

func get_pedersen(witness_name string, crs_name string, pair_name string, commit_path string, rcom_path string) {
	var witness Witness 
	if _, err := toml.DecodeFile(witness_name, &witness); err != nil {
		panic(err)
	} 
	data , err := os.ReadFile(pair_name)
	if err != nil {panic(err)}
	pairing , err := pbc.NewPairingFromString(string(data))
	if err != nil {panic(err)}
	content := read_from_file_to_string(crs_name)
	crs := string_to_struct_crs(content, pairing)

	r_com := pairing.NewZr().Rand()
	temp1 := pairing.NewG1()
	temp1.PowZn(crs.Random_h, r_com)
	com := pairing.NewG1()
	ss := fmt.Sprintf("%x", sha256.Sum256([]byte(witness.W )))
	ss_, _ := pairing.NewZr().SetString(ss, 16)
	com.PowZn(crs.Random_g, ss_)

	com.Mul(com, temp1)

	write_to_string_to_file(commit_path , PedersenCom{com} )

	file, err := os.Create(rcom_path)
	if (err!= nil) {fmt.Println(err)}
	_, err = file.WriteString(r_com.String())
	if (err != nil) {fmt.Println(err)}
	file.Close()
}

func main() {
	args := os.Args
	fmt.Println(len(args))
	if len(args) != 6 {
		fmt.Println("panic!!")
		os.Exit(3)
	}
	get_pedersen(args[1], args[2], args[3], args[4], args[5])
}
