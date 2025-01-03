package main

import (
	"github.com/Nik-U/pbc"
)

type CRS struct {
	Random_g *pbc.Element
	Random_h *pbc.Element
}

type Config struct {
	L1 int
	L2 int
	K int
	Bit_length int
	Pair_file string
	Phi []string
	Keys_path string
	Data_path string
	S []int 
	Seed int64
}

type Witness struct {
	W string
}

type PedersenCom struct {
	Com *pbc.Element
}

type Tau struct {
	Tau_f []*pbc.Element
	Tau_p []*pbc.Element
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
	K_bond []*pbc.Element // k+1
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
	pairing *pbc.Pairing
	//group Group
	//pk PK
	//msk MSK
}

type Cy struct {
	C0 []*pbc.Element
	C1 [][]*pbc.Element
	C2 [][]*pbc.Element
	C *pbc.Element
}

type Group struct {
	pairing *pbc.Pairing
	g1 *pbc.Element
	g2 *pbc.Element
	gt *pbc.Element
}

type PI struct {
	Tau_p []*pbc.Element
	C_bond_i []*pbc.Element
	Z_com_i []*pbc.Element
	Z_bond_t [][]*pbc.Element
}

type SubSets struct {
	Subsets [][]int
}
