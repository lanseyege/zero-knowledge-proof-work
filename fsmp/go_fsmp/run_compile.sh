
go build -o fSMP fSMP.go pkDIPE.go SME.go utils.go get_structs.go

go build -o get_commitment get_commitment.go utils.go get_structs.go

go build -o fSMP_prover fSMP_prover.go pkDIPE.go SME.go utils.go get_structs.go

go build -o fSMP_verifier fSMP_verifier.go pkDIPE.go SME.go utils.go get_structs.go
