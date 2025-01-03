package main

import (
	"fmt"
	//"github.com/pelletier/go-toml"
	"github.com/BurntSushi/toml"
)

type Config struct {
	La int
	Lb int
	K int
	Length int
	Pair_file string
	Phi []string
	S []int
	Keys string
	Data string
}
func main() {

	fmt.Println("vim-go")
	config_name := "./config/pa.toml"
	var conf Config
	if _ , err := toml.DecodeFile(config_name, &conf); err != nil {
		panic(err)
	}
	//conf.La = len(conf.Phi)
	fmt.Println(conf)
	fmt.Println(conf.La)
	fmt.Println(conf.Lb)
	fmt.Println(conf.K)
	fmt.Println(conf.Length)
	fmt.Println(conf.Pair_file)


}
