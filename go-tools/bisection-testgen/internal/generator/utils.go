package generator

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"sort"

	st "github.com/cometbft/cometbft/state"
	"github.com/cometbft/cometbft/types"
)

// NewState is used to initiate a state that will be used and manipulated
// by functions to create blocks for the "simulated" blockchain
// It creates an INITIAL state with the given parameters
func NewState(chainID string, valSet *types.ValidatorSet, nextValSet *types.ValidatorSet) st.State {
	consensusParams := types.DefaultConsensusParams()

	return st.State{
		Version: st.InitStateVersion,

		ChainID:       chainID,
		InitialHeight: 1,

		LastBlockHeight: 0,
		LastBlockID:     types.BlockID{},
		LastBlockTime:   genTime,

		NextValidators:              nextValSet,
		Validators:                  valSet,
		LastValidators:              types.NewValidatorSet(nil),
		LastHeightValidatorsChanged: 1,

		ConsensusParams:                  *consensusParams,
		LastHeightConsensusParamsChanged: 1,

		AppHash: []byte("app_hash"),
	}
}

// Called after creating each block to update the validator set, proposer,
// last block id, privVals etc.
// In case of privVals, it adds the new ones to the list
// and performs a sort operation on it.
func updateState(
	state st.State,
	blockID types.BlockID,
	privVals types.PrivValidatorsByAddress,
	newPrivVals types.PrivValidatorsByAddress,
) (st.State, types.PrivValidatorsByAddress) {
	state.LastBlockHeight++
	state.LastValidators = state.Validators.Copy()
	state.Validators = state.NextValidators.Copy()
	state.Validators.IncrementProposerPriority(1)
	state.LastBlockID = blockID

	// Adds newPrivVals if they are not already present in privVals
	if newPrivVals != nil {
		for _, npv := range newPrivVals {
			if !contains(privVals, npv) {
				privVals = append(privVals, npv)
			}
		}
	}

	// Checks if a validator has been removed from the set
	// If so, removes it from privVals too
	if len(privVals) > len(state.Validators.Validators) {
		for i := 0; i < len(privVals); i++ {
			pubKey, err := privVals[i].GetPubKey()
			if err != nil {
				fmt.Println(err)
			}
			_, val := state.Validators.GetByAddress(pubKey.Address())
			if val == nil {
				// removing the privVal when no corresponding entry found in the validator set
				privVals = append(privVals[:i], privVals[i+1:]...)
				i = i - 1
			}
		}
	}

	sort.Sort(privVals)

	return state, privVals
}

// Checks if privVals contain the privVal - used by updateState()
func contains(privVals types.PrivValidatorsByAddress, npv types.PrivValidator) bool {
	pk, err := npv.GetPubKey()
	if err != nil {
		panic(fmt.Sprintf("error getting public key: %v", err))
	}

	for _, n := range privVals {
		pk2, err := n.GetPubKey()
		if err != nil {
			panic(fmt.Sprintf("error getting public key: %v", err))
		}
		if bytes.Equal(pk.Address(), pk2.Address()) {
			return true
		}
	}
	return false
}

// ReadFile returns the byte slice of the content in the given file
// "file" parameter is the path to the file to be read
func ReadFile(file string) []byte {
	jsonFile, err := os.Open(file)
	if err != nil {
		panic(err)
	}
	defer jsonFile.Close()

	dat, err := io.ReadAll(jsonFile)
	if err != nil {
		panic(err)
	}

	return dat
}
