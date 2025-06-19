package generator

import (
	"time"

	"github.com/cometbft/cometbft/types"
)

const MULTI_PEER_BISECTION_PATH = "./tests/json/bisection/multi_peer/"

func caseBisectionConflictingValidCommitsFromTheOnlyWitness(valList ValList) {
	description := "Case: Trusted height=1, found conflicting valid commit at height=11 from the only witness available, should expect error"
	primaryValSetChanges := ValSetChanges{}.getDefault(valList.Copy())
	alternativeValSetChanges := ValSetChanges{}.getDefault(valList.Copy())
	last := len(alternativeValSetChanges) - 1
	alternativeValSetChanges[last].Validators = valList.Validators[6:8]
	alternativeValSetChanges[last].PrivVals = valList.PrivVals[6:8]
	testBisection, _, _, _, _ := generateMultiPeerBisectionCase(
		description,
		primaryValSetChanges,
		alternativeValSetChanges,
		2,
		expectedOutputError,
	)

	file := MULTI_PEER_BISECTION_PATH + "conflicting_valid_commits_from_the_only_witness.json"
	testBisection.genJSON(file)
}

func caseBisectionConflictingValidCommitsFromOneOfTheWitnesses(valList ValList) {
	description := "Case: Trusted height=1, found conflicting valid commit at height=11 from only one of the two witnesses, should expect error"
	primaryValSetChanges := ValSetChanges{}.getDefault(valList.Copy())
	alternativeValSetChanges := ValSetChanges{}.getDefault(valList.Copy())
	last := len(alternativeValSetChanges) - 1
	alternativeValSetChanges[last].Validators = valList.Validators[6:8]
	alternativeValSetChanges[last].PrivVals = valList.PrivVals[6:8]
	testBisection, _, _, _, _ := generateMultiPeerBisectionCase(
		description,
		primaryValSetChanges,
		alternativeValSetChanges,
		2,
		expectedOutputError,
	)

	testBisection.Witnesses = append(testBisection.Witnesses, testBisection.Primary)

	file := MULTI_PEER_BISECTION_PATH + "conflicting_valid_commits_from_one_of_the_witnesses.json"
	testBisection.genJSON(file)
}

// Also a case where some validators have double signed
func caseBisectionConflictingHeaders(valList ValList) {
	description := "Case: Trusted height=1, bisecting to verify height=5 and receives a conflicting header from witness, should expect error"
	copiedValList := valList.Copy()
	valSetChanges := ValSetChanges{}.getDefault(copiedValList)[:4]
	lastValSetChange := ValList{
		copiedValList.Validators[:4],
		copiedValList.PrivVals[:4],
	}
	valSetChanges = append(valSetChanges, lastValSetChange)

	testBisection, _, _ := generateGeneralBisectionCase(
		description,
		valSetChanges,
		2,
	)
	last := len(testBisection.Primary.LightBlocks) - 1
	testBisection.Primary.LightBlocks[last].SignedHeader.Commit.Signatures[0] = types.CommitSig{
		BlockIDFlag:      types.BlockIDFlagAbsent,
		ValidatorAddress: nil,
	}

	testBisection2, states, _ := generateGeneralBisectionCase(
		description,
		valSetChanges,
		2,
	)

	state := states[len(states)-2]
	state.Validators.IncrementProposerPriority(1)
	lastCommit := testBisection2.Primary.LightBlocks[last-1].SignedHeader.Commit
	time := testBisection2.Primary.LightBlocks[last-1].SignedHeader.Header.Time.Add(2 * time.Second)

	liteBlock, _, _ := generateNextBlock(state, lastValSetChange.PrivVals, lastCommit, time)
	liteBlock.SignedHeader.Commit.Signatures[1] = types.CommitSig{
		BlockIDFlag:      types.BlockIDFlagAbsent,
		ValidatorAddress: nil,
	}
	testBisection2.Primary.LightBlocks[last].SignedHeader = liteBlock.SignedHeader
	testBisection.Witnesses[0] = testBisection2.Primary

	testBisection.HeightToVerify = 5
	testBisection.ExpectedOutput = expectedOutputError

	file := MULTI_PEER_BISECTION_PATH + "conflicting_headers.json"
	testBisection.genJSON(file)
}

func caseBisectionMaliciousValidatorSet(valList ValList) {
	description := "Case: Trusted height = 1, due to malicious validator set it fails to find a witness for it"

	valSetChanges := ValSetChanges{}.getDefault(valList.Copy())
	testBisection, states, privVals, _, _ := generateMultiPeerBisectionCase(
		description,
		valSetChanges,
		valSetChanges,
		2,
		expectedOutputError,
	)

	lastIndex := len(testBisection.Primary.LightBlocks) - 2
	state := states[lastIndex]
	valSet := types.NewValidatorSet(valList.Validators[:2])
	privVals = valList.PrivVals[:2]
	state.Validators = valSet

	lastCommit := testBisection.Primary.LightBlocks[lastIndex].SignedHeader.Commit
	time := state.LastBlockTime.Add(5 * time.Second)

	liteBlock, _, _ := generateNextBlock(state, privVals, lastCommit, time)

	testBisection.Primary.LightBlocks[lastIndex+1].SignedHeader = liteBlock.SignedHeader

	file := MULTI_PEER_BISECTION_PATH + "malicious_validator_set.json"
	testBisection.genJSON(file)
}
