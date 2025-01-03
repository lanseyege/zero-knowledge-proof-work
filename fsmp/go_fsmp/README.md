
# fSMP

## compile the keys code
go build -o fSMP fSMP.go pkDIPE.go SME.go utils.go get\_structs.go

## run keys code
./fSMP ./config/params.toml ./config/witness.toml

# fSMP\_prover
go build -o fSMP\_prover fSMP\_prover.go pkDIPE.go SME.go utils.go get\_structs.go

## run fsmp\_prover
./fSMP\_prover ./config/params.toml ./config/witness.toml

# fSMP\_verifier
go build -o fSMP\_verifier fSMP\_verifier.go pkDIPE.go SME.go utils.go get\_structs.go

## run fsmp\_verifier
./fSMP\_verifier ./config/params.toml
