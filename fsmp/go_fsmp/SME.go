package main

import (
	"fmt"
	"os"
	"github.com/Nik-U/pbc"
)

type SME struct {
	dipe pkDIPE
}

func get_new_sme(l1 int, l2 int, k int, file_name string) SME {
	dipe := get_new_pkdipe(l1, l2, k, file_name)
	return SME{dipe}
}

func (sme SME) Setup() (Group, PK, MSK) {
	return sme.dipe.Setup()
}

func (sme SME) KGen(pk PK, sk MSK, S []int, tau_f []*pbc.Element, g2 *pbc.Element) SKx {
	l1 := sme.dipe.l1
	IS := make([]*pbc.Element, l1)
	for i := 0; i < l1; i ++ {
		IS[i] = sme.dipe.pairing.NewZr()
	}
	EncodeS(IS, l1, S)
	return sme.dipe.KGen(sk, IS, tau_f, g2)
}

func (sme SME) CheckKey(pk PK, S []int, tau_f []*pbc.Element, skx SKx) bool {
	l1 := sme.dipe.l1
	IS := make([]*pbc.Element, l1)
	for i := 0; i < l1; i ++ {
		IS[i] = sme.dipe.pairing.NewZr()
	}
	EncodeS(IS, l1, S)
	return sme.dipe.CheckKey(pk, IS, tau_f, skx)
}

func (sme SME) Enc(pk PK, Phi []string, tau_p []*pbc.Element, w string, msg *pbc.Element) (Cy, []*pbc.Element) {
	i2 := true
	for _, v := range Phi {
		if v == w {
			i2 = false
			break
		}
	}
	if i2 {
		os.Exit(3)
	}
	Iw := make([]*pbc.Element, sme.dipe.l1)
	for i := 0; i < sme.dipe.l1; i ++ {
		Iw[i] = sme.dipe.pairing.NewZr()
	}
	EncodeW(Iw, sme.dipe.l1, w, Phi)
	
	return sme.dipe.Enc(pk, Iw, tau_p, msg)
}

func (sme SME) Query(cy Cy, skx SKx, S []int, tau_f []*pbc.Element, msg *pbc.Element) bool {
	l1 := sme.dipe.l1
	IS := make([]*pbc.Element, l1)
	for i := 0; i < l1; i ++ {
		IS[i] = sme.dipe.pairing.NewZr()
	}
	EncodeS(IS, l1, S)
	msg_ := sme.dipe.Dec(cy, skx, IS, tau_f)
	fmt.Println("msg_")
	fmt.Println(msg_)

	return msg.Equals(msg_)
}

func SME_test() {
	fmt.Println("SME.go ... ")
}

