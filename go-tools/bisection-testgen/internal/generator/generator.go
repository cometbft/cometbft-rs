package generator

import (
	"fmt"
	"time"

	"github.com/cometbft/cometbft/light"
	st "github.com/cometbft/cometbft/state"

	"github.com/cometbft/cometbft/types"
)

func generateNextBlocks(
	numOfBlocks int,
	state st.State,
	privVals types.PrivValidatorsByAddress,
	lastCommit *types.Commit,
	valSetChanges ValSetChanges,
	blockTime time.Time,
) ([]lightBlock, []st.State, types.PrivValidatorsByAddress) {
	var lightBlocks []lightBlock
	var states []st.State

	valSetChanges = append(valSetChanges, valSetChanges[len(valSetChanges)-1])
	for i := 0; i < numOfBlocks; i++ {
		lightblock, st, _ := generateNextBlockWithNextValsUpdate(
			state,
			valSetChanges[i].PrivVals,
			lastCommit,
			valSetChanges[i+1].Validators,
			nil,
			blockTime,
		)
		lightBlocks = append(lightBlocks, lightblock)
		state = st
		lastCommit = lightblock.SignedHeader.Commit
		states = append(states, state)
		blockTime = blockTime.Add(5 * time.Second)
	}

	return lightBlocks, states, privVals
}

func makeLightBlocks(
	valSetChanges ValSetChanges,
) ([]lightBlock, []st.State, types.PrivValidatorsByAddress) {
	signedHeader, state, _ := generateFirstBlockWithNextValsUpdate(
		valSetChanges[0].Validators,
		valSetChanges[0].PrivVals,
		valSetChanges[1].Validators,
		nil,
		firstBlockTime,
	)

	firstBlock := []lightBlock{
		{
			SignedHeader:     signedHeader,
			ValidatorSet:     state.LastValidators,
			NextValidatorSet: state.Validators,
		},
	}
	lastCommit := signedHeader.Commit
	numOfBlocks := len(valSetChanges) - 1
	lightBlocks, states, privVals := generateNextBlocks(
		numOfBlocks,
		state,
		valSetChanges[1].PrivVals,
		lastCommit,
		valSetChanges[1:],
		thirdBlockTime,
	)
	lightBlocks = append(firstBlock, lightBlocks...)
	stateSlice := []st.State{
		state,
	}
	states = append(stateSlice, states...)

	return lightBlocks, states, privVals
}

func generateMultiPeerBisectionCase(
	description string,
	primaryValSetChanges ValSetChanges,
	alternativeValSetChanges ValSetChanges,
	expectedBisections int32,
	expectOutput string,
) (TestBisection, []st.State, types.PrivValidatorsByAddress, []st.State, types.PrivValidatorsByAddress) {
	testBisection, statesPrimary, privValsPrimary := generateGeneralBisectionCase(
		description,
		primaryValSetChanges,
		expectedBisections)

	lightBlocks, statesAlternative, privValsAlternative := makeLightBlocks(alternativeValSetChanges)
	chainID := lightBlocks[1].SignedHeader.Header.ChainID
	testBisection.Witnesses[0] = &MockProvider{ChainIDStr: chainID, LightBlocks: lightBlocks}
	testBisection.ExpectedOutput = expectOutput

	return testBisection, statesPrimary, privValsPrimary, statesAlternative, privValsAlternative
}

func generateGeneralBisectionCase(
	description string,
	valSetChanges ValSetChanges,
	expectedBisections int32,
) (TestBisection, []st.State, types.PrivValidatorsByAddress) {

	lightBlocks, states, privVals := makeLightBlocks(valSetChanges)
	chainID := lightBlocks[1].SignedHeader.Header.ChainID
	primary := &MockProvider{ChainIDStr: chainID, LightBlocks: lightBlocks}

	var witnesses []*MockProvider
	witnesses = append(witnesses, primary)

	trustOptions := TrustOptions{}.make(*lightBlocks[1].SignedHeader, TRUSTING_PERIOD, light.DefaultTrustLevel)
	heightToVerify := int64(len(valSetChanges))

	testBisection := TestBisection{}.make(
		description,
		trustOptions,
		primary,
		witnesses,
		heightToVerify,
		now,
		expectedOutputNoError,
		expectedBisections,
	)

	return testBisection, states, privVals
}

// generateFirstBlock creates the first block of the chain
// with the given list of validators and timestamp
// Thus, It also calls the NewState() to initialize the state
// Returns the signedHeader and state after the first block is created
func generateFirstBlock(
	vals []*types.Validator,
	privVals types.PrivValidatorsByAddress,
	now time.Time,
) (*types.SignedHeader, st.State, types.PrivValidatorsByAddress) {

	valSet := types.NewValidatorSet(vals)
	state := NewState("test-chain-01", valSet, valSet)

	return makeBlock(state, privVals, nil, now)
}

// TODO: Comment!
func generateFirstBlockWithNextValsUpdate(
	vals []*types.Validator,
	privVals types.PrivValidatorsByAddress,
	nextVals []*types.Validator,
	nextPrivVals types.PrivValidatorsByAddress,
	now time.Time,
) (*types.SignedHeader, st.State, types.PrivValidatorsByAddress) {

	valSet := types.NewValidatorSet(vals)
	nextValSet := types.NewValidatorSet(nextVals)
	state := NewState("test-chain-01", valSet, nextValSet)

	return makeBlock(state, privVals, nextPrivVals, now)
}

func makeBlock(
	state st.State,
	privVals types.PrivValidatorsByAddress,
	nextPrivVals types.PrivValidatorsByAddress,
	now time.Time,
) (*types.SignedHeader, st.State, types.PrivValidatorsByAddress) {
	txs := generateTxs()
	evidences := generateEvidences()
	lbh := state.LastBlockHeight + 1
	proposer := state.Validators.Proposer.Address

	// first block has a nil last commit
	block := state.MakeBlock(lbh, txs, &types.Commit{}, evidences, proposer)
	partSet, err := block.MakePartSet(types.BlockPartSizeBytes)
	if err != nil {
		panic(fmt.Sprintf("failed to make part set: %v", err))
	}

	commit := generateCommit(block.Header, partSet, state.Validators, privVals, state.ChainID, now)

	state, privVals = updateState(state, commit.BlockID, privVals, nextPrivVals)

	signedHeader := &types.SignedHeader{
		Header: &block.Header,
		Commit: commit,
	}

	return signedHeader, state, privVals
}

// Builds the Initial struct with given parameters
func generateInitial(signedHeader *types.SignedHeader, nextValidatorSet *types.ValidatorSet, trustingPeriod time.Duration, now time.Time) Initial {
	return Initial{
		SignedHeader:     signedHeader,
		NextValidatorSet: nextValidatorSet,
		TrustingPeriod:   trustingPeriod,
		Now:              now,
	}
}

// This one generates a "next" block,
// i.e. given the first block, this function can be used to build up successive blocks
func generateNextBlock(state st.State, privVals types.PrivValidatorsByAddress, lastCommit *types.Commit, now time.Time) (lightBlock, st.State, types.PrivValidatorsByAddress) {
	txs := generateTxs()
	evidences := generateEvidences()
	lbh := state.LastBlockHeight + 1
	proposer := state.Validators.Proposer.Address

	block := state.MakeBlock(lbh, txs, lastCommit, evidences, proposer)
	partSet, err := block.MakePartSet(types.BlockPartSizeBytes)
	if err != nil {
		panic(fmt.Sprintf("failed to make part set: %v", err))
	}

	commit := generateCommit(block.Header, partSet, state.Validators, privVals, state.ChainID, now)
	lightBlock := lightBlock{
		SignedHeader: &types.SignedHeader{
			Header: &block.Header,
			Commit: commit,
		},
		ValidatorSet:     state.Validators.Copy(),     // dereferencing pointer
		NextValidatorSet: state.NextValidators.Copy(), // dereferencing pointer
	}

	state, _ = updateState(state, commit.BlockID, privVals, nil)
	return lightBlock, state, privVals

}

// Similar to generateNextBlock
// It also takes in new validators and privVals to be added to the NextValidatorSet
// Calls the UpdateWithChangeSet function on state.NextValidatorSet for the same
// Also, you can specify the number of vals to be deleted from it
func generateNextBlockWithNextValsUpdate(
	state st.State,
	privVals types.PrivValidatorsByAddress,
	lastCommit *types.Commit,
	newVals []*types.Validator,
	newPrivVals types.PrivValidatorsByAddress,
	now time.Time,
) (lightBlock, st.State, types.PrivValidatorsByAddress) {

	state.NextValidators = types.NewValidatorSet(newVals)

	// state.NextValidators.IncrementProposerPriority(1)

	txs := generateTxs()
	evidences := generateEvidences()
	lbh := state.LastBlockHeight + 1
	proposer := state.Validators.Proposer.Address

	block := state.MakeBlock(lbh, txs, lastCommit, evidences, proposer)
	partSet, err := block.MakePartSet(types.BlockPartSizeBytes)
	if err != nil {
		panic(fmt.Sprintf("failed to make part set: %v", err))
	}

	commit := generateCommit(block.Header, partSet, state.Validators, privVals, state.ChainID, now)

	lightBlock := lightBlock{
		SignedHeader: &types.SignedHeader{
			Header: &block.Header,
			Commit: commit,
		},
		ValidatorSet:     state.Validators.Copy(),     // dereferencing pointer
		NextValidatorSet: state.NextValidators.Copy(), // dereferencing pointer
	}
	state, newPrivVals = updateState(state, commit.BlockID, privVals, newPrivVals)

	return lightBlock, state, newPrivVals
}

// Builds a general case containing initial and one light block in input
// TODO: change name to genInitialAndInput
func generateGeneralCase(
	vals []*types.Validator,
	privVals types.PrivValidatorsByAddress,
) (Initial, []lightBlock, st.State, types.PrivValidatorsByAddress) {

	var input []lightBlock

	signedHeader, state, privVals := generateFirstBlock(vals, privVals, firstBlockTime)
	initial := generateInitial(signedHeader, state.NextValidators, TRUSTING_PERIOD, now)
	lightBlock, state, _ := generateNextBlock(state, privVals, signedHeader.Commit, secondBlockTime)
	input = append(input, lightBlock)

	return initial, input, state, privVals
}

func generateInitialAndInputSkipBlocks(
	vals []*types.Validator,
	privVals types.PrivValidatorsByAddress,
	numOfBlocksToSkip int,
) (Initial, []lightBlock, st.State, types.PrivValidatorsByAddress) {
	var input []lightBlock

	signedHeader, state, privVals := generateFirstBlock(
		vals,
		privVals,
		firstBlockTime,
	)
	initial := generateInitial(signedHeader, state.NextValidators, TRUSTING_PERIOD, now)

	blockTime := secondBlockTime
	for i := 0; i <= numOfBlocksToSkip; i++ {
		lightBlock, s, _ := generateNextBlock(state, privVals, signedHeader.Commit, blockTime)
		blockTime = blockTime.Add(5 * time.Second)
		state = s

		if i == numOfBlocksToSkip {
			input = append(input, lightBlock)
		}
	}

	return initial, input, state, privVals
}

func generateAndMakeGeneralTestCase(description string, vals []*types.Validator, privVals types.PrivValidatorsByAddress, expectedOutput string) TestCase {
	initial, input, _, _ := generateGeneralCase(vals, privVals)
	return makeTestCase(description, initial, input, expectedOutput)
}

func generateAndMakeNextValsUpdateTestCase(
	description string,
	initialVals []*types.Validator,
	initialPrivVals types.PrivValidatorsByAddress,
	nextVals []*types.Validator,
	nextPrivVals types.PrivValidatorsByAddress,
	expectedOutput string,
) TestCase {

	initial, input, _, _ := generateNextValsUpdateCase(initialVals, initialPrivVals, nextVals, nextPrivVals)
	return makeTestCase(description, initial, input, expectedOutput)
}

// Builds a case where next validator set changes
// makes initial, and input with 2 light blocks
func generateNextValsUpdateCase(
	initialVals []*types.Validator,
	initialPrivVals types.PrivValidatorsByAddress,
	nextVals []*types.Validator,
	nextPrivVals types.PrivValidatorsByAddress,
) (Initial, []lightBlock, st.State, types.PrivValidatorsByAddress) {

	var input []lightBlock

	signedHeader, state, privVals := generateFirstBlock(initialVals, initialPrivVals, firstBlockTime)
	initial := generateInitial(signedHeader, state.NextValidators, TRUSTING_PERIOD, now)

	lightBlock, state, privVals := generateNextBlockWithNextValsUpdate(state, privVals, signedHeader.Commit, nextVals, nextPrivVals, secondBlockTime)
	input = append(input, lightBlock)
	lightBlock, state, _ = generateNextBlock(state, privVals, lightBlock.SignedHeader.Commit, thirdBlockTime)
	input = append(input, lightBlock)

	return initial, input, state, privVals
}

// UPDATE -> mutex on PartSet and functions take pointer to valSet - have to use a pointer
// generateCommit takes in header, partSet from Block that was created,
// validator set, privVals, chain ID and a timestamp to create
// and return a commit type
// To be called after MakeBlock()
func generateCommit(
	header types.Header,
	partSet *types.PartSet,
	valSet *types.ValidatorSet,
	privVals []types.PrivValidator,
	chainID string,
	now time.Time,
) *types.Commit {
	blockID := types.BlockID{
		Hash: header.Hash(),
		PartSetHeader: types.PartSetHeader{
			Hash:  partSet.Hash(),
			Total: partSet.Total(),
		},
	}

	commit, err := makeCommit(blockID, header.Height, 1, valSet, privVals, chainID, now)
	if err != nil {
		fmt.Println(err)
	}

	return commit
}

// copied from https://github.com/cometbft/cometbft/blob/809582f4f269090e51c30bd4e586a91eafea8522/internal/test/commit.go#L41-L84
func makeCommit(blockID types.BlockID, height int64, round int32, valSet *types.ValidatorSet, privVals []types.PrivValidator, chainID string, now time.Time) (*types.Commit, error) {
	sigs := make([]types.CommitSig, len(valSet.Validators))
	for i := 0; i < len(valSet.Validators); i++ {
		sigs[i] = types.NewCommitSigAbsent()
	}

	for _, privVal := range privVals {
		pk, err := privVal.GetPubKey()
		if err != nil {
			return nil, err
		}
		addr := pk.Address()

		idx, _ := valSet.GetByAddressMut(addr)
		if idx < 0 {
			return nil, fmt.Errorf("validator with address %s not in validator set", addr)
		}

		vote := &types.Vote{
			ValidatorAddress: addr,
			ValidatorIndex:   idx,
			Height:           height,
			Round:            round,
			Type:             types.PrecommitType,
			BlockID:          blockID,
			Timestamp:        now,
		}

		v := vote.ToProto()

		if err := privVal.SignVote(chainID, v, false); err != nil {
			return nil, err
		}

		sigs[idx] = types.CommitSig{
			BlockIDFlag:      types.BlockIDFlagCommit,
			ValidatorAddress: addr,
			Timestamp:        now,
			Signature:        v.Signature,
		}
	}

	return &types.Commit{Height: height, Round: round, BlockID: blockID, Signatures: sigs}, nil
}
