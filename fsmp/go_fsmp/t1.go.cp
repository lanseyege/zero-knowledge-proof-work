package main

import (
	"fmt"
	//"go_fsmp/pkDIPE"
	"github.com/Nik-U/pbc"
)

func main() {
	fmt.Println("vim-go")

	l1, l2, k := 2, 3, 3
	filename := "./config/f.param"
	pkdipe := get_new_pkdipe(l1, l2, k, filename)
	fmt.Printf("g1 = %s\n", pkdipe.group.g1)
	pkdipe.Setup()
	x1 := make([]*pbc.Element, l1)
	x2 := make([]*pbc.Element, l2)
	for i := 0; i < l1 ; i ++ {
		if i % 2 == 0 { 
			x1[i] = pkdipe.group.pairing.NewZr().Rand()
			//x1[i].Set0()
		} else {
			x1[i] = pkdipe.group.pairing.NewZr().Rand()
			//x1[i].Set1()
		}
	}
	for i := 0; i < l2 ; i ++ {
		if i % 2 == 0 { 
			x2[i] = pkdipe.group.pairing.NewZr().Rand() 
			//x2[i].Set1()
		} else {
			x2[i] = pkdipe.group.pairing.NewZr().Rand() 
			//x2[i].Set0()
		}
	}
	skx := pkdipe.KGen(x1, x2)
	fmt.Println("K0 and K1")
	fmt.Println(skx.K0)
	fmt.Println(skx.K1)
	flag := pkdipe.CheckKey(x1, x2, skx)
	fmt.Printf("flag = %t\n", flag)
	y1 := make([]*pbc.Element, l1)
	y2 := make([]*pbc.Element, l2)
	for i := 0; i < l1 ; i ++ {
		if i % 2 == 0 { 
			y1[i] = pkdipe.group.pairing.NewZr().Rand() 
			//y1[i].Set0()
		} else {
			y1[i] = pkdipe.group.pairing.NewZr().Rand() 
			//y1[i].Set1()
		}
	}
	for i := 0; i < l2 ; i ++ {
		if i % 2 == 0 { 
			y2[i] = pkdipe.group.pairing.NewZr().Rand() 
			//y2[i].Set1()
		} else {
			y2[i] = pkdipe.group.pairing.NewZr().Rand() 
			//y2[i].Set0()
		}
	}
	msg := pkdipe.group.pairing.NewGT().Rand()
	cy := pkdipe.Enc(y1, y2, msg)
	msg_ := pkdipe.Dec(cy, skx, x1, x2)
	fmt.Printf("msg = %s\n", msg)
	fmt.Printf("msg_ = %s\n", msg_)
}
